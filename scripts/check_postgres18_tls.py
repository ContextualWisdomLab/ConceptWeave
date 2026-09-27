import os
import secrets
import socket
import subprocess
import tempfile
from pathlib import Path

# Run with PostgreSQL 18 and OpenSSL on PATH; clusters and credentials are ephemeral.
pg = Path(subprocess.check_output(['pg_config', '--bindir'], text=True).strip())
version = subprocess.check_output([pg / 'pg_ctl', '--version'], text=True)
if not version.startswith('pg_ctl (PostgreSQL) 18.'):
    raise SystemExit('PostgreSQL 18 is required')
openssl = 'openssl'
started = []
repository = Path(__file__).resolve().parent.parent

def run(*args):
    with setup_log.open('ab') as log:
        subprocess.run(list(map(str, args)), check=True, stdout=log, stderr=log)

def free_ports():
    sockets = [socket.socket(), socket.socket()]
    try:
        for item in sockets:
            item.bind(('127.0.0.1', 0))
        return [item.getsockname()[1] for item in sockets]
    finally:
        for item in sockets:
            item.close()

with tempfile.TemporaryDirectory(prefix='conceptweave-pg18-tls-', dir='/tmp') as directory:
    root = Path(directory)
    setup_log = root / 'setup.log'
    password = secrets.token_hex(24)
    tls_port, plain_port = free_ports()
    try:
        for name in ('ca', 'wrong-ca'):
            run(openssl, 'req', '-x509', '-newkey', 'rsa:2048', '-noenc', '-sha256',
                '-days', '1', '-subj', f'/CN=ConceptWeave {name}',
                '-addext', 'basicConstraints=critical,CA:TRUE',
                '-addext', 'keyUsage=critical,keyCertSign,cRLSign',
                '-keyout', root / f'{name}.key', '-out', root / f'{name}.crt')
            run(openssl, 'x509', '-in', root / f'{name}.crt', '-outform', 'DER',
                '-out', root / f'{name}.der')
        run(openssl, 'req', '-new', '-newkey', 'rsa:2048', '-noenc', '-sha256',
            '-subj', '/CN=localhost', '-keyout', root / 'server.key', '-out', root / 'server.csr')
        (root / 'server.ext').write_text('basicConstraints=critical,CA:FALSE\n'
            'keyUsage=critical,digitalSignature,keyEncipherment\n'
            'extendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost\n')
        run(openssl, 'x509', '-req', '-in', root / 'server.csr', '-CA', root / 'ca.crt',
            '-CAkey', root / 'ca.key', '-CAcreateserial', '-sha256', '-days', '1',
            '-extfile', root / 'server.ext', '-out', root / 'server.crt')
        (root / 'server.key').chmod(0o600)
        sql = root / 'fixture.sql'
        sql.write_text(f"CREATE ROLE cw_source_observer LOGIN PASSWORD '{password}';\n"
            'REVOKE TEMP ON DATABASE postgres FROM PUBLIC;\n'
            'CREATE SCHEMA conceptweave_tls_fixture AUTHORIZATION cw_source_observer;\n'
            'GRANT CONNECT ON DATABASE postgres TO cw_source_observer;\n'
            'GRANT USAGE ON SCHEMA public TO cw_source_observer;\n')
        sql.chmod(0o600)
        for name, port in [('tls', tls_port), ('plain', plain_port)]:
            data = root / name
            sock = root / f'{name}-socket'
            sock.mkdir(mode=0o700)
            run(pg / 'initdb', '-D', data, '-U', 'cw_test_admin', '--encoding=UTF8',
                '--locale=C', '--auth-local=trust', '--auth-host=scram-sha-256', '--no-instructions')
            with (data / 'postgresql.conf').open('a') as config:
                config.write(f"\nlisten_addresses='localhost'\nport={port}\n"
                    f"unix_socket_directories='{sock}'\n")
                if name == 'tls':
                    config.write(f"ssl=on\nssl_cert_file='{root / 'server.crt'}'\n"
                        f"ssl_key_file='{root / 'server.key'}'\n")
            started.append(data)
            run(pg / 'pg_ctl', '-D', data, '-l', root / f'{name}-server.log', '-w', '-t', '10', 'start')
            run(pg / 'psql', '-h', sock, '-p', port, '-U', 'cw_test_admin', '-d', 'postgres',
                '-v', 'ON_ERROR_STOP=1', '-f', sql)
        environment = os.environ.copy()
        environment.update({
            'CONCEPTWEAVE_PG18_TLS_TEST_DSN': f'host=localhost port={tls_port} user=cw_source_observer dbname=postgres password={password}',
            'CONCEPTWEAVE_PG18_PLAINTEXT_TEST_DSN': f'host=localhost port={plain_port} user=cw_source_observer dbname=postgres password={password}',
            'CONCEPTWEAVE_PG18_TLS_CA_DER': str(root / 'ca.der'),
            'CONCEPTWEAVE_PG18_TLS_WRONG_CA_DER': str(root / 'wrong-ca.der'),
        })
        with (root / 'test.log').open('wb') as log:
            subprocess.run(['cargo', '+1.98.0', 'test', '-p', 'conceptweave-postgres-adapter',
                '--test', 'postgres18_capture', 'postgres18_tcp_requires_valid_ca_and_host_name',
                '--locked', '--', '--exact', '--test-threads=1'], check=True, env=environment,
                cwd=repository,
                stdout=log, stderr=log)
        print('owned PostgreSQL 18 TLS runtime conformance passed')
    except subprocess.CalledProcessError:
        # Cargo output contains no fixture credentials; setup output stays private.
        test_log = root / 'test.log'
        if test_log.exists():
            print(test_log.read_text())
        raise SystemExit('PostgreSQL 18 TLS conformance failed')
    finally:
        stop_failed = False
        for data in reversed(started):
            try:
                run(pg / 'pg_ctl', '-D', data, '-w', '-t', '10', '-m', 'fast', 'stop')
            except subprocess.CalledProcessError:
                stop_failed = stop_failed or (data / 'postmaster.pid').exists()
        if stop_failed:
            retained = root.with_name(root.name + '-stop-failed')
            root.rename(retained)
            raise SystemExit(f'Owned server did not stop; private fixture retained at {retained}')
