//! Error handling for the VISA library
#![expect(
    clippy::cast_possible_wrap,
    reason = "Needed for compatibility with the VISA library"
)]
#![expect(
    clippy::cast_sign_loss,
    reason = "Needed for compatibility with the VISA library"
)]

use crate::bindings;
use std::{
    ffi::CString,
    fmt::{Display, Formatter},
};

/// An
#[derive(Debug, Clone, Default)]
pub struct Error {
    /// The error type returned by the VISA library
    pub status: ErrorType,

    /// A description of the error
    pub description: Option<String>,
}
impl Error {
    const ERROR_OK: &[i32] = &[
        bindings::VI_SUCCESS as i32,
        bindings::VI_SUCCESS_EVENT_EN as i32,
        bindings::VI_SUCCESS_EVENT_DIS as i32,
        bindings::VI_SUCCESS_QUEUE_EMPTY as i32,
        bindings::VI_SUCCESS_TERM_CHAR as i32,
        bindings::VI_SUCCESS_MAX_CNT as i32,
        bindings::VI_SUCCESS_DEV_NPRESENT as i32,
        bindings::VI_SUCCESS_TRIG_MAPPED as i32,
        bindings::VI_SUCCESS_QUEUE_NEMPTY as i32,
        bindings::VI_SUCCESS_NCHAIN as i32,
        bindings::VI_SUCCESS_NESTED_SHARED as i32,
        bindings::VI_SUCCESS_NESTED_EXCLUSIVE as i32,
        bindings::VI_SUCCESS_SYNC as i32,
    ];
    const MAX_DESC_LENGTH: usize = 1024;

    /// Create an error from a status code returned by the VISA library.
    ///
    /// If a session is provided, the error description will be fetched from the session.
    #[must_use]
    pub fn new(raw_status: i32, session: Option<bindings::ViSession>) -> Self {
        if Self::ERROR_OK.contains(&raw_status) {
            return Self::default();
        }

        match session {
            Some(session) => {
                // Read the error description from the device
                let mut buffer: Vec<i8> = vec![0; Self::MAX_DESC_LENGTH];
                let status = unsafe {
                    bindings::viStatusDesc(session, raw_status, buffer.as_mut_slice().as_mut_ptr())
                };

                let description = Self::ERROR_OK.contains(&status).then(|| {
                    // Transform into a u8 buffer
                    let buffer: Vec<u8> = buffer.iter().map(|&x| x as u8).collect();

                    // Get a slice including only a single \0
                    let mut buffer = buffer
                        .iter()
                        .take_while(|&&x| x != 0)
                        .copied()
                        .collect::<Vec<u8>>();
                    buffer.push(0);

                    // Into string
                    CString::from_vec_with_nul(buffer)
                        .ok()
                        .and_then(|cstr| cstr.into_string().ok())
                        .unwrap_or_default()
                });

                Self {
                    status: ErrorType::from(raw_status),
                    description,
                }
            }

            None => Self {
                status: ErrorType::from(raw_status),
                description: None,
            },
        }
    }

    /// Create an error from a message
    pub fn from_msg(description: impl AsRef<str>) -> Self {
        Self {
            status: ErrorType::default(),
            description: Some(description.as_ref().to_string()),
        }
    }

    /// Wrap a call to a VISA binding
    ///
    /// # Errors
    /// Retiurns an error if the status code is not `VI_SUCCESS`
    pub fn wrap_binding<F>(session: Option<bindings::ViSession>, f: F) -> Result<(), Self>
    where
        F: FnOnce() -> i32,
    {
        let status = f();
        if Self::ERROR_OK.contains(&status) {
            Ok(())
        } else {
            Err(Self::new(status, session))
        }
    }
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.description {
            Some(desc) => write!(f, "{}: {}", self.status, desc),
            None => write!(f, "{}", self.status),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            status: ErrorType::SystemError,
            description: Some(e.to_string()),
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Input contained a null byte".to_string()),
        }
    }
}

