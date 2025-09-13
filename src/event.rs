//! Async handling for VISA events
#![expect(
    clippy::cast_possible_wrap,
    reason = "Needed for compatibility with the VISA library"
)]

use crate::{bindings, error::Error};

/// The types of events that can be handled
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Event {
    IoCompletion = bindings::VI_EVENT_IO_COMPLETION,
    Trig = bindings::VI_EVENT_TRIG,
    ServiceReq = bindings::VI_EVENT_SERVICE_REQ,
    Clear = bindings::VI_EVENT_CLEAR,
    Exception = bindings::VI_EVENT_EXCEPTION,
    GpibCic = bindings::VI_EVENT_GPIB_CIC,
    GpibTalk = bindings::VI_EVENT_GPIB_TALK,
    GpibListen = bindings::VI_EVENT_GPIB_LISTEN,
    VxiVmeSysfail = bindings::VI_EVENT_VXI_VME_SYSFAIL,
    VxiVmeSysreset = bindings::VI_EVENT_VXI_VME_SYSRESET,
    VxiSigp = bindings::VI_EVENT_VXI_SIGP,
    VxiVmeIntr = bindings::VI_EVENT_VXI_VME_INTR,
    PxiIntr = bindings::VI_EVENT_PXI_INTR,
    TcpipConnect = bindings::VI_EVENT_TCPIP_CONNECT,
    UsbIntr = bindings::VI_EVENT_USB_INTR,
    All = bindings::VI_ALL_ENABLED_EVENTS,
}
impl TryFrom<u32> for Event {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            bindings::VI_EVENT_IO_COMPLETION => Ok(Self::IoCompletion),
            bindings::VI_EVENT_TRIG => Ok(Self::Trig),
            bindings::VI_EVENT_SERVICE_REQ => Ok(Self::ServiceReq),
            bindings::VI_EVENT_CLEAR => Ok(Self::Clear),
            bindings::VI_EVENT_EXCEPTION => Ok(Self::Exception),
            bindings::VI_EVENT_GPIB_CIC => Ok(Self::GpibCic),
            bindings::VI_EVENT_GPIB_TALK => Ok(Self::GpibTalk),
            bindings::VI_EVENT_GPIB_LISTEN => Ok(Self::GpibListen),
            bindings::VI_EVENT_VXI_VME_SYSFAIL => Ok(Self::VxiVmeSysfail),
            bindings::VI_EVENT_VXI_VME_SYSRESET => Ok(Self::VxiVmeSysreset),
            bindings::VI_EVENT_VXI_SIGP => Ok(Self::VxiSigp),
            bindings::VI_EVENT_VXI_VME_INTR => Ok(Self::VxiVmeIntr),
            bindings::VI_EVENT_PXI_INTR => Ok(Self::PxiIntr),
            bindings::VI_EVENT_TCPIP_CONNECT => Ok(Self::TcpipConnect),
            bindings::VI_EVENT_USB_INTR => Ok(Self::UsbIntr),
            bindings::VI_ALL_ENABLED_EVENTS => Ok(Self::All),
            _ => Err(Error::default()),
        }
    }
}

/// Event handling mechanisms
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum HandlingMechanism {
    Queue = bindings::VI_QUEUE,
    Handler = bindings::VI_HNDLR,
    SuspendHandler = bindings::VI_SUSPEND_HNDLR,
}

/// A simple handler for VISA events.
///
/// If you need to make use of the visa `user_data` field, use [`HandlerWithData`] instead.
pub trait Handler {
    /// Handles the event itself
    ///
    /// # Errors
    /// Should return an error if the event cannot be handled
    fn handle(
        session: bindings::ViSession,
        event_type: Event,
        event: bindings::ViEvent,
    ) -> Result<(), Error>;
}
impl<T: Handler> HandlerWithData for T {
    type Data = std::ffi::c_void;

    fn handle(
        session: bindings::ViSession,
        event_type: Event,
        event: bindings::ViEvent,
        _user_data: &Self::Data,
    ) -> Result<(), Error> {
        Self::handle(session, event_type, event)
    }
}

/// A more complex handler with the ability to use provided data from VISA.
///
/// If you do not need that, use [`Handler`] instead.
pub trait HandlerWithData {
    /// Data type expected by the handler, use `std::ffi::c_void` if no data is needed
    type Data;

    /// Handles the event itself
    ///
    /// # Errors
    /// Should return an error if the event cannot be handled
    fn handle(
        session: bindings::ViSession,
        event_type: Event,
        event: bindings::ViEvent,
        user_data: &Self::Data,
    ) -> Result<(), Error>;

    /// Convert the handler to a C handler
    /// # Safety
    /// Do not use directly, use `register_handler` instead
    unsafe extern "system" fn c_handler(
        session: bindings::ViSession,
        event_type: bindings::ViEventType,
        event: bindings::ViEvent,
        user_data: bindings::ViAddr,
    ) -> bindings::ViStatus {
        // Convert the event type
        let Ok(event_type) = Event::try_from(event_type) else {
            return bindings::VI_ERROR_INV_EVENT;
        };

        // Janky cast to get the user data
        let user_data: *mut Self::Data = user_data.cast::<Self::Data>();
        let user_data: &Self::Data = &*user_data;

        match Self::handle(session, event_type, event, user_data) {
            Ok(()) => bindings::VI_SUCCESS as bindings::ViStatus,
            Err(e) => e.status as bindings::ViStatus,
        }
    }

    /// Convert the handler to a C handler
    /// # Safety
    /// Do not use directly, use `register_handler` instead
    fn into() -> bindings::ViHndlr
    where
        Self: Sized,
    {
        Some(Self::c_handler)
    }
}
