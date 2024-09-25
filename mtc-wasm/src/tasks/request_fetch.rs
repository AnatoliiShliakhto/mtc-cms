use super::*;

pub async fn request_fetch_task(
    url: Cow<'static, str>
) -> Value {
    use_api_client()
        .get(&*url)
        .send()
        .await
        .consume_value()
        .await
}