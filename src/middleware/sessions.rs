use secure_session::middleware::{SessionMiddleware, SessionConfig};
use secure_session::session::{SessionManager, ChaCha20Poly1305SessionManager};
use typemap;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
}

pub struct SessionKey {}

impl typemap::Key for SessionKey {
    type Value = Session;
}

pub fn get_session_middleware(secret: String) -> SessionMiddleware<Session, SessionKey, ChaCha20Poly1305SessionManager<Session>> {
    let manager = ChaCha20Poly1305SessionManager::<Session>::from_password(secret.as_bytes());
    let config = SessionConfig::default();

    SessionMiddleware::<Session, SessionKey, ChaCha20Poly1305SessionManager<Session>>::new(manager, config)
}
