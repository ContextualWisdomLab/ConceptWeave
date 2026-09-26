# Product CI lockfile consistency

The Product workflow's `cargo generate-lockfile --locked` failed on head
`53f5cb68f12fd7d7a532e4c8fb97ded4d8510196`: Cargo tried to update
`Cargo.lock`, then refused because `--locked` forbids that update. No manifest or
lockfile had changed. This command tests whether regeneration would produce the
same lockfile, rather than whether the committed lockfile resolves the declared
dependency graph.

Product CI now runs `cargo metadata --locked --format-version 1` and checks
that the tracked `Cargo.lock` remains unchanged. The local CI contract pins the
command. On the same head, metadata passed; in an isolated Cargo fixture,
adding a dependency without updating its lockfile made metadata fail with
`--locked`. This keeps the consistency check fail-closed without regenerating
the lockfile during every CI run.
