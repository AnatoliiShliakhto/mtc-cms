# # Debug build  

### Start stand-alone SurrealDB server 
```bash
surreal start --log info --user root --password root --bind 0.0.0.0:8000 rocksdb://./data/db
```

### Compile TailwindCSS assets
```bash
npx tailwindcss -i ./mtc-wasm/input.css -o ./assets/tailwind.css --minify
```

### MTC-Server (back-end)
```bash
cargo run
```

### MTC-WASM (front-end)
```bash
dx serve --package mtc-cms
```

### Native Application (desktop)
```bash
cargo tauri dev
```

### Android Application (mobile)
```bash
cargo tauri android dev
```

>---

# # Release build

### Compile TailwindCSS assets
```bash
npx tailwindcss -i ./mtc-wasm/input.css -o ./assets/tailwind.css --minify
```

### Linux MTC-Server (back-end)
```bash
cross build --release --target x86_64-unknown-linux-musl
```

### MTC-WASM (front-end)
```bash
dx build --release --package mtc-cms
```

### Native Application (desktop)
```bash
cargo tauri build
```

### Android Application (mobile)
```bash
cargo tauri android build --apk
```