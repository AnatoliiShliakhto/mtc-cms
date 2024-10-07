use super::*;

pub fn use_local_storage<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UseLocalStorage<T> {
    let state = use_signal(move || {
        let key = key.to_string();
        let value = LocalStorage::get(&key)
            .ok()
            .unwrap_or_else(init);
        StorageEntry { key, value }
    });

    UseLocalStorage { inner: state }
}

#[derive(Clone, Copy)]
pub struct UseLocalStorage<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UseLocalStorage<T> {
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        LocalStorage::set(&inner.key, &value)
            .map_err(|e| error!("{e:#?}"))
            .unwrap();
        inner.value = value;
    }
}
