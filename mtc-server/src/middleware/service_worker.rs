use super::*;

static JS_SERVICE_WORKER: &str = include_str!("../js/sw.js");

#[derive(Deserialize)]
pub struct ServiceWorkerQuery {
    pub session: Cow<'static, str>,
}

/// Handles service worker requests.
///
/// # Returns
///
/// Response with the generated service worker script.
pub async fn service_worker_handler(
    Query(service_worker): Query<ServiceWorkerQuery>,
) -> Result<impl IntoResponse> {
    let re = regex::Regex::new(UUID_PATTERN).unwrap();
    if !re.is_match(&service_worker.session) {
        return Err(GenericError::BadRequest)?
    }

    let precache = tokio::fs::read_to_string(format!("{}/www/precache.txt", env!("DATA_PATH")))
        .await
        .ok()
        .map(|s| s.lines().map(ToString::to_string).collect::<Vec<_>>())
        .unwrap_or_default();

    let service_worker = JS_SERVICE_WORKER
        .replace("{precache}", &format!("{:?}", precache))
        .replace("{front_end_url}", env!("FRONT_END_URL"))
        .replace("{session}", &service_worker.session);

    Ok(([(CONTENT_TYPE, "application/javascript")], service_worker))
}