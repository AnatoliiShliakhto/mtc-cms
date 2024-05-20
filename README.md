# Military Training Center CMS

> RUST

- version >= 1.78

> SurrealDB over RocksDB

> Docker

- version >= 4.29.0

> Cargo CLI: CROSS-RS
> for cross-compile

```
cargo install cross --git https://github.com/cross-rs/cross

cross build --target x86_64-unknown-linux-musl

cross build --release --target x86_64-unknown-linux-musl
```

> CMS-WEB
- Install
```
cargo install dioxus-cli
```
- Build
```
npx tailwindcss -i ./mtc-web/input.css -o ./mtc-web/assets/tailwind.css --watch
```
```
dx serve
```
---- Description will be soon ----