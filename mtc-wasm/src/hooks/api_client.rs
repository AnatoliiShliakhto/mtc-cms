use super::*;

pub fn use_init_api_client() -> Client {
    use_context_provider(Client::new)
}

pub fn use_api_client() -> Client {
    use_context()
}