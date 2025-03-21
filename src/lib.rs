//! # NIST VISA library bindings
//!
//! This crate provides a safe wrapper around the NIST VISA library.  
//! It is designed to be as simple to use as possible.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod variadic_macro;

// Internal bindings - raw library access
#[allow(non_camel_case_types)]
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
