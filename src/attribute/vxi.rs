//! VXI related attributes
use super::{ReadOnlyI16, State};
use crate::bindings;

impl_attr!(
    "For an INSTR session, VI_ATTR_VXI_LA specifies the logical address of the VXI or VME device used by the given session"
    "For a MEMACC or SERVANT session, this attribute specifies the logical address of the local controller."
    VxiLa(ReadOnlyI16, ReadOnlyI16),

    from = |value| {
        match value {
            0..=511 => Some(Self(value)),
            _ => None,
        }
    }
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

impl_attr!(
    "VI_ATTR_4882_COMPLIANT specifies whether the device is 488.2 compliant."
    Is4882Compliant(bool)
);
