use super::*;

pub trait ResponseService {
    fn get_value(self) -> impl std::future::Future<Output=Value> + Sized;
    fn is_ok(self) -> impl std::future::Future<Output=bool> + Sized;
}

impl ResponseService for Result<Response, reqwest::Error> {
    async fn get_value(self) -> Value {
        let Ok(response) = self else {
            error_dialog!("error-connection");
            return Value::Null
        };

        if !response.status().is_success() {
            let value = response.json::<Value>().await.unwrap_or_default();
            let message: Cow<'static, str> = value
                .key_string("message")
                .unwrap_or("error-generic".to_string()).into();
            error_dialog!(&message);
            return Value::Null
        }

        let Ok(value) = response.json::<Value>().await else {
            error_dialog!("error-fetch");
            return Value::Null
        };

        value
    }

    async fn is_ok(self) -> bool {
        let Ok(response) = self else {
            error_dialog!("error-connection");
            return false
        };

        if !response.status().is_success() {
            let value = response.json::<Value>().await.unwrap_or_default();
            let message: Cow<'static, str> = value
                .key_string("message")
                .unwrap_or("error-generic".to_string()).into();
            error_dialog!(&message);
            return false
        }

        true
    }
}
