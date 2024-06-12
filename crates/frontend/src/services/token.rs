use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    pub static ref TOKEN: RwLock<Option<String>> = RwLock::new(None);
}

/// Set jwt in memory.
pub fn set_auth_token(token: Option<String>) {
    let mut guard = TOKEN.write();
    *guard = token;
}

/// Get jwt token from lazy static.
pub(super) fn get_auth_token() -> Option<String> {
    let guard = TOKEN.read();
    guard.clone()
}
