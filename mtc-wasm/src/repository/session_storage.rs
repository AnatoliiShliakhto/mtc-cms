use super::*;

/// Returns a reactive storage hook for session storage.
///
/// The hook will try to initialize itself with the value stored under the given `key`.
/// If no value is found, the hook will use the return value of the given `init` function
/// as the initial value.
///
/// # Arguments
///
/// * `key`: A string that will be used as the key for the session storage entry.
/// * `init`: A function that will be called to initialize the storage entry if no
///   value is found.
pub fn use_session_storage<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UseSessionStorage<T> {
    let state = use_signal(move || {
        let key = key.to_string();
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