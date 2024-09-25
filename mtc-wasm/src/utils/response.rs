use super::*;

pub trait ConsumeResponse {
    fn consume(self) -> impl std::future::Future<Output=Result<Response, Error>> + Sized;
    fn consume_value(self) -> impl std::future::Future<Output=Value> + Sized;
    fn consume_entries(self) -> impl std::future::Future<Output=Option<Vec<Entry>>> + Sized;
}

impl ConsumeResponse for Result<Response, reqwest::Error> {
    async fn consume(self) -> Result<Response, Error> {
        let Ok(response) = self else {
            Err(Error::Network("error-connection".into()))?
        };

        if !response.status().is_success() {
            let message = response.json::<Value>()
                .await?
                .get_str("message").unwrap_or("error-generic".into());

            return Err(Error::Response(message));
        }

        Ok(response)
    }

    async fn consume_value(self) -> Value {
        let message_box_task = use_coroutine_handle::<MessageBoxAction>();
        
        match self.consume().await {
            Ok(response) => {
                response.json::<Value>()
                    .await
                    .unwrap_or_default()
            },
            Err(e) => {
                message_box_task.send(MessageBoxAction::Error(e.message()));
                Value::Null
            }
        }
    }

    async fn consume_entries(self) -> Option<Vec<Entry>> {
        let message_box_task = use_coroutine_handle::<MessageBoxAction>();

        match self.consume().await {
            Ok(response) => {
                if let Ok(entries) = response.json::<Vec<Entry>>().await {
                    Some(entries)
                } else { None }
            },
            Err(e) => {
                message_box_task.send(MessageBoxAction::Error(e.message()));
                None
            }
        }
    }
}
