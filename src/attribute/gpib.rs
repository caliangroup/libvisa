//! GPIB related attributes
use super::State;
use crate::bindings;

impl_attr!(
    "This attribute specifies whether the local controller has gained or lost CIC status."
    GpibRecvCicState(ReadOnlyBool)
);

impl_attr!(
    "`VI_ATTR_GPIB_READDR_EN` specifies whether to use repeat addressing before each read or write operation."
    GpibReaddrEn(bool)
);

impl_attr!(
    "`VI_ATTR_GPIB_PRIMARY_ADDR` specifies the primary address of the GPIB device used by the given session."
    "For the GPIB INTFC Resource, this attribute is Read-Write."
    "Valid values are 0 to 30."
    GpibPrimaryAddr(u16)
);

impl_attr!(
    "`VI_ATTR_GPIB_PRIMARY_ADDR` specifies the secondary address of the GPIB device used by the given session."
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
    "This attribute shows the current state of the GPIB ATN (`ATtentioN`) interface line."
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
    "This attribute shows the current state of the GPIB NDAC (Not Data `ACcepted`) interface line."
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
    "This attribute shows the current state of the GPIB SRQ (Service `ReQuest`) interface line."
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
    "`VI_ATTR_GPIB_REN_STATE` returns the current state of the GPIB REN (Remote `ENable`) interface line."
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
    "`VI_ATTR_GPIB_UNADDR_EN` specifies whether to unaddress the device (UNT and UNL) after each read or write operation."
    GpibUnaddrEn(bool)
);
