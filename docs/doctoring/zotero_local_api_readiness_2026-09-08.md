# Zotero Local API readiness (2026-09-08)

An unauthenticated, read-only loopback probe to
`http://127.0.0.1:23119/api/users/0/items?limit=1` completed with HTTP 200.
Only response metadata was retained; item JSON and bibliographic content were
not copied into this evidence file.

- Zotero version: `10.0.1`
- Connector API version: `3`
- Zotero API version: `3`
- Schema version: `44`
- Total records reported: `8,326`
- Last-modified version: `2`
- Server identity: present (opaque value intentionally withheld)

This proves local endpoint availability and response-contract metadata only. It
does not prove a complete atomic snapshot, steward decision, approval, write,
protected merge, release, or deployment. The next read must use the existing
bounded capture/report path and preserve the private-file boundary.
