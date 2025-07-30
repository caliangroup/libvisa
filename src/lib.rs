//! # NIST VISA library bindings
//!
//! This crate provides a safe wrapper around the NIST VISA library.
//!
//! It is designed to be as simple to use as possible.
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod variadic_macro;

// Internal bindings - raw library access
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(missing_docs)]
#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(clippy::unreadable_literal)]
pub mod bindings;

pub mod attribute;
pub mod error;
pub mod event;
pub mod security_cookie;

#[macro_use]
mod session;
pub use session::*;

mod resource_manager;
pub use resource_manager::*;

/// Only for testing
///
/// Retrieve a local device session, or panic.  
///
/// # Panics
/// Panics if not USB device is found, or if the device cannot be opened
#[must_use]
#[cfg(test)]
pub fn get_local_device() -> Session {
    use std::time::Duration;

    use attribute::AsViWritable;

    let rm = ResourceManager::new().expect("Failed to create resource manager");

    //
    // Do not search if an interface is provided
    let interface = std::env::var("LOCAL_DEVICE_INTF").unwrap_or_else(|_| {
        let mut search = rm
            .search("USB?*INSTR")
            .expect("Failed to search for USB devices");

        let device = search
            .next()
            .expect("No local USB devices found! Cannot continue testing");

        device.interface().to_string()
    });

    let mut device =
        Session::new(&rm, &interface, SessionOptions::default()).expect("Failed to open device");
    attribute::misc::TmoValue::write(&mut device, Duration::from_secs(2))
        .expect("Failed to set timeout");

    device
}
