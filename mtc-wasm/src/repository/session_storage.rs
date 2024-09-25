use super::*;

pub fn use_session_storage<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UseSessionStorage<T> {
    let state = use_signal(move || {
        let key = Cow::Owned(key.to_string());
        let value = SessionStorage::get(&key)
            .ok()
            .unwrap_or_else(init);
        StorageEntry { key, value }
    });

    UseSessionStorage { inner: state }
}

#[derive(Clone, Copy)]
pub struct UseSessionStorage<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UseSessionStorage<T> {
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        SessionStorage::set(&inner.key, &value)
            .map_err(|e| error!("{e:#?}"))
            .unwrap();
        inner.value = value;
    }
}