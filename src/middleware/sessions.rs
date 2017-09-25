use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use iron_sessionstorage::Value;

pub struct Login {
    pub id: String
}

impl Value for Login {
    fn get_key() -> &'static str { "auth" }
    fn into_raw(self) -> String { self.id }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Login { id: value })
        }
    }
}

pub fn get_session_middleware(secret: &str) -> SessionStorage<SignedCookieBackend> {
    let secret_bytes = secret.as_bytes().to_vec();

    SessionStorage::new(SignedCookieBackend::new(secret_bytes))
}
