# MTC-CMS (Military Training Center CMS) 
RUST Content Management System Core Project `In-Dev`
> **MTC-API** `back-end`
- RUST - [Rust Programming Language](https://www.rust-lang.org/)
- Axum - [modular web framework built with Tokio, Tower, and Hyper](https://github.com/tokio-rs/axum)
- SurrealDB [Embedded Database engine](https://surrealdb.com/) over **SpeeDB** or **RocksDB**
> **MTC-WEB** `front-end`
- Dioxus - [Rust WASM GUI library](https://dioxuslabs.com/)
- Tailwind CSS - [CSS styles](https://tailwindcss.com/)
- daisyUI - [Components for Tailwind CSS](https://daisyui.com/)

> **Docker** `container` production release
- ALPINE - [OS Linux x86_64-unknown-linux-musl](https://www.alpinelinux.org/)

## Pre-requires
- Rust, SurrealDB, Node.Js
- **CMake** to compile C++/ASM embedded database engine libraries
- Dioxus
```bash
cargo install dioxus-cli
```
- Cross-RS for cross-compile
```bash
cargo install cross --git https://github.com/cross-rs/cross
```
- PEM certificate files (or make them for localhost with [**MkCert**](https://github.com/Subash/mkcert) utility)
```bash
mkcert -install
mkcert localhost
```
```
Rename files:
 
localhost.pem -> ./cert/cert.pem
localhost-key.pem -> ./cert/key.pem
.env-example -> .env
```
```
[POST]: https://localhost/api/setup
```

## Debug
> Running *(step by step)* or use IDE like JetBrains [RustRover](https://www.jetbrains.com/rust/)
- Standalone [**SurrealDB**](https://surrealdb.com) `[OPTIONAL] DB manager:` [**Surrealist**](https://surrealdb.com/surrealist)
```bash 
surreal start --log info --user root --password root --bind 0.0.0.0:8000 file://./data
```
- **MTC-API** `back-end`
```bash
cargo watch -c -w ./mtc-api/src -x 'run --package mtc-api --bin mtc-api'
```
- **Tailwind CSS** `front-end`
```bash
npx tailwindcss -i ./mtc-web/input.css -o ./mtc-web/assets/tailwind.css --watch
```
- **Dioxus** `front-end`
```bash
cd mtc-web
```
```bash
dx serve
```

## Release
> Compile *(step by step)*
```bash 
cross build --release --target x86_64-unknown-linux-musl
```
```bash
cd mtc-web
```
```bash
dx build --release
```
```bash
cd ..
```
> Make Docker container
```bash
docker compose up --build
```

### ---- Description will be soon ----

## Roadmap
- [x] HTTP/HTTPS server
- [x] Authentication middleware
- [x] Core REST API end-points
- [ ] Single type API 
- [ ] Collection type API
- [ ] File manager API
- [ ] Protected file store
- [ ] SQL Migrations API 
- [ ] Front-end admin panel
- [ ] Quiz API (tests, exams etc.)
- [ ] Cross-platform `Tauri App` for semi-offline using *Windows, Android, iOS*
- [ ] `OPTIONAL` GraphQL API