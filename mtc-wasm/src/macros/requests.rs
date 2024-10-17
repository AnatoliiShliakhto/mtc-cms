#[macro_export]
macro_rules! value_future {
    ($url:expr) => {
        use_resource(move || {
            let url = $url.clone();
            async move {
                use_api_client()()
                    .get(&url)
                    .send()
                    .await
                    .get_value()
                    .await
            }
        })
    };

    ($url:expr, $value:ident) => {
        use_resource(move || {
            let url = $url.clone();
            let value = $value.clone();
            async move {
                 use_api_client()()
                    .post(&url)
                    .json(&value)
                    .send()
                    .await
                    .get_value()
                    .await
            }
        })
    };
}

#[macro_export]
macro_rules! get_request {
    ($url:expr) => {
        use_api_client()()
            .get(&$url)
            .send()
            .await
            .is_ok()
            .await
    };
}

#[macro_export]
macro_rules! post_request {
    ($url:expr) => {
        use_api_client()()
            .post(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        use_api_client()()
            .post(&$url)
            .json(&$value)
            .send()
            .await
            .is_ok()
            .await
    };
}

#[macro_export]
macro_rules! patch_request {
    ($url:expr) => {
        use_api_client()()
            .patch(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        use_api_client()()
            .patch(&$url)
            .json(&$value)
            .send()
            .await
            .is_ok()
            .await
    };
}

#[macro_export]
macro_rules! delete_request {
    ($url:expr) => {
        use_api_client()()
            .delete(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        use_api_client()()
            .delete(&$url)
            .json(&$value)
            .send()
            .await
            .is_ok()
            .await
    };
}