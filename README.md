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
- ALPINE - [Linux OS x86_64-unknown-linux-musl](https://www.alpinelinux.org/)

## Pre-Requires
- Rust, SurrealDB, Node.Js
- CMake to compile C++ embedded database engine libraries
- Dioxus
```bash
cargo install dioxus-cli
```
- Cross-RS for cross-compile
```bash
cargo install cross --git https://github.com/cross-rs/cross
```
- PEM certificate (or make them for localhost with [**MkCert**]() utility)
```bash
mkcert -install
mkcert localhost
```
*Rename files to* **cert.pem, key.pem** *copy them to* **./cert** *directory* 
- rename **.env-example** to **.env**
```
POST: https://localhost/api/setup
```

## Debugging
> Requires
- Standalone **SurrealDB** with manager **Surrealist**
```bash 
surreal start --log info --user root --password root --bind 0.0.0.0:8000 file://./mtc-cms
```
- Tailwind CSS `front-end`
```bash
npx tailwindcss -i ./mtc-web/input.css -o ./mtc-web/assets/tailwind.css --watch
```
- Dioxus `front-end`
```bash
dx serve
```

## Release
> Compile
```bash
cross build --release --target x86_64-unknown-linux-musl
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
- [ ] Front-end admin panel