//! Specifies the attributes that can be set or read on a session.
//!
//! Note: Not all attributes are implemented yet.
//! For those Not implementing AsViReadable/AsViWritable yet, use
//! `Session::get_attribute_raw` and `Session::set_attribute_raw` to get/set the raw value of the attribute.
#![expect(
    clippy::cast_possible_truncation,
    reason = "Needed for compatibility with the VISA library"
)]
#![expect(
    clippy::cast_possible_wrap,
    reason = "Needed for compatibility with the VISA library"
)]
#![expect(
    clippy::cast_sign_loss,
    reason = "Needed for compatibility with the VISA library"
)]

use crate::bindings;

/// A type of atribute that can be read from a device
pub trait AsViReadable {
    /// The VISA attribute type for this attribute
    const VI_ATTR: u32;

    /// The raw type of the attribute expected by VISA
    type RawValue;

    /// The type of the attribute value after conversion
    type Value;

    /// Convert a raw VISA attribute value to the attribute value
    fn from_vi(value: Self::RawValue) -> Option<Self>
    where
        Self: Sized;

    /// Get a reference to the attribute value
    fn value(&self) -> &Self::Value;

    /// Convert the attribute value to a raw VISA attribute value
    fn into_value(self) -> Self::Value;
}

/// A type of attribute that can be written to a device
pub trait AsViWritable {
    /// Convert the attribute value to a raw VISA attribute state
    fn as_vi(&self) -> bindings::ViAttrState;
}

