use super::*;

pub trait ResponseService {
    fn get_value(self) -> impl std::future::Future<Output=Value> + Sized;
    fn is_ok(self) -> impl std::future::Future<Output=bool> + Sized;
}

impl ResponseService for Result<Response, reqwest::Error> {
    /// If the response is a success (200-299), returns the JSON value of the
    /// response. Otherwise, opens an error dialog with the message from the
    /// JSON value of the response, and returns [`Value::Null`].
    ///
    /// If the response is not a success, but there is no JSON value, the
    /// default error message is "error-generic".
    ///
    /// If the response is not a success, and there is no JSON value, and the
    /// JSON value could not be fetched, the default error message is
    /// "error-fetch".
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

    /// If the response is a success (200-299), returns `true`. Otherwise,
    /// opens an error dialog with the message from the JSON value of the
    /// response, and returns `false`.
    ///
    /// If the response is not a success, but there is no JSON value, the
    /// default error message is "error-generic".
    ///
    /// If the response is not a success, and there is no JSON value, and the
    /// JSON value could not be fetched, the default error message is
    /// "error-fetch".
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
