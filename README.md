# MTC-CMS (Military Training Center CMS) 
>**WEB-SITE:** [242 Unit Training Center](https://242.org.ua)

RUST Content Management System Core Project `In-Dev`

> **MTC-API** `back-end`
- RUST - [Rust Programming Language](https://www.rust-lang.org/)
- Axum - [modular web framework built with Tokio, Tower, and Hyper](https://github.com/tokio-rs/axum)
- SurrealDB [Embedded Database engine](https://surrealdb.com/) over **RocksDB**
> **MTC-WEB** `front-end`
- Dioxus - [Rust WASM GUI library](https://dioxuslabs.com/)
- Tailwind CSS - [CSS styles](https://tailwindcss.com/)
- daisyUI - [Components for Tailwind CSS](https://daisyui.com/)

> **Docker** `container` production release
- ALPINE - [OS Linux x86_64-unknown-linux-musl](https://www.alpinelinux.org/)

## Pre-requires
- Rust, SurrealDB, Node.Js
- **LVMM** + **CMake** to compile C++/ASM embedded database engine libraries
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
 
localhost.pem -> ./data/cert/ssl.crt
localhost-key.pem -> ./data/cert/private.key
.env-example -> .env
```

## Settings
- **.cargo/config.toml**
- WASM front-end packs: **/mtc-wasm/packs**
- rename **.env.example** to **.env**

## Debug
> Running *(step by step)* or use IDE like JetBrains [RustRover](https://www.jetbrains.com/rust/)
- Standalone [**SurrealDB**](https://surrealdb.com) `[OPTIONAL] DB manager:` [**Surrealist**](https://surrealdb.com/surrealist)
```bash 
surreal start --log info --user root --password root --bind 0.0.0.0:8000 rocksdb://./data/db
```
- **MTC-API** `back-end`
```bash
cargo watch -c -w ./mtc-wasm/src -x 'run --package mtc-wasm --bin mtc-wasm'
```
- **Tailwind CSS** `front-end`
```bash
npx tailwindcss -i ./mtc-wasm/input.css -o ./mtc-wasm/assets/assets/tailwind.css --watch --minify
```
- **Dioxus** `front-end`
```bash
cd mtc-wasm
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
cd mtc-wasm
```
```bash
dx build --release
```

### ---- Description will be soon ----

## Roadmap
- [x] HTTP/HTTPS server
- [x] Authentication middleware
- [x] Core REST API end-points
- [x] Custom API service
- [x] File manager API
- [x] Private file store
- [x] SQL Migrations API 
- [x] Front-end admin panel
- [ ] Schedule module
- [ ] Learning module
- [ ] Quiz module
- [ ] Instructor utils module
- [ ] Cross-platform `Tauri App` for semi-offline using *Windows, Android, iOS*