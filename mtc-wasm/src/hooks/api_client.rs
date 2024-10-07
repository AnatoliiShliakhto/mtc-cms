use super::*;

pub fn use_init_api_client() {
    use_context_provider(UseApiClient::default);
}

pub fn use_api_client() -> Signal<Client> {
    use_context::<UseApiClient>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UseApiClient {
    inner: Signal<Client>,
}