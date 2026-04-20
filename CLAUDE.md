# CLAUDE.md — MediaKraken Agent Rules

This file defines how AI agents must behave when making changes to this repository.
If these rules conflict with a user request, ASK for confirmation only if the change is destructive or high-risk;
otherwise, prefer the safest minimal change.

---

## 0) Prime Directives

1. **Be minimal.** Only change what is necessary to satisfy the request.
2. **Do not refactor** unrelated code, rename things, or reorganize folders unless explicitly requested.
3. **No breaking changes** to public interfaces unless explicitly requested.
4. **Never “guess” values** that must be exact (label pixel sizes, DPI, SQL schema, URLs). Use explicit constants.
5. **Prefer clarity over cleverness.** Readability and maintenance > micro-optimizations.

---

## 1) Repository Expectations (Rust Web App)

### Rust toolchain & quality
- Code must compile with:
  - `cargo check`
- Code must pass lints with:
  - `cargo clippy -- -D warnings`
- Formatting:
  - `cargo fmt`
- Avoid adding dependencies unless necessary; prefer standard library and existing crates.

### Error handling
- **No `unwrap()` / `expect()`** in production paths.
- Prefer a project-standard error type (e.g. `AppError`) and `thiserror` patterns if already present.
- Use `?` propagation with meaningful context (e.g. `anyhow::Context`) only if the project already uses it.

### Async/Tokio
- **Do not block the async runtime.**
  - No synchronous file I/O or heavy CPU in request handlers without `spawn_blocking`.
  - Avoid `tokio::spawn` unless necessary. If used:
  - explain why,
  - ensure proper cancellation/error handling,
  - avoid detached “fire-and-forget” unless explicitly requested.

---

## 2) Axum / HTTP Handlers

- Handlers should return `impl IntoResponse` (or project standard).
- Validate user input (query params, path params, JSON bodies).
- **Never trust client data.**
- Avoid leaking internal errors to clients; map to safe HTTP responses.

---

## 3) Database (Postgres / SQLx)

### Compile-time safety
- Prefer `sqlx::query!` / `query_as!` macros where possible.
- If dynamic SQL is required, it must be parameterized (no string interpolation).

### Migrations
- **Do not modify existing migration files** unless the user explicitly requests it.
- Do not drop/alter tables destructively unless explicitly requested.
- Any schema change should include:
  - migration,
  - code updates,
  - (if applicable) template/context updates.

### Performance & correctness
- Prefer set-based SQL over per-row loops.
- If adding indexes, ensure they match query patterns.
- Be mindful of locking: use explicit transactions when needed; keep them short.

---

## 4) Templates (Askama)

- Templates live under `/templates` (or project standard).
- Keep logic minimal in templates; do logic in Rust.
- If a template context struct changes, **update all templates** that use it.
- Ensure template structs derive required traits (`Serialize` etc.) only if needed and already used.

---

## 5) Frontend (TailwindCSS + Alpine.js)

- Tailwind only; avoid inline styles unless already established in project.
- Alpine.js for interactivity; keep components small and predictable.
- Mobile-first: ensure functionality works on small screens.
- Do not introduce new JS frameworks.

### UI behavior rules
- Any interactive icon/button must have:
  - proper `aria-label`,
  - stable hover/active behavior,
  - state-driven classes (avoid “hover flicker” bugs).

---

## 6) Brother QL-800 Label / Imaging Rules

These rules exist because label printing is extremely sensitive to exact pixel sizing.

- **When generating or modifying label images:**
  - Use **exact pixel dimensions**, explicitly stated.
  - Do **not** add borders unless explicitly requested.
  - “Tight / no border” means content flush to edges within safe print area only if user says so.

- **If the user specifies a pixel size (e.g. `732x342`)**
  - you MUST output exactly that size.
  - Do not “helpfully” adjust aspect ratio.
  - Do not change DPI metadata unless explicitly asked.

- If a label spec is “62mm x 29mm”:
  - Do not compute px from mm unless the user asks you to.
  - Use the pixel dimensions the user provides as the source of truth.

---

## 7) CNPG / WAL / Object Storage (Garage S3)

- Never log secrets, tokens, access keys, or connection strings.
- If changing CNPG manifests:
  - avoid destructive actions (deleting PVCs, wiping WAL) unless explicitly requested.
  - call out any restart/reconcile requirements in the response.
- Treat endpoints, ports, and bucket names as exact:
  - do not “guess” corrections.

---

## 8) Security & Privacy

- No hardcoded credentials.
- Redact secrets in examples:
  - Use `REDACTED` or `${ENV_VAR}` placeholders.
- Do not introduce insecure defaults:
  - no `0.0.0.0` binds unless already used or explicitly requested,
  - no disabling TLS verification unless explicitly requested and clearly labeled unsafe.

---

## 9) Testing & Verification Expectations

When making changes, the agent should (at minimum) describe how to verify:

- Rust:
  - `cargo fmt`
  - `cargo check`
  - `cargo clippy -- -D warnings`
- For template changes:
  - run the app and load the relevant page(s)
- For SQL changes:
  - indicate the query paths affected
  - ensure parameterization
- For label/image output:
  - confirm pixel dimensions exactly match the requested size

---

## 10) Output & Collaboration Style

- Prefer patches/diffs or clearly separated “before/after” snippets.
- State assumptions explicitly.
- If multiple approaches exist, pick the safest minimal one and mention the alternative briefly.
- If a request is ambiguous, make a reasonable choice that is reversible and non-destructive.

---

## 11) Do Not Touch (Unless Explicitly Asked)

- `Cargo.lock` (unless dependency changes are requested)
- Production secrets / `.env` files
- Existing migrations (edit-only is destructive history)
- Backup/WAL archives
- Any data directories (PVC contents)

---

## 12) Quick Project Glossary (Optional)

- MediaKraken: Rust/Axum app + Askama templates + Tailwindv4/Alpine UI
- DB: Postgres via SQLx
- CNPG: CloudNativePG for Postgres ops
- Garage S3: object storage backend for backups/WAL