/// Macro simplifying the implementation of AsViReadable/AsViWritable for an attribute
macro_rules! impl_attr {
    (
        $($docs:literal)*
        $name:ident(ReadOnlyI16)
    ) => {
        impl_attr! {
            $($docs)*
            $name(i16, ReadOnlyI16), from = |value| {
                Some(Self(value))
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(ReadOnlyBool)
    ) => {
        impl_attr! {
            $($docs)*
            $name(bindings::ViBoolean, bool),
            from = |value| {
                match value {
                    x if x == bindings::VI_TRUE as bindings::ViBoolean => Some(Self(true)),
                    x if x == bindings::VI_FALSE as bindings::ViBoolean => Some(Self(false)),
                    _ => None,
                }
            }
            into = |value| {
                if value {
                    bindings::VI_TRUE as bindings::ViAttrState
                } else {
                    bindings::VI_FALSE as bindings::ViAttrState
                }
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(ReadOnlyU32)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u32, ReadOnlyU32), from = |value| {
                Some(Self(value))
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(ReadOnlyU16)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u16, ReadOnlyU16), from = |value| {
                Some(Self(value))
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(u32)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u32, u32), from = |value| {
                Some(Self(value))
            }
            into = |value| {
                value as bindings::ViAttrState
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(u16)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u16, u16), from = |value| {
                Some(Self(value))
            }
            into = |value| {
                bindings::ViAttrState::from(value)
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(u8)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u8, u8), from = |value| {
                Some(Self(value))
            }
            into = |value| {
                bindings::ViAttrState::from(value)
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(bool)
    ) => {
        impl_attr! {
            $($docs)*
            $name(bindings::ViBoolean, bool), from = |value| {
                match value {
                    x if x == bindings::VI_TRUE as bindings::ViBoolean => Some(Self(true)),
                    x if x == bindings::VI_FALSE as bindings::ViBoolean => Some(Self(false)),
                    _ => None,
                }
            }
            into = |value| {
                if value {
                    bindings::VI_TRUE as bindings::ViAttrState
                } else {
                    bindings::VI_FALSE as bindings::ViAttrState
                }
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(String)
    ) => {
        impl_attr! {
            $($docs)*
            $name([std::ffi::c_char; 256], String), from = |value| {
                let value = unsafe { std::ffi::CStr::from_ptr(value.as_ptr()) };
                Some(Self(value.to_string_lossy().into_owned()))
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident($raw:ty, $value:ty), from = |$src_id:ident| $from:block $(into = |$dst_id:ident|$into:block)?
    ) => {
        $(#[doc = $docs])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(pub $value);
        impl $name {
            /// Create a new instance of the attribute
            #[must_use]
            pub fn new(value: $value) -> Self {
                Self(value)
            }
        }
        impl AsViReadable for $name {
            const VI_ATTR: u32 = AttributeType::$name as u32;
            type RawValue = $raw;
            type Value = $value;

            fn from_vi($src_id: Self::RawValue) -> Option<Self> $from

            fn value(&self) -> &Self::Value {
                &self.0
            }

            fn into_value(self) -> Self::Value {
                self.0
            }
        }

        $(
            impl AsViWritable for $name {
                fn as_vi(&self) -> bindings::ViAttrState {
                    let $dst_id = self.0;
                    $into
                }
            }
        )?
    };
}

/// Attribute states
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Asserted = bindings::VI_STATE_ASSERTED as i32,
    Unasserted = bindings::VI_STATE_UNASSERTED as i32,
    Unknown = bindings::VI_STATE_UNKNOWN,
}

impl_attr!(
    "VI_ATTR_RSRC_CLASS specifies the resource class (for example, 'INSTR') as defined by the canonical resource name."
    RsrcClass(String)
);

impl_attr!(
    "VI_ATTR_RSRC_NAME is the unique identifier for a resource."
    "Refer to VISA Resource Syntax and Examples for the syntax of resource strings and examples."
    RsrcName(String)
);

impl_attr!(
    "VI_ATTR_RSRC_IMPL_VERSION is the resource version that uniquely identifies each of the different revisions or implementations of a resource."
    "This attribute value is defined by the individual manufacturer and increments with each new revision."
    "The format of the value has the upper 12 bits as the major number of the version,"
    "the next lower 12 bits as the minor number of the version,"
    "and the lowest 8 bits as the sub-minor number of the version."
    RsrcImplVersion(u32)
);

impl_attr!(
    "VI_ATTR_RSRC_LOCK_STATE indicates the current locking state of the resource."
    "The resource can be unlocked, locked with an exclusive lock, or locked with a shared lock."
    RsrcLockState(u32, AccessMode),

    from = |value| {
        let value = match value {
            x if x == AccessMode::NoLock as u32 => AccessMode::NoLock,
            x if x == AccessMode::ExclusiveLock as u32 => AccessMode::ExclusiveLock,
            x if x == AccessMode::SharedLock as u32 => AccessMode::SharedLock,
            x if x == AccessMode::LoadConfig as u32 => AccessMode::LoadConfig,
            _ => return None,
        };
        Some(Self(value))
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "VI_ATTR_MAX_QUEUE_LENGTH specifies the maximum number of events that can be queued at any time on the given session. Events that occur after the queue has become full will be discarded."
    "VI_ATTR_MAX_QUEUE_LENGTH is a Read/Write attribute until the first time viEnableEvent() is called on a session. Thereafter, this attribute is Read Only."
    MaxQueueLength(u32)
);

impl_attr!(
    "VI_ATTR_FDC_CHNL determines which Fast Data Channel (FDC) will be used to transfer the buffer."
    FdcChnl(u16, u16),
    from = |value| {
        match value {
            0..=7 => Some(Self(value)),
            _ => None,
        }
    }
    into = |value| {
        bindings::ViAttrState::from(value)
    }
);

/// Fast Data Channel (FDC) mode
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FdcModeType {
    Normal = bindings::VI_FDC_NORMAL,
    Stream = bindings::VI_FDC_STREAM,
}
impl_attr!(
    "VI_ATTR_FDC_MODE specifies which Fast Data Channel (FDC) mode to use (either normal or stream mode)."
    FdcMode(u32, FdcModeType),
    from = |value| {
        match value {
            bindings::VI_FDC_NORMAL => Some(Self(FdcModeType::Normal)),
            bindings::VI_FDC_STREAM => Some(Self(FdcModeType::Stream)),
            _ => None,
        }
    }
    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "Setting VI_ATTR_FDC_GEN_SIGNAL_EN to VI_TRUE lets the servant send a signal"
    "when control of the FDC channel is passed back to the commander."
    "This action frees the commander from having to poll the FDC header while engaging in an FDC transfer."
    FdcGenSignalEn(bool)
);

impl_attr!(
    "Setting VI_ATTR_FDC_USE_PAIR to VI_TRUE specifies to use a channel pair for transferring data. Otherwise, only one channel will be used."
    FdcUsePair(bool)
);

impl_attr!(
    "VI_ATTR_SEND_END_EN specifies whether to assert END during the transfer of the last byte of the buffer."
    "VI_ATTR_SEND_END_EN is relevant only in viWrite and related operations."
    "On Serial INSTR sessions, if this attribute is set to VI_FALSE, the write will transmit the exact contents of the user buffer, without modifying it and without appending anything to the data being written."
    "If this attribute is set to VI_TRUE, VISA will perform the behavior described in VI_ATTR_ASRL_END_OUT."
    "On GPIB, VXI, TCP/IP INSTR, and USB INSTR sessions, if this attribute is set to VI_TRUE, VISA will include the 488.2 defined 'end of message' terminator."
    SendEndEn(bool)
);

impl_attr!(
    "VI_ATTR_TERMCHAR is the termination character. When the termination character is read and VI_ATTR_TERMCHAR_EN is enabled during a read operation, the read operation terminates."
    "For a Serial INSTR session, VI_ATTR_TERMCHAR is Read/Write when the corresponding session is not enabled to receive VI_EVENT_ASRL_TERMCHAR events."
    "When the session is enabled to receive VI_EVENT_ASRL_TERMCHAR events, the attribute VI_ATTR_TERMCHAR is Read Only."
    "For all other session types, the attribute VI_ATTR_TERMCHAR is always Read/Write"
    TermChar(u8)
);

impl_attr!(
    "VI_ATTR_TMO_VALUE specifies the minimum timeout value to use (in milliseconds) when accessing the device associated with the given session."
    "A timeout value of VI_TMO_IMMEDIATE means that operations should never wait for the device to respond."
    "A timeout value of VI_TMO_INFINITE disables the timeout mechanism."
    "Notice that the actual timeout value used by the driver may be higher than the requested one."
    "The actual timeout value is returned when this attribute is retrieved via viGetAttribute()."
    TmoValue(u32, std::time::Duration),

    from = |value| {
        Some(Self(std::time::Duration::from_millis(u64::from(value))))
    }

    into = |value| {
        value.as_millis() as bindings::ViAttrState
    }
);

/// The type of the interface
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Gpib = bindings::VI_INTF_GPIB,
    Vxi = bindings::VI_INTF_VXI,
    GpibVxi = bindings::VI_INTF_GPIB_VXI,
    Asrl = bindings::VI_INTF_ASRL,
    Pxi = bindings::VI_INTF_PXI,
    Tcpip = bindings::VI_INTF_TCPIP,
    Usb = bindings::VI_INTF_USB,
}
impl_attr!(
    "VI_ATTR_INTF_TYPE specifies the interface type of the given session."
    IntfType(u16, InterfaceType),
    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == bindings::VI_INTF_GPIB => Some(Self(InterfaceType::Gpib)),
            x if x == bindings::VI_INTF_VXI => Some(Self(InterfaceType::Vxi)),
            x if x == bindings::VI_INTF_GPIB_VXI => Some(Self(InterfaceType::GpibVxi)),
            x if x == bindings::VI_INTF_ASRL => Some(Self(InterfaceType::Asrl)),
            x if x == bindings::VI_INTF_PXI => Some(Self(InterfaceType::Pxi)),
            x if x == bindings::VI_INTF_TCPIP => Some(Self(InterfaceType::Tcpip)),
            x if x == bindings::VI_INTF_USB => Some(Self(InterfaceType::Usb)),
            _ => None,
        }
    }
);

impl_attr!(
    "VI_ATTR_INTF_NUM specifies the board number for the given interface."
    IntfNum(ReadOnlyU16)
);

impl_attr!(
    "VI_ATTR_GPIB_READDR_EN specifies whether to use repeat addressing before each read or write operation."
    GpibReaddrEn(bool)
);

impl_attr!(
    "VI_ATTR_IO_PROT specifies which protocol to use."
    "In VXI, you can choose normal word serial or fast data channel (FDC). "
    "In GPIB, you can choose normal or high-speed (HS-488) transfers."
    "In serial, TCPIP, or USB RAW, you can choose normal transfers or 488.2-defined strings."
    "In USB INSTR, you can choose normal or vendor-specific transfers."
    IoProt(u16)
);

impl_attr!(
    "This attribute specifies whether I/O accesses should use DMA (VI_TRUE) or Programmed I/O (VI_FALSE)."
    "In some implementations, this attribute may have global effects even though it is documented to be a local attribute."
    "Since this affects performance and not functionality, that behavior is acceptable."
    DmaAllowEn(bool)
);

impl_attr!(
    "VI_ATTR_ASRL_BAUD is the baud rate of the interface."
    "It is represented as an unsigned 32-bit integer so that any baud rate can be used, but it usually requires a commonly used rate such as: "
    "300, 1200, 2400, or 9600 baud."
    AsrlBaud(u32)
);

impl_attr!(
    "VI_ATTR_ASRL_DATA_BITS is the number of data bits contained in each frame (from 5 to 8)."
    "he data bits for each frame are located in the low-order bits of every byte stored in memory."
    AsrlDataBits(u16)
);

/// The type of the parity used with every frame transmitted and received.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsrlParityType {
    None = bindings::VI_ASRL_PAR_NONE,
    Odd = bindings::VI_ASRL_PAR_ODD,
    Even = bindings::VI_ASRL_PAR_EVEN,
    Mark = bindings::VI_ASRL_PAR_MARK,
    Space = bindings::VI_ASRL_PAR_SPACE,
}

impl_attr!(
    "VI_ATTR_ASRL_PARITY is the parity used with every frame transmitted and received."
    AsrlParity(u16, AsrlParityType),
    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AsrlParityType::None as u32 => Some(Self(AsrlParityType::None)),
            x if x == AsrlParityType::Odd as u32 => Some(Self(AsrlParityType::Odd)),
            x if x == AsrlParityType::Even as u32 => Some(Self(AsrlParityType::Even)),
            x if x == AsrlParityType::Mark as u32 => Some(Self(AsrlParityType::Mark)),
            x if x == AsrlParityType::Space as u32 => Some(Self(AsrlParityType::Space)),
            _ => None,
        }
    }
    into = |value| {
        value as bindings::ViAttrState
    }
);

/// The number of stop bits used to indicate the end of a frame.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsrlStopBitsType {
    One = bindings::VI_ASRL_STOP_ONE,
    One5 = bindings::VI_ASRL_STOP_ONE5,
    Two = bindings::VI_ASRL_STOP_TWO,
}

impl_attr!(
    "VI_ATTR_ASRL_STOP_BITS is the number of stop bits used to indicate the end of a frame."
    "The value VI_ASRL_STOP_ONE5 indicates one-and-one-half (1.5) stop bits."
    AsrlStopBits(u16, AsrlStopBitsType),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AsrlStopBitsType::One as u32 => Some(Self(AsrlStopBitsType::One)),
            x if x == AsrlStopBitsType::One5 as u32 => Some(Self(AsrlStopBitsType::One5)),
            x if x == AsrlStopBitsType::Two as u32 => Some(Self(AsrlStopBitsType::Two)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

/// The type of flow control used by the transfer mechanism.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsrlFlowCntrlType {
    /// the transfer mechanism does not use flow control, and buffers on both sides of the connection are assumed to be large enough to hold all data transferred.
    None = bindings::VI_ASRL_FLOW_NONE,

    /// the transfer mechanism uses the XON and XOFF characters to perform flow control.
    /// The transfer mechanism controls input flow by sending XOFF when the low-level I/O receive buffer is nearly full, and it controls the output flow by suspending transmission when XOFF is received.
    XonXoff = bindings::VI_ASRL_FLOW_XON_XOFF,

    /// the transfer mechanism uses the RTS output signal and the CTS input signal to perform flow control.
    /// The transfer mechanism controls input flow by unasserting the RTS signal when the low-level I/O receive buffer is nearly full, and it controls output flow by suspending the transmission when the CTS signal is unasserted.
    RtsCts = bindings::VI_ASRL_FLOW_RTS_CTS,

    ///the transfer mechanism uses the DTR output signal and the DSR input signal to perform flow control.
    /// The transfer mechanism controls input flow by unasserting the DTR signal when the low-level I/O receive buffer is nearly full, and it controls output flow by suspending the transmission when the DSR signal is unasserted.
    DtrDsr = bindings::VI_ASRL_FLOW_DTR_DSR,
}

impl_attr!(
    "VI_ATTR_ASRL_FLOW_CNTRL indicates the type of flow control used by the transfer mechanism."
    AsrlFlowCntrl(u16, AsrlFlowCntrlType),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AsrlFlowCntrlType::None as u32 => Some(Self(AsrlFlowCntrlType::None)),
            x if x == AsrlFlowCntrlType::XonXoff as u32 => Some(Self(AsrlFlowCntrlType::XonXoff)),
            x if x == AsrlFlowCntrlType::RtsCts as u32 => Some(Self(AsrlFlowCntrlType::RtsCts)),
            x if x == AsrlFlowCntrlType::DtrDsr as u32 => Some(Self(AsrlFlowCntrlType::DtrDsr)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "VI_ATTR_RD_BUF_OPER_MODE specifies the operational mode of the formatted I/O read buffer."
    "When the operational mode is set to VI_FLUSH_DISABLE (default), the buffer is flushed only on explicit calls to viFlush()."
    "If the operational mode is set to VI_FLUSH_ON_ACCESS, the read buffer is flushed every time a viScanf() (or related) operation completes."
    RdBufOperMode(u16)
);

impl_attr!(
    "This is the current size of the formatted I/O input buffer for this session. The user can modify this value by calling `session::set_read_buffer`"
    RdBufSize(ReadOnlyU32)
);

/// The operational mode of the formatted I/O write buffer.
#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum WrBufOperModeType {
    #[default]
    /// the buffer is flushed when an END indicator is written to the buffer, or when the buffer fills up.
    OnAccess = bindings::VI_FLUSH_ON_ACCESS,

    /// the write buffer is flushed under the same conditions, and also every time a `viPrintf()` (or related) operation completes.
    OnFull = bindings::VI_FLUSH_WHEN_FULL,
}

impl_attr!(
    "VI_ATTR_WR_BUF_OPER_MODE specifies the operational mode of the formatted I/O write buffer."
    WrBufOperMode(u16, WrBufOperModeType),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == WrBufOperModeType::OnAccess as u32 => Some(Self(WrBufOperModeType::OnAccess)),
            x if x == WrBufOperModeType::OnFull as u32 => Some(Self(WrBufOperModeType::OnFull)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "This is the current size of the formatted I/O output buffer for this session. The user can modify this value by calling `session::set_write_buffer`"
    WrBufSize(ReadOnlyU32)
);
/*
impl_attr!(
    ""
    SuppressEndEn()
);

impl_attr!(
    ""
    TermCharEn()
);

impl_attr!(
    ""
    DestAccessPriv()
);

impl_attr!(
    ""
    DestByteOrder()
);

impl_attr!(
    ""
    SrcAccessPriv()
);

impl_attr!(
    ""
    SrcByteOrder()
);

impl_attr!(
    ""
    SrcIncrement()
);

impl_attr!(
    ""
    DestIncrement()
);

impl_attr!(
    ""
    WinAccessPriv()
);

impl_attr!(
    ""
    WinByteOrder()
);

impl_attr!(
    ""
    CmdrLa()
);

impl_attr!(
    ""
    MainframeLa()
);

impl_attr!(
    ""
    ManfName()
);
*/
impl_attr!(
    "This string attribute is the model name of the device."
    ModelName(String)
);
/*
impl_attr!(
    ""
    AsrlAvailNum()
);

impl_attr!(
    ""
    AsrlCtsState()
);

impl_attr!(
    ""
    AsrlDcdState()
);

impl_attr!(
    ""
    AsrlDsrState()
);

impl_attr!(
    ""
    AsrlDtrState()
);

impl_attr!(
    ""
    AsrlEndIn()
);

impl_attr!(
    ""
    AsrlEndOut()
);

impl_attr!(
    ""
    AsrlReplaceChar()
);

impl_attr!(
    ""
    AsrlRiState()
);

impl_attr!(
    ""
    AsrlRtsState()
);

impl_attr!(
    ""
    AsrlXonChar()
);

impl_attr!(
    ""
    AsrlXoffChar()
);

impl_attr!(
    ""
    WinAccess()
);

impl_attr!(
    ""
    RmSession()
);

impl_attr!(
    ""
    VxiLa()
);

impl_attr!(
    ""
    ManfId()
);

impl_attr!(
    ""
    MemSize32()
);

impl_attr!(
    ""
    MemSpace()
);

impl_attr!(
    ""
    ModelCode()
);

impl_attr!(
    ""
    Slot()
);

impl_attr!(
    ""
    IntfInstName()
);

impl_attr!(
    ""
    ImmediateServ()
);

impl_attr!(
    ""
    IntfParentNum()
);

impl_attr!(
    ""
    RsrcSpecVersion()
);

impl_attr!(
    ""
    RsrcManfName()
);

impl_attr!(
    ""
    RsrcManfId()
);

impl_attr!(
    ""
    TrigId()
);

*/

impl_attr!(
    "VI_ATTR_GPIB_PRIMARY_ADDR specifies the primary address of the GPIB device used by the given session."
    "For the GPIB INTFC Resource, this attribute is Read-Write."
    "Valid values are 0 to 30."
    GpibPrimaryAddr(u16)
);

impl_attr!(
    "VI_ATTR_GPIB_PRIMARY_ADDR specifies the secondary address of the GPIB device used by the given session."
    "For the GPIB INTFC Resource, this attribute is Read-Write."
    "Valid values are 0 to 30, or 0x"
    GpibSecondaryAddr(u16, Option<u16>),

    from = |value| {
        const VI_NO_SEC_ADDR: u16 = bindings::VI_NO_SEC_ADDR as u16;
        match value {
            0..=30 => Some(Self(Some(value))),
            VI_NO_SEC_ADDR => Some(Self(None)),
            _ => None,
        }
    }

    into = |value| {
        let value = value.unwrap_or(bindings::VI_NO_SEC_ADDR as u16);
        bindings::ViAttrState::from(value)
    }
);

impl_attr!(
    "This attribute shows the current state of the GPIB ATN (ATtentioN) interface line."
    GpibAtnState(i16, State),
    from = |value| {
        let value = value as i16;
        match value {
            x if x == State::Asserted as i16 => Some(Self(State::Asserted)),
            x if x == State::Unasserted as i16 => Some(Self(State::Unasserted)),
            x if x == State::Unknown as i16 => Some(Self(State::Unknown)),
            _ => None,
        }
    }
);

/// The type of the GPIB address state
#[allow(missing_docs)]
#[repr(i16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GpibAddrStateType {
    Unaddressed = bindings::VI_GPIB_UNADDRESSED as i16,
    Talker = bindings::VI_GPIB_TALKER as i16,
    Listener = bindings::VI_GPIB_LISTENER as i16,
}

impl_attr!(
    "This attribute shows whether the specified GPIB interface is currently addressed to talk or listen, or is not addressed."
    GpibAddrState(i16, GpibAddrStateType),
    from = |value| {
        let value = value as i16;
        match value {
            x if x == GpibAddrStateType::Unaddressed as i16 => Some(Self(GpibAddrStateType::Unaddressed)),
            x if x == GpibAddrStateType::Talker as i16 => Some(Self(GpibAddrStateType::Talker)),
            x if x == GpibAddrStateType::Listener as i16 => Some(Self(GpibAddrStateType::Listener)),
            _ => None,
        }
    }
);

impl_attr!(
    "This attribute shows whether the specified GPIB interface is currently CIC (Controller In Charge)."
    GpibCicState(ReadOnlyBool)
);

impl_attr!(
    "This attribute shows the current state of the GPIB NDAC (Not Data ACcepted) interface line."
    GpibNdacState(i16, State),
    from = |value| {
        let value = value as i16;
        match value {
            x if x == State::Asserted as i16 => Some(Self(State::Asserted)),
            x if x == State::Unasserted as i16 => Some(Self(State::Unasserted)),
            x if x == State::Unknown as i16 => Some(Self(State::Unknown)),
            _ => None,
        }
    }
);

impl_attr!(
    "This attribute shows the current state of the GPIB SRQ (Service ReQuest) interface line."
    GpibSrqState(i16, State),
    from = |value| {
        let value = value as i16;
        match value {
            x if x == State::Asserted as i16 => Some(Self(State::Asserted)),
            x if x == State::Unasserted as i16 => Some(Self(State::Unasserted)),
            x if x == State::Unknown as i16 => Some(Self(State::Unknown)),
            _ => None,
        }
    }
);

impl_attr!(
    "This attribute shows whether the specified GPIB interface is currently the system controller."
    "In some implementations, this attribute may be modified only through a configuration utility."
    "On these systems this attribute is read-only (RO)."
    GpibSysCntrlState(bool)
);

/// This attribute specifies the total number of meters of GPIB cable used in the specified GPIB interface.
/// Valid values are 0 to 15 meters.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GpibHs488CableLength {
    NotImplemented,
    Disabled,
    Meters(u8),
}

impl_attr!(
    "This attribute specifies the total number of meters of GPIB cable used in the specified GPIB interface."
    "Valid values are 0 to 15 meters."
    GpibHs488CblLen(i16, GpibHs488CableLength),
    from = |value| {
        let value = value as i16;
        match value {
            x if x == bindings::VI_GPIB_HS488_NIMPL as i16 => Some(Self(GpibHs488CableLength::NotImplemented)),
            x if x == bindings::VI_GPIB_HS488_DISABLED as i16 => Some(Self(GpibHs488CableLength::Disabled)),
            1..=15 => Some(Self(GpibHs488CableLength::Meters(value as u8))),
            _ => None,
        }
    }

    into = |value| {
        (match value {
            GpibHs488CableLength::NotImplemented => bindings::VI_GPIB_HS488_NIMPL as i16,
            GpibHs488CableLength::Disabled => bindings::VI_GPIB_HS488_DISABLED as i16,

            GpibHs488CableLength::Meters(0) => 1,
            GpibHs488CableLength::Meters(x) if x > 15 => 15,
            GpibHs488CableLength::Meters(x) => i16::from(x),
        }) as bindings::ViAttrState
    }

);

impl_attr!(
    "VI_ATTR_GPIB_REN_STATE returns the current state of the GPIB REN (Remote ENable) interface line."
    GpibRenState(i32, State),

    from = |value| {
        let value = value as i32;
        match value {
            x if x == State::Asserted as i32 => Some(Self(State::Asserted)),
            x if x == State::Unasserted as i32 => Some(Self(State::Unasserted)),
            x if x == State::Unknown as i32 => Some(Self(State::Unknown)),
            _ => None,
        }
    }
);

impl_attr!(
    "VI_ATTR_GPIB_UNADDR_EN specifies whether to unaddress the device (UNT and UNL) after each read or write operation."
    GpibUnaddrEn(bool)
);

impl_attr!(
    "This attribute specifies the 488-style status byte of the local controller or device associated with this session."
    "If this attribute is written and bit 6 (40h) is set, this device or controller will assert a service request (SRQ) if it is defined for this interface."
    DevStatusByte(u8)
);

impl_attr!(
    "This attribute specifies whether viReadToFile() will overwrite (truncate) or append when opening a file."
    FileAppendEn(bool)
);

impl_attr!(
    "This attribute shows which VXI trigger lines this implementation supports."
    "This is a bit vector with bits 0-9 corresponding to VI_TRIG_TTL0 through VI_TRIG_ECL1."
    VxiTrigSupport(ReadOnlyU32)
);

impl_attr!(
    "This attribute shows the current state of the VXI/VME interrupt lines. This is a bit vector with bits 0-6 corresponding to interrupt lines 1-7."
    VxiVmeIntrStatus(ReadOnlyU16)
);

/// VXI device class type
#[allow(missing_docs)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VxiDevClassType {
    Memory = bindings::VI_VXI_CLASS_MEMORY as u16,
    Extended = bindings::VI_VXI_CLASS_EXTENDED as u16,
    Message = bindings::VI_VXI_CLASS_MESSAGE as u16,
    Register = bindings::VI_VXI_CLASS_REGISTER as u16,
    Other = bindings::VI_VXI_CLASS_OTHER as u16,
}

impl_attr!(
    "This attribute represents the VXI-defined device class to which the resource belongs, either:"
    "message based (VI_VXI_CLASS_MESSAGE),"
    "register based (VI_VXI_CLASS_REGISTER),"
    "extended (VI_VXI_CLASS_EXTENDED),"
    "or memory (VI_VXI_CLASS_MEMORY)."
    "VME devices are usually either register based or belong to a miscellaneous class (VI_VXI_CLASS_OTHER)."
    VxiDevClass(u16, VxiDevClassType),

    from = |value| {
        let value = value as u16;
        match value {
            x if x == VxiDevClassType::Memory as u16 => Some(Self(VxiDevClassType::Memory)),
            x if x == VxiDevClassType::Extended as u16 => Some(Self(VxiDevClassType::Extended)),
            x if x == VxiDevClassType::Message as u16 => Some(Self(VxiDevClassType::Message)),
            x if x == VxiDevClassType::Register as u16 => Some(Self(VxiDevClassType::Register)),
            x if x == VxiDevClassType::Other as u16 => Some(Self(VxiDevClassType::Other)),
            _ => None,
        }
    }
);

impl_attr!(
    "This attribute shows which VXI trigger lines this implementation supports.
    This is a bit vector with bits 0-9 corresponding to VI_TRIG_TTL0 through VI_TRIG_ECL1."
    VxiTrigStatus(ReadOnlyU32)
);

impl_attr!(
    "This attribute shows the current state of the VXI/VME SYSFAIL (SYStem FAILure) backplane line."
    VxiVmeSysfailState(i16, State),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == State::Asserted as i16 => Some(Self(State::Asserted)),
            x if x == State::Unasserted as i16 => Some(Self(State::Unasserted)),
            x if x == State::Unknown as i16 => Some(Self(State::Unknown)),
            _ => None,
        }
    }
);

