//! Resource related attributes
use super::AccessMode;
use crate::bindings;

impl_attr!(
    "`VI_ATTR_RSRC_CLASS` specifies the resource class (for example, 'INSTR') as defined by the canonical resource name."
    RsrcClass(ReadOnlyString)
);

impl_attr!(
    "`VI_ATTR_RSRC_NAME` is the unique identifier for a resource."
    "Refer to VISA Resource Syntax and Examples for the syntax of resource strings and examples."
    RsrcName(ReadOnlyString)
);

impl_attr!(
    "`VI_ATTR_RSRC_IMPL_VERSION` is the resource version that uniquely identifies each of the different revisions or implementations of a resource."
    "This attribute value is defined by the individual manufacturer and increments with each new revision."
    "The format of the value has the upper 12 bits as the major number of the version,"
    "the next lower 12 bits as the minor number of the version,"
    "and the lowest 8 bits as the sub-minor number of the version."
    RsrcImplVersion(u32)
);

impl_attr!(
    "`VI_ATTR_RSRC_LOCK_STATE` indicates the current locking state of the resource."
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
    "`VI_ATTR_RSRC_SPEC_VERSION` is the resource version that uniquely identifies the version of the VISA specification to which the implementation is compliant."
    "The format of the value has the upper 12 bits as the major number of the version, the next lower 12 bits as the minor number of the version, and the lowest 8 bits as the sub-minor number of the version."
    "The current VISA specification defines the value to be 00300000h."
    RsrcSpecVersion(ReadOnlyU32)
);

impl_attr!(
    "`VI_ATTR_RSRC_MANF_NAME` is a string that corresponds to the manufacturer name of the vendor that implemented the VISA library."
    "This attribute is not related to the device manufacturer attributes."
    "The value of this attribute is for display purposes only and not for programmatic decisions, as the value can differ between VISA implementations and/or revisions"
    RsrcManfName(ReadOnlyString)
);

impl_attr!(
    "`VI_ATTR_RSRC_MANF_ID` is a value that corresponds to the VXI manufacturer ID of the vendor that implemented the VISA library."
    "This attribute is not related to the device manufacturer attributes."
    RsrcManfId(ReadOnlyU16)
);
