use dioxus::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use tracing::debug;

/// Storage that persists across application reloads
#[derive(Clone, Debug)]
pub struct UsePersistent<T: 'static + Clone> {
    key: String,
    default: T,
}

/* =============================================================================
DIOXUS WEB TARGET
============================================================================= */

impl<T: Serialize + DeserializeOwned + Clone + Debug + 'static> UsePersistent<T> {
    /// Returns a clone of the value
    pub fn get(&self) -> T {
        let value = LocalStorage::get(&self.key)
            .ok()
            .unwrap_or(self.default.clone());
        debug!("Storage: Getting {:?} => {:?}", self.key, value);
        value
    }

    /// Sets the value
    pub fn set(&mut self, value: T) {
        debug!("Storage: Setting {:?} => {:?}", self.key, value);
        LocalStorage::set(&self.key, &value).expect("Failed to save to local storage");
    }

    /// Remove the value from local storage and set the current
    /// value to the default provided when creating the UsePersistent object,
    pub fn remove(&mut self) {
        debug!("Storage: Removing {:?}", self.key);
        LocalStorage::delete(&self.key);
    }

    /// Creates a new StorageEntry.
    ///
    /// Used to embed UsePersistent into other hooks without creating a new Signal.
    pub(super) fn new(key: impl ToString, init: impl FnOnce() -> T) -> Self {
        let key = key.to_string();
        let default = init();
        let mut current: Option<T> = LocalStorage::get(&key).ok();
        if current.is_none() {
            current = Some(default.clone());
            LocalStorage::set(&key, current.clone().unwrap())
                .expect("Failed to save to local storage");
        }
        debug!("Storage: Initializing {:?} => {:?}", key, current);
        Self { key, default }
    }
}

/// A persistent storage hook that can be used to store data across application reloads.
#[allow(clippy::needless_return)]
pub fn use_persistent<
    T: Serialize + DeserializeOwned + Default + Debug + Clone + 'static,
>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> Signal<UsePersistent<T>> {
    use_signal(move || UsePersistent::new(key, init))
}

#[cfg(test)]
mod tests {
    use super::UsePersistent;
    use gloo::storage::{LocalStorage, Storage};
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    fn clean() {
        LocalStorage::clear();
    }

    #[test]
    fn new_with_default() {
        let storage = UsePersistent::new("key".to_string(), || "My Value".to_string());
        assert_eq!(storage.get(), "My Value".to_string());
        clean()
    }

    #[test]
    fn new_with_value() {
        UsePersistent::new("key".to_string(), || "My Value".to_string());
        let storage2 = UsePersistent::new("key".to_string(), || "DEFAULT".to_string());
        assert_eq!(storage2.get(), "My Value".to_string());
        clean()
    }

    #[test]
    fn modified_and_persistent() {
        let mut storage1 =
            UsePersistent::new("key".to_string(), || "My Value".to_string());
        storage1.set("My modified Value".to_string());
        assert_eq!(storage1.get(), "My modified Value".to_string());

        let mut storage2 =
            UsePersistent::new("key".to_string(), || "DEFAULT".to_string());
        assert_eq!(storage2.get(), "My modified Value".to_string());

        storage2.set("Modified by storage2".to_string());
        assert_eq!(storage1.get(), "Modified by storage2".to_string());

        clean()
    }

    #[test]
    fn removed_and_set_to_default() {
        let mut storage1 =
            UsePersistent::new("key".to_string(), || "My Value".to_string());
        storage1.set("My modified Value".to_string());
        assert_eq!(storage1.get(), "My modified Value".to_string());

        storage1.remove();
        assert_eq!(storage1.get(), "My Value".to_string());

        let storage2 = UsePersistent::new("key".to_string(), || "DEFAULT".to_string());
        assert_eq!(storage2.get(), "DEFAULT".to_string());

        clean()
    }
}
