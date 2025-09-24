# Best Practice Registration Demo (Rust + Axum)

A clean, best-practice demo service showing how to build a **registration & login flow in Rust**.  
Designed as a portfolio piece for CV/GitHub → demonstrates async-first APIs, domain separation, and scalable architecture.

---

## 🎯 Purpose
- Show how to build beyond "toy endpoints."
- Demonstrate **Axum + Tokio** with clean project structure.
- Provide a foundation for future domain extensions (compliance, marketing, limits).

---

## 🚀 Scope (Phase 1)
Endpoints implemented in this phase:

- `POST /register` → creates a new player (UUID, username, email, password)
- `POST /login` → authenticates a player, returns a session token
- `GET /player/:id` → fetch player info (from in-memory store)
- `GET /health` → simple closure-based health check

👉 All state is held in `Vec<Player>`, wrapped in `Arc<Mutex<AppState>>` (no DB yet).

---

## 📂 Project Layout
