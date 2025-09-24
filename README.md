# Best Practice Registration Demo (Rust + Axum)

A clean PoC showing how to build a **registration & login-ready flow** in Rust.

- ✅ Async-first (Axum 0.8.4 + Tokio)
- ✅ Secure (Argon2id password hashing, no plaintext in API)
- ✅ Domain-driven layout (models, handlers, state, routes)
- ✅ Seeded demo users at startup
- ✅ Ready for extension (compliance, events, DB)

---

## Endpoints

- `GET /health` → `"OK"`
- `POST /register` → create new player
- `GET /player/{id}` → lookup player (returns `{found, player}`)
- (Seeded players Alice, Bob, Charlie with random UUIDs at startup)

---

## Seeded Players

On startup, console shows:

Seeded player: alice (alice@example.com) -> <uuid>
password = password1
hash = $argon2id$...
Seeded player: bob (bob@example.com) -> <uuid>
password = hunter2
hash = $argon2id$...
Seeded player: charlie (charlie@example.com) -> <uuid>
password = letmein
hash = $argon2id$...

yaml
Copy code

Use these UUIDs for `/player/{id}` queries.

---

## Quickstart

```bash
cargo run
In PowerShell:

powershell
Copy code
# health
Invoke-RestMethod http://127.0.0.1:3000/health

# register new player
Invoke-RestMethod http://127.0.0.1:3000/register `
  -Method POST `
  -Body '{"username":"dave","email":"dave@example.com","password":"secret"}' `
  -ContentType "application/json"

# get player by id
Invoke-RestMethod http://127.0.0.1:3000/player/<uuid>
Roadmap
Add /players to list all users

Add /login with Argon2 verification + session token

Replace Vec with Postgres

Emit domain events (RegistrationCompleted, LoginSuccess)

Add compliance stubs

yaml
Copy code

---

## 2. `.vscode/settings.json`
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.allFeatures": true,
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer"
}
3. dev-setup.md
markdown
Copy code
# Dev Setup

## Prereqs
- [Rust](https://www.rust-lang.org/tools/install)
- VS Code with extensions:
  - rust-analyzer
  - crates
  - even better TOML

## Useful CLI Tools
```bash
rustup component add clippy rustfmt
cargo install cargo-edit cargo-watch
clippy → linting / best practices

rustfmt → auto formatting

cargo-edit → cargo add axum instead of manual Cargo.toml

cargo-watch → cargo watch -x run for live reload

Run
bash
Copy code
cargo run
Test (PowerShell)
powershell
Copy code
Invoke-RestMethod http://127.0.0.1:3000/health
yaml
Copy code

---

## 4. Suggested `.gitignore`
```gitignore
/target
**/*.rs.bk
Cargo.lock
