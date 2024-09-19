use crate::{bindings, error::Error};

/// Event type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Event handling mechanism
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlingMechanism {
    Queue = bindings::VI_QUEUE,
    Handler = bindings::VI_HNDLR,
    SuspendHandler = bindings::VI_SUSPEND_HNDLR,
}

pub type NoHandlerData = std::ffi::c_void;
pub trait Handler {
    /// Data type expected by the handler, use `NoHandlerData` if no data is needed
    type Data;

    /// Handle the event
    fn handle(
        session: bindings::ViSession,
        event_type: Event,
        event: bindings::ViEvent,
        user_data: &Self::Data,
    ) -> Result<(), Error>;

    /// Convert the handler to a C handler
    /// # Safety
    /// Do not use directly, use `register_handler` instead
    unsafe extern "stdcall" fn c_handler(
        session: bindings::ViSession,
        event_type: bindings::ViEventType,
        event: bindings::ViEvent,
        user_data: bindings::ViAddr,
    ) -> bindings::ViStatus {
        let event_type = match Event::try_from(event_type) {
            Ok(event_type) => event_type,
            Err(_) => return bindings::VI_ERROR_INV_EVENT,
        };

        let user_data: *mut Self::Data = user_data as *mut _ as *mut Self::Data;
        let user_data: &Self::Data = &*user_data;

        match Self::handle(session, event_type, event, user_data) {
            Ok(_) => bindings::VI_SUCCESS as bindings::ViStatus,
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
