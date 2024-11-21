use super::*;

static JS_SERVICE_WORKER: &str = include_str!("../js/sw.js");

#[derive(Deserialize)]
pub struct ServiceWorkerQuery {
    pub session: Cow<'static, str>,
}

pub async fn service_worker_handler(
    Query(service_worker): Query<ServiceWorkerQuery>,
) -> Result<impl IntoResponse> {
    let re = regex::Regex::new(UUID_PATTERN).unwrap();
    if !re.is_match(&service_worker.session) {
        return Err(GenericError::BadRequest)?
    }

    let precache =
        tokio::fs::read_to_string(format!("{}/www/precache.txt", env!("DATA_PATH")))
            .await.unwrap_or_default();
    let precache: Vec<String> = precache
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect();
    let precache= format!("{:?}", precache);

    let service_worker = JS_SERVICE_WORKER
        .replace("{precache}", &precache)
        .replace("{front_end_url}", env!("FRONT_END_URL"))
        .replace("{session}", &service_worker.session);

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/javascript"));

    Ok((headers, service_worker))
}