/*
impl_attr!(
    ""
    TcpipAddr()
);

impl_attr!(
    ""
    TcpipHostname()
);

impl_attr!(
    ""
    TcpipPort()
);

impl_attr!(
    ""
    TcpipDeviceName()
);

impl_attr!(
    ""
    TcpipNodelay()
);

impl_attr!(
    ""
    TcpipKeepalive()
);

*/

impl_attr!(
    "VI_ATTR_4882_COMPLIANT specifies whether the device is 488.2 compliant."
    Is4882Compliant(bool)
);

/*

impl_attr!(
    ""
    UsbSerialNum()
);

impl_attr!(
    ""
    UsbIntfcNum()
);

impl_attr!(
    ""
    UsbProtocol()
);

impl_attr!(
    ""
    UsbMaxIntrSize()
);

impl_attr!(
    ""
    PxiDevNum()
);

impl_attr!(
    ""
    PxiFuncNum()
);

impl_attr!(
    ""
    PxiBusNum()
);

impl_attr!(
    ""
    PxiChassis()
);

impl_attr!(
    ""
    PxiSlotpath()
);

impl_attr!(
    ""
    PxiSlotLbusLeft()
);

impl_attr!(
    ""
    PxiSlotLbusRight()
);

impl_attr!(
    ""
    PxiTrigBus()
);

impl_attr!(
    ""
    PxiStarTrigBus()
);

impl_attr!(
    ""
    PxiStarTrigLine()
);

impl_attr!(
    ""
    PxiSrcTrigBus()
);

impl_attr!(
    ""
    PxiDestTrigBus()
);

impl_attr!(
    ""
    PxiIsExpress()
);

impl_attr!(
    ""
    PxiSlotLwidth()
);

impl_attr!(
    ""
    PxiMaxLwidth()
);

impl_attr!(
    ""
    PxiActualLwidth()
);

impl_attr!(
    ""
    PxiDstarBus()
);

impl_attr!(
    ""
    PxiDstarSet()
);

impl_attr!(
    ""
    PxiAllowWriteCombine()
);

impl_attr!(
    ""
    TcpipHislipOverlapEn()
);

impl_attr!(
    ""
    TcpipHislipVersion()
);

impl_attr!(
    ""
    TcpipHislipMaxMessageKb()
);

impl_attr!(
    ""
    TcpipIsHislip()
);

impl_attr!(
    ""
    JobId()
);

impl_attr!(
    ""
    EventType()
);

impl_attr!(
    ""
    SigpStatusId()
);

impl_attr!(
    ""
    RecvTrigId()
);

impl_attr!(
    ""
    IntrStatusId()
);

impl_attr!(
    ""
    RetCount32()
);

impl_attr!(
    ""
    RecvIntrLevel()
);

impl_attr!(
    ""
    OperName()
);

impl_attr!(
    ""
    GpibRecvCicState()
);

impl_attr!(
    ""
    RecvTcpipAddr()
);

impl_attr!(
    ""
    UsbRecvIntrSize()
);

impl_attr!(
    ""
    UsbRecvIntrData()
);

impl_attr!(
    ""
    PxiRecvIntrSeq()
);

impl_attr!(
    ""
    PxiRecvIntrData()
);

impl_attr!(
    ""
    UserData()
);

impl_attr!(
    ""
    RetCount()
);

impl_attr!(
    ""
    WinBaseAddr()
);

impl_attr!(
    ""
    WinSize()
);

 */

