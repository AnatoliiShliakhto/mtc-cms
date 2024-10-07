use super::*;

pub async fn request_fetch_entries_task(
    url: Cow<'static, str>,
) -> Option<Vec<Entry>> {
    use_api_client()()
        .get(&*url)
        .send()
        .await
        .consume_entries()
        .await
}