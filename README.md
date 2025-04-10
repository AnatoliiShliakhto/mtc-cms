# MTC-CMS (Military Training Center CMS) 
>**WEB-SITE:** [242 Unit Training Center](https://242.org.ua)

RUST Content Management System (CMS) `In-Dev`

> **MTC-SERVER** `back-end`
- RUST - [Rust Programming Language](https://www.rust-lang.org/)
- Axum - [modular web framework built with Tokio, Tower, and Hyper](https://github.com/tokio-rs/axum)
- SurrealDB [Embedded Database engine](https://surrealdb.com/) over **RocksDB**
> **MTC-WASM** `front-end`
- Dioxus - [Rust WASM GUI library](https://dioxuslabs.com/)
- Tailwind CSS - [CSS styles](https://tailwindcss.com/)
- daisyUI - [Components for Tailwind CSS](https://daisyui.com/)

> **MTC-APP** `desktop` & `mobile` application
- Tauri **v2.0** - [Create small, fast, secure, cross-platform applications](https://v2.tauri.app/)

## Pre-requires
- RUST Tool-Chains, SurrealDB, Node.Js
- **LLVM** + **CMake** + **NASM** to compile C++/ASM embedded database engine libraries
- Dioxus CLI
```bash
cargo install dioxus-cli
```
- Cross-RS for cross-compile
```bash
cargo install cross --git https://github.com/cross-rs/cross
```
- Cargo Make
```bash
cargo install --force cargo-make
```

- PEM certificate files (or make them for localhost with [**MkCert**](https://github.com/Subash/mkcert) utility)
```bash
mkcert -install
mkcert localhost
```
- Rename files:
```
localhost.pem -> ./data/cert/ssl.crt
localhost-key.pem -> ./data/cert/private.key
```
## Project Settings
Rename and modify settings file as you wish
```text
./.cargo/config.toml.example -> ./.cargo/config.toml
```

UI WASM `PACKS` in folder
```text
./mtc-wasm/src/packs/
```

## Project Description

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
- [x] Learning module
- [x] Cross-platform `Tauri App` for *Windows, Android, iOS*
- [ ] Quiz module
- [ ] Instructor utils module
