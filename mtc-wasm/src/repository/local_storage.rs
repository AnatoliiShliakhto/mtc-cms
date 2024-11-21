use super::*;

pub fn use_local_storage(
    key: impl ToString,
    init: impl FnOnce() -> Value,
) -> UseLocalStorage {
    let state = use_signal(move || {
        let key = key.to_string();
        let mcrypt = new_magic_crypt!(env!("CRYPT_KEY"), 256);

        let value: String = LocalStorage::get(&key)
            .ok()
            .unwrap_or_else(|| {
                let init = init().to_string();
                let encrypted = mcrypt.encrypt_str_to_base64(init.clone());
                LocalStorage::set(&key, &encrypted)
                    .map_err(|e| error!("{e:#?}"))
                    .unwrap();
                encrypted
            });

        let decrypted = mcrypt.decrypt_base64_to_string(&value).unwrap_or_default();
        let value = serde_json::from_str(&decrypted).unwrap_or_default();

        StorageEntry { key, value }
    });

    UseLocalStorage { inner: state }
}

#[derive(Clone, Copy)]
pub struct UseLocalStorage {
    inner: Signal<StorageEntry<Value>>,
}

impl UseLocalStorage {
    pub fn get(&self) -> Value {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: Value) {
        let mut inner = self.inner.write();
        let mcrypt = new_magic_crypt!(env!("CRYPT_KEY"), 256);
        let encrypted = mcrypt.encrypt_str_to_base64(value.clone().to_string());
        LocalStorage::set(&inner.key, &encrypted)
            .map_err(|e| error!("{e:#?}"))
            .unwrap();
        inner.value = value;
    }
}
