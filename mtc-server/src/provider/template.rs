use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Template {
    pub index_html: Cow<'static, str>,
}

impl Template {
    pub async fn init(config: &Config) -> Self {
        let index_html =
            tokio::fs::read_to_string(format!("{}/index.html", config.paths.www_path))
                .await
                .expect("should be able to read the index.html file");

        Template {
            index_html: Cow::from(index_html),
        }
    }
}