# Codex Protocol

Before writing code in this repository, Codex must read the relevant files under `AI_PROTOCOLS/`.

## Required Reading

At the start of a coding task:

1. Read `AI_PROTOCOLS/AI_WORKFLOW.md`.
2. Read the protocol file that matches the area being changed:
   - Database or migrations: `AI_PROTOCOLS/MIGRATION_GUIDE.md` and `AI_PROTOCOLS/TABLE_ADDING_PROTOCOL.md`
   - Repository layer: `AI_PROTOCOLS/REPO_GUIDE.md`
   - Service layer: `AI_PROTOCOLS/SERVICE_GUIDE.md`
   - Web server, routes, handlers, DTOs, SSE, or WebSocket: `AI_PROTOCOLS/WEB_SERVER_GUIDE.md`
   - Auth integration: `AI_PROTOCOLS/AUTH_INTEGRATION_GUIDE.md`
   - Error codes: `AI_PROTOCOLS/ERROR_CODE_GUIDE.md`
   - LLM client or model calls: `AI_PROTOCOLS/LLM_CLIENT_GUIDE.md`

## Working Rule

Use the protocol documents as the project boundary. Follow the existing migration, repo, service, and web-server examples before adding new patterns.

If a requested change introduces behavior that is not covered by `AI_PROTOCOLS/`, update the protocol or API docs together with the code.
