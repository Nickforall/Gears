use iron_csrf::csrf::{AesGcmCsrfProtection, CsrfProtection};
use iron_csrf::{CsrfProtectionMiddleware, CsrfConfig};

pub fn get_csrf_middleware(key: String) -> CsrfProtectionMiddleware<AesGcmCsrfProtection> {
    let protect = AesGcmCsrfProtection::from_password(key.as_bytes());
    let config = CsrfConfig::default();

    CsrfProtectionMiddleware::new(protect, config)
}
