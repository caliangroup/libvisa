//! ASRL related attributes
#![allow(clippy::cast_enum_truncation)]
use crate::bindings;

impl_attr!(
    "`VI_ATTR_ASRL_BAUD` is the baud rate of the interface."
    "It is represented as an unsigned 32-bit integer so that any baud rate can be used, but it usually requires a commonly used rate such as: "
    "300, 1200, 2400, or 9600 baud."
    AsrlBaud(u32)
);

impl_attr!(
    "`VI_ATTR_ASRL_DATA_BITS` is the number of data bits contained in each frame (from 5 to 8)."
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
    "`VI_ATTR_ASRL_PARITY` is the parity used with every frame transmitted and received."
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
    "`VI_ATTR_ASRL_STOP_BITS` is the number of stop bits used to indicate the end of a frame."
    "The value `VI_ASRL_STOP_ONE5` indicates one-and-one-half (1.5) stop bits."
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
    "`VI_ATTR_ASRL_FLOW_CNTRL` indicates the type of flow control used by the transfer mechanism."
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
    "`VI_ATTR_ASRL_AVAIL_NUM` shows the number of bytes available in the low-level I/O receive buffer."
    AsrlAvailNum(ReadOnlyU32)
);

/// ASRL State modes
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsrlState {
    /// the state of the line is asserted
    Asserted = bindings::VI_STATE_ASSERTED,

    /// the state of the line is unasserted
    Unasserted = bindings::VI_STATE_UNASSERTED,

    /// the state of the line is unknown
    Unknown = bindings::VI_STATE_UNKNOWN as u32,
}
impl_attr!(
    "`VI_ATTR_ASRL_CTS_STATE` shows the current state of the Clear To Send (CTS) input signal."
    AsrlCtsState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }
);
impl_attr!(
    "`VI_ATTR_ASRL_DCD_STATE` represents the current state of the Data Carrier Detect (DCD) input signal."
    "The DCD signal is often used by modems to indicate the detection of a carrier (remote modem) on the telephone line."
    "The DCD signal is also known as Receive Line Signal Detect (RLSD)."
    "This attribute is Read Only except when the `VI_ATTR_ASRL_WIRE_MODE` attribute is set to `VI_ASRL_WIRE_232_DCE`, or `VI_ASRL_WIRE_232_AUTO` with the hardware currently in the DCE state."
    AsrlDcdState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "`VI_ATTR_ASRL_DSR_STATE` shows the current state of the Data Set Ready (DSR) input signal."
    AsrlDsrState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }
);
impl_attr!(
    "`VI_ATTR_ASRL_DTR_STATE` shows the current state of the Data Terminal Ready (DTR) input signal."
    "When the `VI_ATTR_ASRL_FLOW_CNTRL` attribute is set to `VI_ASRL_FLOW_DTR_DSR`, this attribute is Read Only. Querying the value will return `VI_STATE_UNKNOWN`."
    AsrlDtrState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "`VI_ATTR_ASRL_RI_STATE` represents the current state of the Ring Indicator (RI) input signal."
    "The RI signal is often used by modems to indicate that the telephone line is ringing."
    "This attribute is Read Only except when the `VI_ATTR_ASRL_WIRE_MODE` attribute is set to `VI_ASRL_WIRE_232_DCE`, or `VI_ASRL_WIRE_232_AUTO` with the hardware currently in the DCE state."
    AsrlRiState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "`VI_ATTR_ASRL_RTS_STATE` is used to manually assert or unassert the Request To Send (RTS) output signal."
    "When the `VI_ATTR_ASRL_FLOW_CNTRL` attribute is set to `VI_ASRL_FLOW_RTS_CTS`, this attribute is Read Only. Querying the value will return `VI_STATE_UNKNOWN`."
    AsrlRtsState(i16, AsrlState),

    from = |value| {
        let value = value as i16;
        match value {
            x if x == AsrlState::Asserted as i16 => Some(Self(AsrlState::Asserted)),
            x if x == AsrlState::Unasserted as i16 => Some(Self(AsrlState::Unasserted)),
            x if x == AsrlState::Unknown as i16 => Some(Self(AsrlState::Unknown)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

/// The type of the wire mode
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsrlEnd {
    /// the transfer mechanism does not use flow control, and buffers on both sides of the connection are assumed to be large enough to hold all data transferred.
    None = bindings::VI_ASRL_END_NONE,

    /// the transfer mechanism uses the XON and XOFF characters to perform flow control.
    LastBit = bindings::VI_ASRL_END_LAST_BIT,

    /// the transfer mechanism uses the RTS output signal and the CTS input signal to perform flow control.
    Termchar = bindings::VI_ASRL_END_TERMCHAR,

    /// the transfer mechanism uses the DTR output signal and the DSR input signal to perform flow control.
    /// - Only valid for `AsrlEndOut`
    Break = bindings::VI_ASRL_END_BREAK,
}
impl_attr!(
    "`VI_ATTR_ASRL_END_IN` indicates the method used to terminate read operations."
    "- If it is set to `VI_ASRL_END_NONE`, the read will not terminate until all of the requested data is received (or an error occurs)."
    "- If it is set to `VI_ASRL_END_LAST_BIT`, the read will terminate as soon as a character arrives with its last bit set. For example, if `VI_ATTR_ASRL_DATA_BITS` is set to 8, the read will terminate when a character arrives with the 8th bit set."
    "- If it is set to `VI_ASRL_END_TERMCHAR`, the read will terminate as soon as the character in `VI_ATTR_TERMCHAR` is received. In this case, `VI_ATTR_TERMCHAR_EN` is ignored."
    "Because the default value of `VI_ATTR_TERMCHAR` is 0Ah (linefeed), read operations on serial ports will stop reading whenever a linefeed is encountered. To change this behavior, you must change the value of one of these attributes — `VI_ATTR_ASRL_END_IN` or `VI_ATTR_TERMCHAR`."
    AsrlEndIn(u16, AsrlEnd),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AsrlEnd::None as u32 => Some(Self(AsrlEnd::None)),
            x if x == AsrlEnd::LastBit as u32 => Some(Self(AsrlEnd::LastBit)),
            x if x == AsrlEnd::Termchar as u32 => Some(Self(AsrlEnd::Termchar)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "`VI_ATTR_ASRL_END_OUT` indicates the method used to terminate write operations."
    "If it is set to `VI_ASRL_END_NONE`, the write will transmit the exact contents of the user buffer, without modifying it and without appending anything to the data being written."
    "If it is set to `VI_ASRL_END_LAST_BIT`, and `VI_ATTR_SEND_END_EN` is set to `VI_TRUE`, the write will send all but the last character with the highest bit clear, then transmit the last character with the highest bit set. For example, if `VI_ATTR_ASRL_DATA_BITS` is set to 8, the write will clear the eighth bit for all but the last character, then transmit the last character with the eighth bit set. If `VI_ATTR_SEND_END_EN` is set to `VI_FALSE`, the write will send all the characters with the highest bit clear."
    "If it is set to `VI_ASRL_END_TERMCHAR`, and `VI_ATTR_SEND_END_EN` is set to `VI_TRUE`, the write will send the character in `VI_ATTR_TERMCHAR` after the data being transmitted. If `VI_ATTR_SEND_END_EN` is set to `VI_FALSE`, the write will transmit the exact contents of the user buffer, without modifying it and without appending anything to the data being written."
    "If it is set to `VI_ASRL_END_BREAK`, and `VI_ATTR_SEND_END_EN` is set to `VI_TRUE`, the write will transmit a break after all the characters for the write have been sent. If `VI_ATTR_SEND_END_EN` is set to ``VI_FALSE``, the write will transmit the exact contents of the user buffer, without modifying it and without appending anything to the data being written."
    AsrlEndOut(u16, AsrlEnd),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AsrlEnd::None as u32 => Some(Self(AsrlEnd::None)),
            x if x == AsrlEnd::LastBit as u32 => Some(Self(AsrlEnd::LastBit)),
            x if x == AsrlEnd::Termchar as u32 => Some(Self(AsrlEnd::Termchar)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "`VI_ATTR_ASRL_REPLACE_CHAR` specifies the character to be used to replace incoming characters that arrive with errors (such as parity error)."
    AsrlReplaceChar(u8, char),

    from = |value| {
        Some(Self(value as u8 as char))
    }

    into = |value| {
        let c = match value {
            c if c.is_ascii() => c as u8,
            _ => b'?',
        };

        bindings::ViAttrState::from(c)
    }
);

impl_attr!(
    "`VI_ATTR_ASRL_XON_CHAR` specifies the value of the XON character used for XON/XOFF flow control (both directions)."
    "If XON/XOFF flow control (software handshaking) is not being used, the value of this attribute is ignored."
    AsrlXonChar(u8, char),

    from = |value| {
        Some(Self(value as u8 as char))
    }

    into = |value| {
        let c = match value {
            c if c.is_ascii() => c as u8,
            _ => b'?',
        };

        bindings::ViAttrState::from(c)
    }
);

impl_attr!(
    "`VI_ATTR_ASRL_XOFF_CHAR` specifies the value of the XOFF character used for XON/XOFF flow control (both directions)."
    "If XON/XOFF flow control (software handshaking) is not being used, the value of this attribute is ignored."
    AsrlXoffChar(u8, char),

    from = |value| {
        Some(Self(value as u8 as char))
    }

    into = |value| {
        let c = match value {
            c if c.is_ascii() => c as u8,
            _ => b'?',
        };

        bindings::ViAttrState::from(c)
    }
);
