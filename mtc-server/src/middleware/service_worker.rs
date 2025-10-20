use super::*;
use std::path::Path;
use std::sync::LazyLock;

static JS_SERVICE_WORKER: LazyLock<String> = LazyLock::new(|| {
    let mut precache = vec!["".to_string(), "/index.html".to_string()];


    let www_path = env("WWW_PATH", "/etc/242-mtc/www");
    let mut path = Path::new(&www_path);
    if cfg!(debug_assertions) {
        path = Path::new("./target/dx/mtc-wasm/debug/web/public").into()
    }

    for entry in std::fs::read_dir(path.join("assets")).unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            if !filename.ends_with(".br") {
                precache.push(format!("/assets/{}", filename));
            }
        }
    }

    let sw = std::fs::read_to_string(path.join("sw.js")).unwrap_or(
        include_str!("../../../mtc-wasm/assets/js/sw.js").to_string()
    );

    sw.replace("['/index.html']", &format!("{:?}", precache))
});

#[derive(Deserialize)]
pub struct ServiceWorkerQuery {
    pub session: Cow<'static, str>,
}

#[handler]
pub async fn service_worker_handler(
    Query(service_worker): Query<ServiceWorkerQuery>,
) {
    let re = regex::Regex::new(UUID_PATTERN).unwrap();
    if !re.is_match(&service_worker.session) {
        Err(GenericError::BadRequest)?
    }

    let service_worker = JS_SERVICE_WORKER.replace("{session}", &service_worker.session);

    Ok(([
            (CONTENT_TYPE, "application/javascript"),
            (CACHE_CONTROL, "no-cache; no-store; must-revalidate; private; max-age=0"),
        ], service_worker))
}

fn env(key: &str, default: &'static str) -> String {
    dotenv::var(key).unwrap_or(default.into())
}