impl From<std::ffi::FromVecWithNulError> for Error {
    fn from(_: std::ffi::FromVecWithNulError) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Invalid UTF-8 string".to_string()),
        }
    }
}

impl From<std::ffi::IntoStringError> for Error {
    fn from(_: std::ffi::IntoStringError) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Invalid UTF-8 string".to_string()),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Invalid UTF-8 string".to_string()),
        }
    }
}

impl From<std::ffi::FromBytesWithNulError> for Error {
    fn from(_: std::ffi::FromBytesWithNulError) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Input contained a null byte".to_string()),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self {
            status: ErrorType::InvParameter,
            description: Some("Invalid UTF-8 string".to_string()),
        }
    }
}

/// An error type returned by the VISA library
#[repr(i32)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ErrorType {
    #[default]
    SystemError = bindings::VI_ERROR_SYSTEM_ERROR,
    InvObject = bindings::VI_ERROR_INV_OBJECT,
    RsrcLocked = bindings::VI_ERROR_RSRC_LOCKED,
    InvExpr = bindings::VI_ERROR_INV_EXPR,
    RsrcNfound = bindings::VI_ERROR_RSRC_NFOUND,
    InvRsrcName = bindings::VI_ERROR_INV_RSRC_NAME,
    InvAccMode = bindings::VI_ERROR_INV_ACC_MODE,
    Tmo = bindings::VI_ERROR_TMO,
    ClosingFailed = bindings::VI_ERROR_CLOSING_FAILED,
    InvDegree = bindings::VI_ERROR_INV_DEGREE,
    InvJobId = bindings::VI_ERROR_INV_JOB_ID,
    NsupAttr = bindings::VI_ERROR_NSUP_ATTR,
    NsupAttrState = bindings::VI_ERROR_NSUP_ATTR_STATE,
    AttrReadonly = bindings::VI_ERROR_ATTR_READONLY,
    InvLockType = bindings::VI_ERROR_INV_LOCK_TYPE,
    InvAccessKey = bindings::VI_ERROR_INV_ACCESS_KEY,
    InvEvent = bindings::VI_ERROR_INV_EVENT,
    InvMech = bindings::VI_ERROR_INV_MECH,
    HndlrNinstalled = bindings::VI_ERROR_HNDLR_NINSTALLED,
    InvHndlrRef = bindings::VI_ERROR_INV_HNDLR_REF,
    InvContext = bindings::VI_ERROR_INV_CONTEXT,
    Nenabled = bindings::VI_ERROR_NENABLED,
    Abort = bindings::VI_ERROR_ABORT,
    RawWrProtViol = bindings::VI_ERROR_RAW_WR_PROT_VIOL,
    RawRdProtViol = bindings::VI_ERROR_RAW_RD_PROT_VIOL,
    OutpProtViol = bindings::VI_ERROR_OUTP_PROT_VIOL,
    InpProtViol = bindings::VI_ERROR_INP_PROT_VIOL,
    Berr = bindings::VI_ERROR_BERR,
    InProgress = bindings::VI_ERROR_IN_PROGRESS,
    InvSetup = bindings::VI_ERROR_INV_SETUP,
    QueueError = bindings::VI_ERROR_QUEUE_ERROR,
    Alloc = bindings::VI_ERROR_ALLOC,
    InvMask = bindings::VI_ERROR_INV_MASK,
    Io = bindings::VI_ERROR_IO,
    InvFmt = bindings::VI_ERROR_INV_FMT,
    NsupFmt = bindings::VI_ERROR_NSUP_FMT,
    LineInUse = bindings::VI_ERROR_LINE_IN_USE,
    LineNreserved = bindings::VI_ERROR_LINE_NRESERVED,
    NsupMode = bindings::VI_ERROR_NSUP_MODE,
    SrqNoccurred = bindings::VI_ERROR_SRQ_NOCCURRED,
    InvSpace = bindings::VI_ERROR_INV_SPACE,
    InvOffset = bindings::VI_ERROR_INV_OFFSET,
    InvWidth = bindings::VI_ERROR_INV_WIDTH,
    NsupOffset = bindings::VI_ERROR_NSUP_OFFSET,
    NsupVarWidth = bindings::VI_ERROR_NSUP_VAR_WIDTH,
    WindowNmapped = bindings::VI_ERROR_WINDOW_NMAPPED,
    RespPending = bindings::VI_ERROR_RESP_PENDING,
    Nlisteners = bindings::VI_ERROR_NLISTENERS,
    Ncic = bindings::VI_ERROR_NCIC,
    NsysCntlr = bindings::VI_ERROR_NSYS_CNTLR,
    NsupOper = bindings::VI_ERROR_NSUP_OPER,
    IntrPending = bindings::VI_ERROR_INTR_PENDING,
    AsrlParity = bindings::VI_ERROR_ASRL_PARITY,
    AsrlFraming = bindings::VI_ERROR_ASRL_FRAMING,
    AsrlOverrun = bindings::VI_ERROR_ASRL_OVERRUN,
    TrigNmapped = bindings::VI_ERROR_TRIG_NMAPPED,
    NsupAlignOffset = bindings::VI_ERROR_NSUP_ALIGN_OFFSET,
    UserBuf = bindings::VI_ERROR_USER_BUF,
    RsrcBusy = bindings::VI_ERROR_RSRC_BUSY,
    NsupWidth = bindings::VI_ERROR_NSUP_WIDTH,
    InvParameter = bindings::VI_ERROR_INV_PARAMETER,
    InvProt = bindings::VI_ERROR_INV_PROT,
    InvSize = bindings::VI_ERROR_INV_SIZE,
    WindowMapped = bindings::VI_ERROR_WINDOW_MAPPED,
    NimplOper = bindings::VI_ERROR_NIMPL_OPER,
    InvLength = bindings::VI_ERROR_INV_LENGTH,
    InvMode = bindings::VI_ERROR_INV_MODE,
    SesnNlocked = bindings::VI_ERROR_SESN_NLOCKED,
    MemNshared = bindings::VI_ERROR_MEM_NSHARED,
    LibraryNfound = bindings::VI_ERROR_LIBRARY_NFOUND,
    NsupIntr = bindings::VI_ERROR_NSUP_INTR,
    InvLine = bindings::VI_ERROR_INV_LINE,
    FileAccess = bindings::VI_ERROR_FILE_ACCESS,
    FileIo = bindings::VI_ERROR_FILE_IO,
    NsupLine = bindings::VI_ERROR_NSUP_LINE,
    NsupMech = bindings::VI_ERROR_NSUP_MECH,
    IntfNumNconfig = bindings::VI_ERROR_INTF_NUM_NCONFIG,
    ConnLost = bindings::VI_ERROR_CONN_LOST,
    Npermission = bindings::VI_ERROR_NPERMISSION,
}
impl From<i32> for ErrorType {
    fn from(value: i32) -> Self {
        match value {
            x if x == ErrorType::SystemError as i32 => ErrorType::SystemError,
            x if x == ErrorType::InvObject as i32 => ErrorType::InvObject,
            x if x == ErrorType::RsrcLocked as i32 => ErrorType::RsrcLocked,
            x if x == ErrorType::InvExpr as i32 => ErrorType::InvExpr,
            x if x == ErrorType::RsrcNfound as i32 => ErrorType::RsrcNfound,
            x if x == ErrorType::InvRsrcName as i32 => ErrorType::InvRsrcName,
            x if x == ErrorType::InvAccMode as i32 => ErrorType::InvAccMode,
            x if x == ErrorType::Tmo as i32 => ErrorType::Tmo,
            x if x == ErrorType::ClosingFailed as i32 => ErrorType::ClosingFailed,
            x if x == ErrorType::InvDegree as i32 => ErrorType::InvDegree,
            x if x == ErrorType::InvJobId as i32 => ErrorType::InvJobId,
            x if x == ErrorType::NsupAttr as i32 => ErrorType::NsupAttr,
            x if x == ErrorType::NsupAttrState as i32 => ErrorType::NsupAttrState,
            x if x == ErrorType::AttrReadonly as i32 => ErrorType::AttrReadonly,
            x if x == ErrorType::InvLockType as i32 => ErrorType::InvLockType,
            x if x == ErrorType::InvAccessKey as i32 => ErrorType::InvAccessKey,
            x if x == ErrorType::InvEvent as i32 => ErrorType::InvEvent,
            x if x == ErrorType::InvMech as i32 => ErrorType::InvMech,
            x if x == ErrorType::HndlrNinstalled as i32 => ErrorType::HndlrNinstalled,
            x if x == ErrorType::InvHndlrRef as i32 => ErrorType::InvHndlrRef,
            x if x == ErrorType::InvContext as i32 => ErrorType::InvContext,
            x if x == ErrorType::Nenabled as i32 => ErrorType::Nenabled,
            x if x == ErrorType::Abort as i32 => ErrorType::Abort,
            x if x == ErrorType::RawWrProtViol as i32 => ErrorType::RawWrProtViol,
            x if x == ErrorType::RawRdProtViol as i32 => ErrorType::RawRdProtViol,
            x if x == ErrorType::OutpProtViol as i32 => ErrorType::OutpProtViol,
            x if x == ErrorType::InpProtViol as i32 => ErrorType::InpProtViol,
            x if x == ErrorType::Berr as i32 => ErrorType::Berr,
            x if x == ErrorType::InProgress as i32 => ErrorType::InProgress,
            x if x == ErrorType::InvSetup as i32 => ErrorType::InvSetup,
            x if x == ErrorType::QueueError as i32 => ErrorType::QueueError,
            x if x == ErrorType::Alloc as i32 => ErrorType::Alloc,
            x if x == ErrorType::InvMask as i32 => ErrorType::InvMask,
            x if x == ErrorType::Io as i32 => ErrorType::Io,
            x if x == ErrorType::InvFmt as i32 => ErrorType::InvFmt,
            x if x == ErrorType::NsupFmt as i32 => ErrorType::NsupFmt,
            x if x == ErrorType::LineInUse as i32 => ErrorType::LineInUse,
            x if x == ErrorType::LineNreserved as i32 => ErrorType::LineNreserved,
            x if x == ErrorType::NsupMode as i32 => ErrorType::NsupMode,
            x if x == ErrorType::SrqNoccurred as i32 => ErrorType::SrqNoccurred,
            x if x == ErrorType::InvSpace as i32 => ErrorType::InvSpace,
            x if x == ErrorType::InvOffset as i32 => ErrorType::InvOffset,
            x if x == ErrorType::InvWidth as i32 => ErrorType::InvWidth,
            x if x == ErrorType::NsupOffset as i32 => ErrorType::NsupOffset,
            x if x == ErrorType::NsupVarWidth as i32 => ErrorType::NsupVarWidth,
            x if x == ErrorType::WindowNmapped as i32 => ErrorType::WindowNmapped,
            x if x == ErrorType::RespPending as i32 => ErrorType::RespPending,
            x if x == ErrorType::Nlisteners as i32 => ErrorType::Nlisteners,
            x if x == ErrorType::Ncic as i32 => ErrorType::Ncic,
            x if x == ErrorType::NsysCntlr as i32 => ErrorType::NsysCntlr,
            x if x == ErrorType::NsupOper as i32 => ErrorType::NsupOper,
            x if x == ErrorType::IntrPending as i32 => ErrorType::IntrPending,
            x if x == ErrorType::AsrlParity as i32 => ErrorType::AsrlParity,
            x if x == ErrorType::AsrlFraming as i32 => ErrorType::AsrlFraming,
            x if x == ErrorType::AsrlOverrun as i32 => ErrorType::AsrlOverrun,
            x if x == ErrorType::TrigNmapped as i32 => ErrorType::TrigNmapped,
            x if x == ErrorType::NsupAlignOffset as i32 => ErrorType::NsupAlignOffset,
            x if x == ErrorType::UserBuf as i32 => ErrorType::UserBuf,
            x if x == ErrorType::RsrcBusy as i32 => ErrorType::RsrcBusy,
            x if x == ErrorType::NsupWidth as i32 => ErrorType::NsupWidth,
            x if x == ErrorType::InvParameter as i32 => ErrorType::InvParameter,
            x if x == ErrorType::InvProt as i32 => ErrorType::InvProt,
            x if x == ErrorType::InvSize as i32 => ErrorType::InvSize,
            x if x == ErrorType::WindowMapped as i32 => ErrorType::WindowMapped,
            x if x == ErrorType::NimplOper as i32 => ErrorType::NimplOper,
            x if x == ErrorType::InvLength as i32 => ErrorType::InvLength,
            x if x == ErrorType::InvMode as i32 => ErrorType::InvMode,
            x if x == ErrorType::SesnNlocked as i32 => ErrorType::SesnNlocked,
            x if x == ErrorType::MemNshared as i32 => ErrorType::MemNshared,
            x if x == ErrorType::LibraryNfound as i32 => ErrorType::LibraryNfound,
            x if x == ErrorType::NsupIntr as i32 => ErrorType::NsupIntr,
            x if x == ErrorType::InvLine as i32 => ErrorType::InvLine,
            x if x == ErrorType::FileAccess as i32 => ErrorType::FileAccess,
            x if x == ErrorType::FileIo as i32 => ErrorType::FileIo,
            x if x == ErrorType::NsupLine as i32 => ErrorType::NsupLine,
            x if x == ErrorType::NsupMech as i32 => ErrorType::NsupMech,
            x if x == ErrorType::IntfNumNconfig as i32 => ErrorType::IntfNumNconfig,
            x if x == ErrorType::ConnLost as i32 => ErrorType::ConnLost,
            x if x == ErrorType::Npermission as i32 => ErrorType::Npermission,

            _ => ErrorType::SystemError,
        }
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorType::SystemError => write!(f, "System error"),
            ErrorType::InvObject => write!(f, "Invalid object"),
            ErrorType::RsrcLocked => write!(f, "Resource locked"),
            ErrorType::InvExpr => write!(f, "Invalid expression"),
            ErrorType::RsrcNfound => write!(f, "Resource not found"),
            ErrorType::InvRsrcName => write!(f, "Invalid resource name"),
            ErrorType::InvAccMode => write!(f, "Invalid access mode"),
            ErrorType::Tmo => write!(f, "Timeout"),
            ErrorType::ClosingFailed => write!(f, "Closing failed"),
            ErrorType::InvDegree => write!(f, "Invalid degree"),
            ErrorType::InvJobId => write!(f, "Invalid job ID"),
            ErrorType::NsupAttr => write!(f, "Attribute not supported"),
            ErrorType::NsupAttrState => write!(f, "Attribute state not supported"),
            ErrorType::AttrReadonly => write!(f, "Attribute is read-only"),
            ErrorType::InvLockType => write!(f, "Invalid lock type"),
            ErrorType::InvAccessKey => write!(f, "Invalid access key"),
            ErrorType::InvEvent => write!(f, "Invalid event"),
            ErrorType::InvMech => write!(f, "Invalid mechanism"),
            ErrorType::HndlrNinstalled => write!(f, "Handler not installed"),
            ErrorType::InvHndlrRef => write!(f, "Invalid handler reference"),
            ErrorType::InvContext => write!(f, "Invalid context"),
            ErrorType::Nenabled => write!(f, "Not enabled"),
            ErrorType::Abort => write!(f, "Abort"),
            ErrorType::RawWrProtViol => write!(f, "Raw write protection violation"),
            ErrorType::RawRdProtViol => write!(f, "Raw read protection violation"),
            ErrorType::OutpProtViol => write!(f, "Output protection violation"),
            ErrorType::InpProtViol => write!(f, "Input protection violation"),
            ErrorType::Berr => write!(f, "Bus error"),
            ErrorType::InProgress => write!(f, "Operation in progress"),
            ErrorType::InvSetup => write!(f, "Invalid setup"),
            ErrorType::QueueError => write!(f, "Queue error"),
            ErrorType::Alloc => write!(f, "Allocation error"),
            ErrorType::InvMask => write!(f, "Invalid mask"),
            ErrorType::Io => write!(f, "I/O error"),
            ErrorType::InvFmt => write!(f, "Invalid format"),
            ErrorType::NsupFmt => write!(f, "Format not supported"),
            ErrorType::LineInUse => write!(f, "Line in use"),
            ErrorType::LineNreserved => write!(f, "Line not reserved"),
            ErrorType::NsupMode => write!(f, "Mode not supported"),
            ErrorType::SrqNoccurred => write!(f, "SRQ not occurred"),
            ErrorType::InvSpace => write!(f, "Invalid space"),
            ErrorType::InvOffset => write!(f, "Invalid offset"),
            ErrorType::InvWidth => write!(f, "Invalid width"),
            ErrorType::NsupOffset => write!(f, "Offset not supported"),
            ErrorType::NsupVarWidth => write!(f, "Variable width not supported"),
            ErrorType::WindowNmapped => write!(f, "Window not mapped"),
            ErrorType::RespPending => write!(f, "Response pending"),
            ErrorType::Nlisteners => write!(f, "Number of listeners"),
            ErrorType::Ncic => write!(f, "CIC not supported"),
            ErrorType::NsysCntlr => write!(f, "System controller not supported"),
            ErrorType::NsupOper => write!(f, "Operation not supported"),
            ErrorType::IntrPending => write!(f, "Interrupt pending"),
            ErrorType::AsrlParity => write!(f, "Asynchronous parity error"),
            ErrorType::AsrlFraming => write!(f, "Asynchronous framing error"),
            ErrorType::AsrlOverrun => write!(f, "Asynchronous overrun error"),
            ErrorType::TrigNmapped => write!(f, "Trigger not mapped"),
            ErrorType::NsupAlignOffset => write!(f, "Alignment offset not supported"),
            ErrorType::UserBuf => write!(f, "User buffer"),
            ErrorType::RsrcBusy => write!(f, "Resource busy"),
            ErrorType::NsupWidth => write!(f, "Width not supported"),
            ErrorType::InvParameter => write!(f, "Invalid parameter"),
            ErrorType::InvProt => write!(f, "Invalid protection"),
            ErrorType::InvSize => write!(f, "Invalid size"),
            ErrorType::WindowMapped => write!(f, "Window mapped"),
            ErrorType::NimplOper => write!(f, "Operation not implemented"),
            ErrorType::InvLength => write!(f, "Invalid length"),
            ErrorType::InvMode => write!(f, "Invalid mode"),
            ErrorType::SesnNlocked => write!(f, "Session not locked"),
            ErrorType::MemNshared => write!(f, "Memory not shared"),
            ErrorType::LibraryNfound => write!(f, "Library not found"),
            ErrorType::NsupIntr => write!(f, "Interrupt not supported"),
            ErrorType::InvLine => write!(f, "Invalid line"),
            ErrorType::FileAccess => write!(f, "File access error"),
            ErrorType::FileIo => write!(f, "File I/O error"),
            ErrorType::NsupLine => write!(f, "Line not supported"),
            ErrorType::NsupMech => write!(f, "Mechanism not supported"),
            ErrorType::IntfNumNconfig => write!(f, "Interface number not configured"),
            ErrorType::ConnLost => write!(f, "Connection lost"),
            ErrorType::Npermission => write!(f, "No permission"),
        }
    }
}
