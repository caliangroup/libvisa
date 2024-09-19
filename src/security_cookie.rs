//! Security cookie implementation.
use crate::bindings;

pub struct SecurityCookie(usize);
impl SecurityCookie {
    /// Initialize the security cookie system
    pub fn init() {
        unsafe { bindings::__security_init_cookie() }
    }

    /// Create a new security cookie
    /// You should call `SecurityCookie::init()` before calling this method
    pub fn new() -> Self {
        let cookie = unsafe { bindings::__security_cookie };
        SecurityCookie(cookie)
    }

    /// Check the security cookie
    pub fn check(&self) {
        unsafe { bindings::__security_check_cookie(self.0) }
    }

    pub fn report_gsfailure(&self) -> ! {
        unsafe { bindings::__report_gsfailure() }
    }
}

impl Default for SecurityCookie {
    fn default() -> Self {
        SecurityCookie::new()
    }
}
