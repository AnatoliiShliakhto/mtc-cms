#[macro_export]
macro_rules! value_future {
    ($url:expr) => {
        use_resource(move || {
            let url = $url.clone();
            async move {
                state!(client)
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
                 state!(client)
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
macro_rules! value_request {
    ($url:expr) => {
        state!(client)
            .get(&$url)
            .send()
            .await
            .get_value()
            .await
    };

    ($url:expr, $value:ident) => {
        state!(client)
            .post(&$url)
            .json(&$value)
            .send()
            .await
            .get_value()
            .await
    };
}

#[macro_export]
macro_rules! get_request {
    ($url:expr) => {
        state!(client)
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
        state!(client)
            .post(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        state!(client)
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
        state!(client)
            .patch(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        state!(client)
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
        state!(client)
            .delete(&$url)
            .send()
            .await
            .is_ok()
            .await
    };

    ($url:expr, $value:ident) => {
        state!(client)
            .delete(&$url)
            .json(&$value)
            .send()
            .await
            .is_ok()
            .await
    };
}