impl_attr!(
    "VI_ATTR_MEM_BASE, VI_ATTR_MEM_BASE_32, and VI_ATTR_MEM_BASE_64 specify the base address of the device in VXIbus memory address space."
    "This base address is applicable to A24 or A32 address space."
    "If the value of VI_ATTR_MEM_SPACE is VI_A16_SPACE, the value of this attribute is meaningless for the given VXI device."
    MemBase(bindings::ViBusAddress, bindings::ViBusAddress),
    from = |value| {
        Some(Self(value))
    }
);

impl_attr!(
    "VI_ATTR_MEM_SIZE, VI_ATTR_MEM_SIZE_32, and VI_ATTR_MEM_SIZE_64 specify the size of memory requested by the device in VXIbus address space."
    "If the value of VI_ATTR_MEM_SPACE is VI_A16_SPACE, the value of this attribute is meaningless for the given VXI device."
    MemSize(bindings::ViBusSize, bindings::ViBusSize),
    from = |value| {
        Some(Self(value))
    }
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar0(u16)
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar1(u16)
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar2(u16)
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar3(u16)
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar4(u16)
);

impl_attr!(
    "Memory type used by the device in the specified BAR (if applicable)."
    PxiMemTypeBar5(u16)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar0(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar1(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar2(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar3(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar4(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar5(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar0(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar1(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar2(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar3(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar4(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding VI_ATTR_PXI_MEM_TYPE_BARx is VI_PXI_ADDR_NONE, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar5(ReadOnlyU32)
);

/// A string attribute value that cannot be modified.
pub type ReadOnlyString = String;

/// A boolean attribute value that cannot be modified.
pub type ReadOnlyBool = bool;

/// A 16-bit unsigned integer attribute value that cannot be modified.
pub type ReadOnlyU16 = u16;

/// A 32-bit unsigned integer attribute value that cannot be modified.
pub type ReadOnlyU32 = u32;

/// A 16-bit signed integer attribute value that cannot be modified.
pub type ReadOnlyI16 = i16;

#[repr(u32)]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AttributeType {
    RsrcClass = bindings::VI_ATTR_RSRC_CLASS,
    RsrcName = bindings::VI_ATTR_RSRC_NAME,
    RsrcImplVersion = bindings::VI_ATTR_RSRC_IMPL_VERSION,
    RsrcLockState = bindings::VI_ATTR_RSRC_LOCK_STATE,
    MaxQueueLength = bindings::VI_ATTR_MAX_QUEUE_LENGTH,
    FdcChnl = bindings::VI_ATTR_FDC_CHNL,
    FdcMode = bindings::VI_ATTR_FDC_MODE,
    FdcGenSignalEn = bindings::VI_ATTR_FDC_GEN_SIGNAL_EN,
    FdcUsePair = bindings::VI_ATTR_FDC_USE_PAIR,
    SendEndEn = bindings::VI_ATTR_SEND_END_EN,
    TermChar = bindings::VI_ATTR_TERMCHAR,
    TmoValue = bindings::VI_ATTR_TMO_VALUE,
    GpibReaddrEn = bindings::VI_ATTR_GPIB_READDR_EN,
    IoProt = bindings::VI_ATTR_IO_PROT,
    DmaAllowEn = bindings::VI_ATTR_DMA_ALLOW_EN,
    AsrlBaud = bindings::VI_ATTR_ASRL_BAUD,
    AsrlDataBits = bindings::VI_ATTR_ASRL_DATA_BITS,
    AsrlParity = bindings::VI_ATTR_ASRL_PARITY,
    AsrlStopBits = bindings::VI_ATTR_ASRL_STOP_BITS,
    AsrlFlowCntrl = bindings::VI_ATTR_ASRL_FLOW_CNTRL,
    RdBufOperMode = bindings::VI_ATTR_RD_BUF_OPER_MODE,
    RdBufSize = bindings::VI_ATTR_RD_BUF_SIZE,
    WrBufOperMode = bindings::VI_ATTR_WR_BUF_OPER_MODE,
    WrBufSize = bindings::VI_ATTR_WR_BUF_SIZE,
    SuppressEndEn = bindings::VI_ATTR_SUPPRESS_END_EN,
    TermCharEn = bindings::VI_ATTR_TERMCHAR_EN,
    DestAccessPriv = bindings::VI_ATTR_DEST_ACCESS_PRIV,
    DestByteOrder = bindings::VI_ATTR_DEST_BYTE_ORDER,
    SrcAccessPriv = bindings::VI_ATTR_SRC_ACCESS_PRIV,
    SrcByteOrder = bindings::VI_ATTR_SRC_BYTE_ORDER,
    SrcIncrement = bindings::VI_ATTR_SRC_INCREMENT,
    DestIncrement = bindings::VI_ATTR_DEST_INCREMENT,
    WinAccessPriv = bindings::VI_ATTR_WIN_ACCESS_PRIV,
    WinByteOrder = bindings::VI_ATTR_WIN_BYTE_ORDER,
    GpibAtnState = bindings::VI_ATTR_GPIB_ATN_STATE,
    GpibAddrState = bindings::VI_ATTR_GPIB_ADDR_STATE,
    GpibCicState = bindings::VI_ATTR_GPIB_CIC_STATE,
    GpibNdacState = bindings::VI_ATTR_GPIB_NDAC_STATE,
    GpibSrqState = bindings::VI_ATTR_GPIB_SRQ_STATE,
    GpibSysCntrlState = bindings::VI_ATTR_GPIB_SYS_CNTRL_STATE,
    GpibHs488CblLen = bindings::VI_ATTR_GPIB_HS488_CBL_LEN,
    CmdrLa = bindings::VI_ATTR_CMDR_LA,
    VxiDevClass = bindings::VI_ATTR_VXI_DEV_CLASS,
    MainframeLa = bindings::VI_ATTR_MAINFRAME_LA,
    ManfName = bindings::VI_ATTR_MANF_NAME,
    ModelName = bindings::VI_ATTR_MODEL_NAME,
    VxiVmeIntrStatus = bindings::VI_ATTR_VXI_VME_INTR_STATUS,
    VxiTrigStatus = bindings::VI_ATTR_VXI_TRIG_STATUS,
    VxiVmeSysfailState = bindings::VI_ATTR_VXI_VME_SYSFAIL_STATE,
    AsrlAvailNum = bindings::VI_ATTR_ASRL_AVAIL_NUM,
    AsrlCtsState = bindings::VI_ATTR_ASRL_CTS_STATE,
    AsrlDcdState = bindings::VI_ATTR_ASRL_DCD_STATE,
    AsrlDsrState = bindings::VI_ATTR_ASRL_DSR_STATE,
    AsrlDtrState = bindings::VI_ATTR_ASRL_DTR_STATE,
    AsrlEndIn = bindings::VI_ATTR_ASRL_END_IN,
    AsrlEndOut = bindings::VI_ATTR_ASRL_END_OUT,
    AsrlReplaceChar = bindings::VI_ATTR_ASRL_REPLACE_CHAR,
    AsrlRiState = bindings::VI_ATTR_ASRL_RI_STATE,
    AsrlRtsState = bindings::VI_ATTR_ASRL_RTS_STATE,
    AsrlXonChar = bindings::VI_ATTR_ASRL_XON_CHAR,
    AsrlXoffChar = bindings::VI_ATTR_ASRL_XOFF_CHAR,
    WinAccess = bindings::VI_ATTR_WIN_ACCESS,
    RmSession = bindings::VI_ATTR_RM_SESSION,
    VxiLa = bindings::VI_ATTR_VXI_LA,
    ManfId = bindings::VI_ATTR_MANF_ID,
    MemSpace = bindings::VI_ATTR_MEM_SPACE,
    ModelCode = bindings::VI_ATTR_MODEL_CODE,
    Slot = bindings::VI_ATTR_SLOT,
    IntfInstName = bindings::VI_ATTR_INTF_INST_NAME,
    ImmediateServ = bindings::VI_ATTR_IMMEDIATE_SERV,
    IntfParentNum = bindings::VI_ATTR_INTF_PARENT_NUM,
    RsrcSpecVersion = bindings::VI_ATTR_RSRC_SPEC_VERSION,
    IntfType = bindings::VI_ATTR_INTF_TYPE,
    GpibPrimaryAddr = bindings::VI_ATTR_GPIB_PRIMARY_ADDR,
    GpibSecondaryAddr = bindings::VI_ATTR_GPIB_SECONDARY_ADDR,
    RsrcManfName = bindings::VI_ATTR_RSRC_MANF_NAME,
    RsrcManfId = bindings::VI_ATTR_RSRC_MANF_ID,
    IntfNum = bindings::VI_ATTR_INTF_NUM,
    TrigId = bindings::VI_ATTR_TRIG_ID,
    GpibRenState = bindings::VI_ATTR_GPIB_REN_STATE,
    GpibUnaddrEn = bindings::VI_ATTR_GPIB_UNADDR_EN,
    DevStatusByte = bindings::VI_ATTR_DEV_STATUS_BYTE,
    FileAppendEn = bindings::VI_ATTR_FILE_APPEND_EN,
    VxiTrigSupport = bindings::VI_ATTR_VXI_TRIG_SUPPORT,
    TcpipAddr = bindings::VI_ATTR_TCPIP_ADDR,
    TcpipHostname = bindings::VI_ATTR_TCPIP_HOSTNAME,
    TcpipPort = bindings::VI_ATTR_TCPIP_PORT,
    TcpipDeviceName = bindings::VI_ATTR_TCPIP_DEVICE_NAME,
    TcpipNodelay = bindings::VI_ATTR_TCPIP_NODELAY,
    TcpipKeepalive = bindings::VI_ATTR_TCPIP_KEEPALIVE,
    Is4882Compliant = bindings::VI_ATTR_4882_COMPLIANT,
    UsbSerialNum = bindings::VI_ATTR_USB_SERIAL_NUM,
    UsbIntfcNum = bindings::VI_ATTR_USB_INTFC_NUM,
    UsbProtocol = bindings::VI_ATTR_USB_PROTOCOL,
    UsbMaxIntrSize = bindings::VI_ATTR_USB_MAX_INTR_SIZE,
    PxiDevNum = bindings::VI_ATTR_PXI_DEV_NUM,
    PxiFuncNum = bindings::VI_ATTR_PXI_FUNC_NUM,
    PxiBusNum = bindings::VI_ATTR_PXI_BUS_NUM,
    PxiChassis = bindings::VI_ATTR_PXI_CHASSIS,
    PxiSlotpath = bindings::VI_ATTR_PXI_SLOTPATH,
    PxiSlotLbusLeft = bindings::VI_ATTR_PXI_SLOT_LBUS_LEFT,
    PxiSlotLbusRight = bindings::VI_ATTR_PXI_SLOT_LBUS_RIGHT,
    PxiTrigBus = bindings::VI_ATTR_PXI_TRIG_BUS,
    PxiStarTrigBus = bindings::VI_ATTR_PXI_STAR_TRIG_BUS,
    PxiStarTrigLine = bindings::VI_ATTR_PXI_STAR_TRIG_LINE,
    PxiSrcTrigBus = bindings::VI_ATTR_PXI_SRC_TRIG_BUS,
    PxiDestTrigBus = bindings::VI_ATTR_PXI_DEST_TRIG_BUS,
    PxiMemTypeBar0 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR0,
    PxiMemTypeBar1 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR1,
    PxiMemTypeBar2 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR2,
    PxiMemTypeBar3 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR3,
    PxiMemTypeBar4 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR4,
    PxiMemTypeBar5 = bindings::VI_ATTR_PXI_MEM_TYPE_BAR5,
    PxiIsExpress = bindings::VI_ATTR_PXI_IS_EXPRESS,
    PxiSlotLwidth = bindings::VI_ATTR_PXI_SLOT_LWIDTH,
    PxiMaxLwidth = bindings::VI_ATTR_PXI_MAX_LWIDTH,
    PxiActualLwidth = bindings::VI_ATTR_PXI_ACTUAL_LWIDTH,
    PxiDstarBus = bindings::VI_ATTR_PXI_DSTAR_BUS,
    PxiDstarSet = bindings::VI_ATTR_PXI_DSTAR_SET,
    PxiAllowWriteCombine = bindings::VI_ATTR_PXI_ALLOW_WRITE_COMBINE,
    TcpipHislipOverlapEn = bindings::VI_ATTR_TCPIP_HISLIP_OVERLAP_EN,
    TcpipHislipVersion = bindings::VI_ATTR_TCPIP_HISLIP_VERSION,
    TcpipHislipMaxMessageKb = bindings::VI_ATTR_TCPIP_HISLIP_MAX_MESSAGE_KB,
    TcpipIsHislip = bindings::VI_ATTR_TCPIP_IS_HISLIP,
    JobId = bindings::VI_ATTR_JOB_ID,
    EventType = bindings::VI_ATTR_EVENT_TYPE,
    SigpStatusId = bindings::VI_ATTR_SIGP_STATUS_ID,
    RecvTrigId = bindings::VI_ATTR_RECV_TRIG_ID,
    IntrStatusId = bindings::VI_ATTR_INTR_STATUS_ID,
    RecvIntrLevel = bindings::VI_ATTR_RECV_INTR_LEVEL,
    OperName = bindings::VI_ATTR_OPER_NAME,
    GpibRecvCicState = bindings::VI_ATTR_GPIB_RECV_CIC_STATE,
    RecvTcpipAddr = bindings::VI_ATTR_RECV_TCPIP_ADDR,
    UsbRecvIntrSize = bindings::VI_ATTR_USB_RECV_INTR_SIZE,
    UsbRecvIntrData = bindings::VI_ATTR_USB_RECV_INTR_DATA,
    PxiRecvIntrSeq = bindings::VI_ATTR_PXI_RECV_INTR_SEQ,
    PxiRecvIntrData = bindings::VI_ATTR_PXI_RECV_INTR_DATA,
    UserData = bindings::VI_ATTR_USER_DATA,
    RetCount = bindings::VI_ATTR_RET_COUNT,
    WinBaseAddr = bindings::VI_ATTR_WIN_BASE_ADDR,
    WinSize = bindings::VI_ATTR_WIN_SIZE,
    MemBase = bindings::VI_ATTR_MEM_BASE,
    MemSize = bindings::VI_ATTR_MEM_SIZE,
    PxiMemBaseBar0 = bindings::VI_ATTR_PXI_MEM_BASE_BAR0,
    PxiMemBaseBar1 = bindings::VI_ATTR_PXI_MEM_BASE_BAR1,
    PxiMemBaseBar2 = bindings::VI_ATTR_PXI_MEM_BASE_BAR2,
    PxiMemBaseBar3 = bindings::VI_ATTR_PXI_MEM_BASE_BAR3,
    PxiMemBaseBar4 = bindings::VI_ATTR_PXI_MEM_BASE_BAR4,
    PxiMemBaseBar5 = bindings::VI_ATTR_PXI_MEM_BASE_BAR5,
    PxiMemSizeBar0 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR0,
    PxiMemSizeBar1 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR1,
    PxiMemSizeBar2 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR2,
    PxiMemSizeBar3 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR3,
    PxiMemSizeBar4 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR4,
    PxiMemSizeBar5 = bindings::VI_ATTR_PXI_MEM_SIZE_BAR5,
}

/// Access modes for opening a session
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessMode {
    /// Used to acquire an exclusive lock immediately upon opening a session.
    /// If a lock cannot be acquired, the session is closed and an error is returned
    ExclusiveLock = bindings::VI_EXCLUSIVE_LOCK,

    /// Used to acquire a lock on the resource when the session is opened
    /// Not valid for session opening
    SharedLock = bindings::VI_SHARED_LOCK,

    /// Uses VISA supplied default values for the session
    NoLock = bindings::VI_NO_LOCK,

    /// Used to configure attributes to values specified by some external configuration utility
    /// NI-VISA currently supports `VI_LOAD_CONFIG` only on Serial INSTR sessions
    LoadConfig = bindings::VI_LOAD_CONFIG,
}
