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

#[macro_use]
mod traits;
pub use traits::*;

pub mod asrl;
pub mod gpib;
pub mod misc;
pub mod pxi;
pub mod rsrc;
pub mod tcpip;
pub mod usb;
pub mod vxi;

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

/// Attribute states
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Asserted = bindings::VI_STATE_ASSERTED as i32,
    Unasserted = bindings::VI_STATE_UNASSERTED as i32,
    Unknown = bindings::VI_STATE_UNKNOWN,
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_all_attributes_available() {
        let attr: AttributeType = AttributeType::RsrcClass;
        match attr {
            AttributeType::RsrcClass => rsrc::RsrcClass::attribute_type(),
            AttributeType::RsrcName => rsrc::RsrcName::attribute_type(),
            AttributeType::RsrcImplVersion => rsrc::RsrcImplVersion::attribute_type(),
            AttributeType::RsrcLockState => rsrc::RsrcLockState::attribute_type(),
            AttributeType::RsrcSpecVersion => rsrc::RsrcSpecVersion::attribute_type(),
            AttributeType::RsrcManfName => rsrc::RsrcManfName::attribute_type(),
            AttributeType::RsrcManfId => rsrc::RsrcManfId::attribute_type(),

            AttributeType::AsrlAvailNum => asrl::AsrlAvailNum::attribute_type(),
            AttributeType::AsrlBaud => asrl::AsrlBaud::attribute_type(),
            AttributeType::AsrlDataBits => asrl::AsrlDataBits::attribute_type(),
            AttributeType::AsrlParity => asrl::AsrlParity::attribute_type(),
            AttributeType::AsrlStopBits => asrl::AsrlStopBits::attribute_type(),
            AttributeType::AsrlFlowCntrl => asrl::AsrlFlowCntrl::attribute_type(),
            AttributeType::AsrlCtsState => asrl::AsrlCtsState::attribute_type(),
            AttributeType::AsrlDcdState => asrl::AsrlDcdState::attribute_type(),
            AttributeType::AsrlDsrState => asrl::AsrlDsrState::attribute_type(),
            AttributeType::AsrlDtrState => asrl::AsrlDtrState::attribute_type(),
            AttributeType::AsrlEndIn => asrl::AsrlEndIn::attribute_type(),
            AttributeType::AsrlEndOut => asrl::AsrlEndOut::attribute_type(),
            AttributeType::AsrlReplaceChar => asrl::AsrlReplaceChar::attribute_type(),
            AttributeType::AsrlRiState => asrl::AsrlRiState::attribute_type(),
            AttributeType::AsrlRtsState => asrl::AsrlRtsState::attribute_type(),
            AttributeType::AsrlXonChar => asrl::AsrlXonChar::attribute_type(),
            AttributeType::AsrlXoffChar => asrl::AsrlXoffChar::attribute_type(),

            AttributeType::TcpipAddr => tcpip::TcpipAddr::attribute_type(),
            AttributeType::TcpipHostname => tcpip::TcpipHostname::attribute_type(),
            AttributeType::TcpipPort => tcpip::TcpipPort::attribute_type(),
            AttributeType::TcpipDeviceName => tcpip::TcpipDeviceName::attribute_type(),
            AttributeType::TcpipNodelay => tcpip::TcpipNodelay::attribute_type(),
            AttributeType::TcpipKeepalive => tcpip::TcpipKeepalive::attribute_type(),
            AttributeType::TcpipHislipOverlapEn => tcpip::TcpipHislipOverlapEn::attribute_type(),
            AttributeType::TcpipHislipVersion => tcpip::TcpipHislipVersion::attribute_type(),
            AttributeType::TcpipHislipMaxMessageKb => {
                tcpip::TcpipHislipMaxMessageKb::attribute_type()
            }
            AttributeType::TcpipIsHislip => tcpip::TcpipIsHislip::attribute_type(),

            AttributeType::GpibAtnState => gpib::GpibAtnState::attribute_type(),
            AttributeType::GpibAddrState => gpib::GpibAddrState::attribute_type(),
            AttributeType::GpibCicState => gpib::GpibCicState::attribute_type(),
            AttributeType::GpibNdacState => gpib::GpibNdacState::attribute_type(),
            AttributeType::GpibSrqState => gpib::GpibSrqState::attribute_type(),
            AttributeType::GpibSysCntrlState => gpib::GpibSysCntrlState::attribute_type(),
            AttributeType::GpibHs488CblLen => gpib::GpibHs488CblLen::attribute_type(),
            AttributeType::GpibPrimaryAddr => gpib::GpibPrimaryAddr::attribute_type(),
            AttributeType::GpibSecondaryAddr => gpib::GpibSecondaryAddr::attribute_type(),
            AttributeType::GpibRecvCicState => gpib::GpibRecvCicState::attribute_type(),
            AttributeType::GpibRenState => gpib::GpibRenState::attribute_type(),
            AttributeType::GpibUnaddrEn => gpib::GpibUnaddrEn::attribute_type(),
            AttributeType::GpibReaddrEn => gpib::GpibReaddrEn::attribute_type(),

            AttributeType::VxiDevClass => vxi::VxiDevClass::attribute_type(),
            AttributeType::VxiVmeIntrStatus => vxi::VxiVmeIntrStatus::attribute_type(),
            AttributeType::VxiTrigStatus => vxi::VxiTrigStatus::attribute_type(),
            AttributeType::VxiVmeSysfailState => vxi::VxiVmeSysfailState::attribute_type(),
            AttributeType::VxiLa => vxi::VxiLa::attribute_type(),
            AttributeType::VxiTrigSupport => vxi::VxiTrigSupport::attribute_type(),

            AttributeType::PxiDevNum => pxi::PxiDevNum::attribute_type(),
            AttributeType::PxiFuncNum => pxi::PxiFuncNum::attribute_type(),
            AttributeType::PxiBusNum => pxi::PxiBusNum::attribute_type(),
            AttributeType::PxiChassis => pxi::PxiChassis::attribute_type(),
            AttributeType::PxiSlotpath => pxi::PxiSlotpath::attribute_type(),
            AttributeType::PxiSlotLbusLeft => pxi::PxiSlotLbusLeft::attribute_type(),
            AttributeType::PxiSlotLbusRight => pxi::PxiSlotLbusRight::attribute_type(),
            AttributeType::PxiMemTypeBar0 => pxi::PxiMemTypeBar0::attribute_type(),
            AttributeType::PxiMemTypeBar1 => pxi::PxiMemTypeBar1::attribute_type(),
            AttributeType::PxiMemTypeBar2 => pxi::PxiMemTypeBar2::attribute_type(),
            AttributeType::PxiMemTypeBar3 => pxi::PxiMemTypeBar3::attribute_type(),
            AttributeType::PxiMemTypeBar4 => pxi::PxiMemTypeBar4::attribute_type(),
            AttributeType::PxiMemTypeBar5 => pxi::PxiMemTypeBar5::attribute_type(),
            AttributeType::PxiMemBaseBar0 => pxi::PxiMemBaseBar0::attribute_type(),
            AttributeType::PxiMemBaseBar1 => pxi::PxiMemBaseBar1::attribute_type(),
            AttributeType::PxiMemBaseBar2 => pxi::PxiMemBaseBar2::attribute_type(),
            AttributeType::PxiMemBaseBar3 => pxi::PxiMemBaseBar3::attribute_type(),
            AttributeType::PxiMemBaseBar4 => pxi::PxiMemBaseBar4::attribute_type(),
            AttributeType::PxiMemBaseBar5 => pxi::PxiMemBaseBar5::attribute_type(),
            AttributeType::PxiMemSizeBar0 => pxi::PxiMemSizeBar0::attribute_type(),
            AttributeType::PxiMemSizeBar1 => pxi::PxiMemSizeBar1::attribute_type(),
            AttributeType::PxiMemSizeBar2 => pxi::PxiMemSizeBar2::attribute_type(),
            AttributeType::PxiMemSizeBar3 => pxi::PxiMemSizeBar3::attribute_type(),
            AttributeType::PxiMemSizeBar4 => pxi::PxiMemSizeBar4::attribute_type(),
            AttributeType::PxiMemSizeBar5 => pxi::PxiMemSizeBar5::attribute_type(),

            AttributeType::PxiIsExpress => todo!(), //pxi::PxiIsExpress::attribute_type(),
            AttributeType::PxiSlotLwidth => todo!(), //pxi::PxiSlotLwidth::attribute_type(),
            AttributeType::PxiMaxLwidth => todo!(), //pxi::PxiMaxLwidth::attribute_type(),
            AttributeType::PxiActualLwidth => todo!(), //pxi::PxiActualLwidth::attribute_type(),
            AttributeType::PxiDstarBus => todo!(),  //pxi::PxiDstarBus::attribute_type(),
            AttributeType::PxiDstarSet => todo!(),  //pxi::PxiDstarSet::attribute_type(),
            AttributeType::PxiAllowWriteCombine => todo!(), //pxi::PxiAllowWriteCombine::attribute_type(),
            AttributeType::PxiRecvIntrSeq => todo!(),       //pxi::PxiRecvIntrSeq::attribute_type(),
            AttributeType::PxiRecvIntrData => todo!(), //pxi::PxiRecvIntrData::attribute_type(),
            AttributeType::PxiTrigBus => todo!(),      //pxi::PxiTrigBus::attribute_type(),
            AttributeType::PxiStarTrigBus => todo!(),  //pxi::PxiStarTrigBus::attribute_type(),
            AttributeType::PxiStarTrigLine => todo!(), //pxi::PxiStarTrigLine::attribute_type(),
            AttributeType::PxiSrcTrigBus => todo!(),   //pxi::PxiSrcTrigBus::attribute_type(),
            AttributeType::PxiDestTrigBus => todo!(),  //pxi::PxiDestTrigBus::attribute_type(),

            AttributeType::UsbSerialNum => usb::UsbSerialNum::attribute_type(),
            AttributeType::UsbIntfcNum => usb::UsbIntfcNum::attribute_type(),
            AttributeType::UsbProtocol => usb::UsbProtocol::attribute_type(),
            AttributeType::UsbMaxIntrSize => usb::UsbMaxIntrSize::attribute_type(),
            AttributeType::UsbRecvIntrSize => usb::UsbRecvIntrSize::attribute_type(),
            AttributeType::UsbRecvIntrData => usb::UsbRecvIntrData::<0>::attribute_type(),

            AttributeType::DevStatusByte => misc::DevStatusByte::attribute_type(),
            AttributeType::MemBase => misc::MemBase::attribute_type(),
            AttributeType::MemSize => misc::MemSize::attribute_type(),
            AttributeType::MaxQueueLength => misc::MaxQueueLength::attribute_type(),
            AttributeType::FdcChnl => misc::FdcChnl::attribute_type(),
            AttributeType::FdcMode => misc::FdcMode::attribute_type(),
            AttributeType::FdcGenSignalEn => misc::FdcGenSignalEn::attribute_type(),
            AttributeType::FdcUsePair => misc::FdcUsePair::attribute_type(),
            AttributeType::SendEndEn => misc::SendEndEn::attribute_type(),
            AttributeType::TermChar => misc::TermChar::attribute_type(),
            AttributeType::TmoValue => misc::TmoValue::attribute_type(),
            AttributeType::IoProt => misc::IoProt::attribute_type(),
            AttributeType::DmaAllowEn => misc::DmaAllowEn::attribute_type(),
            AttributeType::RdBufOperMode => misc::RdBufOperMode::attribute_type(),
            AttributeType::RdBufSize => misc::RdBufSize::attribute_type(),
            AttributeType::WrBufOperMode => misc::WrBufOperMode::attribute_type(),
            AttributeType::WrBufSize => misc::WrBufSize::attribute_type(),
            AttributeType::SuppressEndEn => misc::SuppressEndEn::attribute_type(),
            AttributeType::TermCharEn => misc::TermCharEn::attribute_type(),
            AttributeType::DestAccessPriv => misc::DestAccessPriv::attribute_type(),
            AttributeType::DestByteOrder => misc::DestByteOrder::attribute_type(),
            AttributeType::SrcAccessPriv => misc::SrcAccessPriv::attribute_type(),
            AttributeType::SrcByteOrder => misc::SrcByteOrder::attribute_type(),
            AttributeType::SrcIncrement => misc::SrcIncrement::attribute_type(),
            AttributeType::DestIncrement => misc::DestIncrement::attribute_type(),
            AttributeType::WinAccessPriv => misc::WinAccessPriv::attribute_type(),
            AttributeType::WinByteOrder => misc::WinByteOrder::attribute_type(),
            AttributeType::CmdrLa => misc::CmdrLa::attribute_type(),
            AttributeType::MainframeLa => misc::MainframeLa::attribute_type(),
            AttributeType::ManfName => misc::ManfName::attribute_type(),
            AttributeType::ModelName => misc::ModelName::attribute_type(),
            AttributeType::IntfType => misc::IntfType::attribute_type(),
            AttributeType::IntfNum => misc::IntfNum::attribute_type(),
            AttributeType::FileAppendEn => misc::FileAppendEn::attribute_type(),

            AttributeType::JobId => todo!(), //misc::JobId::attribute_type(),
            AttributeType::EventType => todo!(), //misc::EventType::attribute_type(),
            AttributeType::SigpStatusId => todo!(), //misc::SigpStatusId::attribute_type(),
            AttributeType::RecvTrigId => todo!(), //misc::RecvTrigId::attribute_type(),
            AttributeType::IntrStatusId => todo!(), //misc::IntrStatusId::attribute_type(),
            AttributeType::RecvIntrLevel => todo!(), //misc::RecvIntrLevel::attribute_type(),
            AttributeType::OperName => todo!(), //misc::OperName::attribute_type(),
            AttributeType::RecvTcpipAddr => todo!(), //misc::RecvTcpipAddr::attribute_type(),
            AttributeType::UserData => todo!(), //misc::UserData::attribute_type(),
            AttributeType::RetCount => todo!(), //misc::RetCount::attribute_type(),
            AttributeType::WinBaseAddr => todo!(), //misc::WinBaseAddr::attribute_type(),
            AttributeType::WinSize => todo!(), //misc::WinSize::attribute_type(),
            AttributeType::Is4882Compliant => todo!(), //misc::Is4882Compliant::attribute_type(),
            AttributeType::TrigId => todo!(), //misc::TrigId::attribute_type(),
            AttributeType::WinAccess => todo!(), //misc::WinAccess::attribute_type(),
            AttributeType::RmSession => todo!(), //misc::RmSession::attribute_type(),
            AttributeType::ManfId => todo!(), //misc::ManfId::attribute_type(),
            AttributeType::MemSpace => todo!(), //misc::MemSpace::attribute_type(),
            AttributeType::ModelCode => todo!(), //misc::ModelCode::attribute_type(),
            AttributeType::Slot => todo!(),  //misc::Slot::attribute_type(),
            AttributeType::IntfInstName => todo!(), //misc::IntfInstName::attribute_type(),
            AttributeType::ImmediateServ => todo!(), //misc::ImmediateServ::attribute_type(),
            AttributeType::IntfParentNum => todo!(), //misc::IntfParentNum::attribute_type(),
        };
    }
}
