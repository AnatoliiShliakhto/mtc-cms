use super::*;

static JS_SERVICE_WORKER: &str = include_str!("../js/sw.js");

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

    let precache = tokio::fs::read_to_string(format!("{}/www/precache", env!("DATA_PATH")))
        .await
        .ok()
        .map(|s| s.lines().map(ToString::to_string).collect::<Vec<_>>())
        .unwrap_or_default();

    let service_worker = JS_SERVICE_WORKER
        .replace("['/index.html']", &format!("{:?}", precache))
        .replace("{session}", &service_worker.session);

    Ok(([
            (CONTENT_TYPE, "application/javascript"),
            (CACHE_CONTROL, "no-cache; no-store; must-revalidate; private; max-age=0"),
        ], service_worker))
}