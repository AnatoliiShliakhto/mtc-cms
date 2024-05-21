use axum::BoxError;
use axum::extract::Host;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use tokio::net::TcpListener;

// addr as tuple (host, http port, https port)
pub async fn redirect_http_to_https(addr: (String, String, String)) {
    fn make_https(
        host: String,
        uri: Uri,
        ports: (String, String),
    ) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.0, &ports.1);
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let listener = TcpListener::bind(format!("{}:{}", &addr.0, &addr.1)).await.unwrap();

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, (addr.1, addr.2)) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}