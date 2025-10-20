use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Template {
    pub index_html: Cow<'static, str>,
    pub gate_pass_back_html: Cow<'static, str>,
    pub gate_pass_front_html: Cow<'static, str>,
    pub gate_pass_email_html: Cow<'static, str>,
}

impl Template {
    pub async fn init(config: &Config) -> Self {
        let index_html = tokio::fs::read_to_string(format!("{}/index.html", config.paths.www_path))
            .await
            .expect("should be able to read the index.html file");

        let gate_pass_back_html =
            tokio::fs::read_to_string(format!("{}/gate_pass_back.html", config.paths.www_path))
                .await
                .expect("should be able to read the gate_pass_back.html file");

        let gate_pass_front_html =
            tokio::fs::read_to_string(format!("{}/gate_pass_front.html", config.paths.www_path))
                .await
                .expect("should be able to read the gate_pass_front.html file");

        let gate_pass_email_html =
            tokio::fs::read_to_string(format!("{}/gate_pass_email.html", config.paths.www_path))
                .await
                .expect("should be able to read the gate_pass_email.html file");

        Template {
            index_html: Cow::from(index_html),
            gate_pass_back_html: Cow::from(gate_pass_back_html),
            gate_pass_front_html: Cow::from(gate_pass_front_html),
            gate_pass_email_html: Cow::from(gate_pass_email_html),
        }
    }
}
