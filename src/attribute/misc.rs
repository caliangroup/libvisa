//! Attributes that do not fit into any specific category.
use crate::bindings;

/*
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
    TrigId()
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
    RecvTcpipAddr()
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

impl_attr!(
    "VI_ATTR_SUPPRESS_END_EN is relevant only in viRead and related operations."
    "For all session types on which this attribute is supported, if this attribute is set to VI_TRUE, read will not terminate due to an END condition. However, a read may still terminate successfully if"
    "VI_ATTR_TERMCHAR_EN is set to VI_TRUE. Otherwise, read will not terminate until all requested data is received (or an error occurs)."
    "On Serial INSTR sessions, if this attribute is set to VI_FALSE, VISA will perform the behavior described in VI_ATTR_ASRL_END_IN."
    "On USB RAW sessions, if this attribute is set to VI_FALSE, VISA will perform the behavior described in VI_ATTR_USB_END_IN."
    "On TCP/IP SOCKET sessions, if this attribute is set to VI_FALSE, if NI-VISA reads some data and then detects a pause in the arrival of data packets, it will terminate the read operation. On TCP/IP SOCKET sessions, this attribute defaults to VI_TRUE in NI-VISA."
    "On VXI INSTR sessions, if this attribute is set to VI_FALSE, the END bit terminates read operations."
    SuppressEndEn(bool)
);

impl_attr!(
    "VI_ATTR_TERMCHAR_EN is a flag that determines whether the read operation should terminate when a termination character is received."
    "This attribute is ignored if VI_ATTR_ASRL_END_IN is set to VI_ASRL_END_TERMCHAR. This attribute is valid for both raw I/O (viRead) and formatted I/O (viScanf)."
    TermCharEn(bool)
);

/// The type of the access mode
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessPrivilege {
    DataPriv = bindings::VI_DATA_PRIV,
    DataNPriv = bindings::VI_DATA_NPRIV,

    ProgPriv = bindings::VI_PROG_PRIV,
    ProgNPriv = bindings::VI_PROG_NPRIV,

    BlockPriv = bindings::VI_BLCK_PRIV,
    BlockNPriv = bindings::VI_BLCK_NPRIV,

    D64Priv = bindings::VI_D64_PRIV,
    D64NPriv = bindings::VI_D64_NPRIV,
}
impl_attr!(
    "VI_ATTR_DEST_ACCESS_PRIV specifies the address modifier to be used in high-level access operations, such as viOutXX() and viMoveOutXX(), when writing to the destination."
    "The values VI_D64_PRIV (6) and VI_D64_NPRIV (7) apply to only the block move operations"
    "If you set this attribute to one of these values and then call one of the viOutXX() operations, the operation returns VI_ERROR_INV_SETUP."
    DestAccessPriv(u16, AccessPrivilege),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AccessPrivilege::DataPriv as u32 => Some(Self(AccessPrivilege::DataPriv)),
            x if x == AccessPrivilege::DataNPriv as u32 => Some(Self(AccessPrivilege::DataNPriv)),
            x if x == AccessPrivilege::ProgPriv as u32 => Some(Self(AccessPrivilege::ProgPriv)),
            x if x == AccessPrivilege::ProgNPriv as u32 => Some(Self(AccessPrivilege::ProgNPriv)),
            x if x == AccessPrivilege::BlockPriv as u32 => Some(Self(AccessPrivilege::BlockPriv)),
            x if x == AccessPrivilege::BlockNPriv as u32 => Some(Self(AccessPrivilege::BlockNPriv)),
            x if x == AccessPrivilege::D64Priv as u32 => Some(Self(AccessPrivilege::D64Priv)),
            x if x == AccessPrivilege::D64NPriv as u32 => Some(Self(AccessPrivilege::D64NPriv)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "VI_ATTR_SRC_ACCESS_PRIV specifies the address modifier to be used in high-level access operations, such as viInXX() and viMoveInXX(), when reading from the source."
    "The values VI_D64_PRIV (6) and VI_D64_NPRIV (7) apply to only the block move operations"
    "If you set this attribute to one of these values and then call one of the viOutXX() operations, the operation returns VI_ERROR_INV_SETUP."
    SrcAccessPriv(u16, AccessPrivilege),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AccessPrivilege::DataPriv as u32 => Some(Self(AccessPrivilege::DataPriv)),
            x if x == AccessPrivilege::DataNPriv as u32 => Some(Self(AccessPrivilege::DataNPriv)),
            x if x == AccessPrivilege::ProgPriv as u32 => Some(Self(AccessPrivilege::ProgPriv)),
            x if x == AccessPrivilege::ProgNPriv as u32 => Some(Self(AccessPrivilege::ProgNPriv)),
            x if x == AccessPrivilege::BlockPriv as u32 => Some(Self(AccessPrivilege::BlockPriv)),
            x if x == AccessPrivilege::BlockNPriv as u32 => Some(Self(AccessPrivilege::BlockNPriv)),
            x if x == AccessPrivilege::D64Priv as u32 => Some(Self(AccessPrivilege::D64Priv)),
            x if x == AccessPrivilege::D64NPriv as u32 => Some(Self(AccessPrivilege::D64NPriv)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "VI_ATTR_WIN_ACCESS_PRIV specifies the address modifier to be used in low-level access operations, such as viMapAddress(), viPeekXX(), and viPokeXX(), when accessing the mapped window."
    "This attribute is Read/Write when the corresponding session is not mapped (that is, when VI_ATTR_WIN_ACCESS is VI_NMAPPED. When the session is mapped, this attribute is Read Only."
    WinAccessPriv(u16, AccessPrivilege),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == AccessPrivilege::DataPriv as u32 => Some(Self(AccessPrivilege::DataPriv)),
            x if x == AccessPrivilege::DataNPriv as u32 => Some(Self(AccessPrivilege::DataNPriv)),
            x if x == AccessPrivilege::ProgPriv as u32 => Some(Self(AccessPrivilege::ProgPriv)),
            x if x == AccessPrivilege::ProgNPriv as u32 => Some(Self(AccessPrivilege::ProgNPriv)),
            x if x == AccessPrivilege::BlockPriv as u32 => Some(Self(AccessPrivilege::BlockPriv)),
            x if x == AccessPrivilege::BlockNPriv as u32 => Some(Self(AccessPrivilege::BlockNPriv)),
            x if x == AccessPrivilege::D64Priv as u32 => Some(Self(AccessPrivilege::D64Priv)),
            x if x == AccessPrivilege::D64NPriv as u32 => Some(Self(AccessPrivilege::D64NPriv)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

/// The byte order to be used in high-level access operations, such as `viOutXX()` and `viMoveInXX()`
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    /// Big-endian byte order
    BigEndian = bindings::VI_BIG_ENDIAN,

    /// Little-endian byte order
    LittleEndian = bindings::VI_LITTLE_ENDIAN,
}

impl_attr!(
    "VI_ATTR_DEST_BYTE_ORDER specifies the byte order to be used in high-level access operations, such as viOutXX() and viMoveOutXX(), when writing to the destination."
    DestByteOrder(u16, ByteOrder),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == ByteOrder::BigEndian as u32 => Some(Self(ByteOrder::BigEndian)),
            x if x == ByteOrder::LittleEndian as u32 => Some(Self(ByteOrder::LittleEndian)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "VI_ATTR_SRC_BYTE_ORDER specifies the byte order to be used in high-level access operations, such as viInXX() and viMoveInXX(), when reading from the source."
    SrcByteOrder(u16, ByteOrder),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == ByteOrder::BigEndian as u32 => Some(Self(ByteOrder::BigEndian)),
            x if x == ByteOrder::LittleEndian as u32 => Some(Self(ByteOrder::LittleEndian)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);
impl_attr!(
    "VI_ATTR_WIN_BYTE_ORDER specifies the byte order to be used in low-level access operations, such as viMapAddress(), viPeekXX(), and viPokeXX(), when accessing the mapped window."
    "This attribute is Read/Write when the corresponding session is not mapped (that is, when VI_ATTR_WIN_ACCESS is VI_NMAPPED. When the session is mapped, this attribute is Read Only."
    WinByteOrder(u16, ByteOrder),

    from = |value| {
        let value = u32::from(value);
        match value {
            x if x == ByteOrder::BigEndian as u32 => Some(Self(ByteOrder::BigEndian)),
            x if x == ByteOrder::LittleEndian as u32 => Some(Self(ByteOrder::LittleEndian)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "VI_ATTR_SRC_INCREMENT is used in the viMoveInXX() operations to specify by how many elements the source offset is to be incremented after every transfer."
    "The default value of this attribute is 1 (that is, the source address will be incremented by 1 after each transfer), and the viMoveInXX() operations move from consecutive elements."
    "If this attribute is set to 0, the viMoveInXX() operations will always read from the same element, essentially treating the source as a FIFO register."
    SrcIncrement(i32, i32),

    from = |value| {
        match value {
            0|1 => Some(Self(value)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "VI_ATTR_DEST_INCREMENT is used in the viMoveOutXX() operations to specify by how many elements the destination offset is to be incremented after every transfer."
    "The default value of this attribute is 1 (that is, the source address will be incremented by 1 after each transfer), and the viMoveInXX() operations move from consecutive elements."
    "If this attribute is set to 0, the viMoveInXX() operations will always read from the same element, essentially treating the source as a FIFO register."
    DestIncrement(i32, i32),

    from = |value| {
        match value {
            0|1 => Some(Self(value)),
            _ => None,
        }
    }

    into = |value| {
        value as bindings::ViAttrState
    }
);

impl_attr!(
    "VI_ATTR_CMDR_LA is the unique logical address of the commander of the VXI device used by the given session."
    CmdrLa(i16, i16),

    from = |value| {
        match value {
            0..=255 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "VI_ATTR_MA.infRAME_LA specifies the lowest logical address in the mainframe. If the logical address is not known, VI_UNKNOWN_LA is returned."
    MainframeLa(i16, i16),

    from = |value| {
        match value {
            0..=255 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "This string attribute is the manufacturer name."
    "The value of this attribute should be used for display purposes only and not for programmatic decisions, as the value can differ between VISA implementations and/or revisions."
    ManfName(ReadOnlyString)
);

impl_attr!(
    "This string attribute is the model name of the device."
    ModelName(ReadOnlyString)
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
