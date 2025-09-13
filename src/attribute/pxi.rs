//! PXI related attributes
use crate::bindings;

impl_attr!(
    "For an INSTR session, `VI_ATTR_VXI_LA` specifies the logical address of the VXI or VME device used by the given session."
    "For a MEMACC or SERVANT session, this attribute specifies the logical address of the local controller."
    PxiDevNum(i16, i16),

    from = |value| {
        match value {
            0..=511 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "This is the PCI function number of the PXI/PCI resource"
    "For most devices, the function number is 0, but a multifunction device may have a function number up to 7."
    "The meaning of a function number other than 0 is device specific."
    PxiFuncNum(u16, u16),

    from = |value| {
        match value {
            0..=7 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "`VI_ATTR_PXI_BUS_NUM` specifies the PCI bus number of this device."
    PxiBusNum(u16, u16),

    from = |value| {
        match value {
            0..=255 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "`VI_ATTR_PXI_CHASSIS` specifies the PXI chassis number of this device. Returns none if the number is not known (-1 raw value)."
    PxiChassis(i16, i16),
    from = |value| {
        match value {
            0..=255 => Some(Self(value)),
            _ => None,
        }
    }
);

impl_attr!(
    "`VI_ATTR_PXI_SLOTPATH` specifies the slot path of this device."
    "The purpose of a PXI slot path is to describe the PCI bus hierarchy in a manner independent of the PCI bus number. PXI slot paths are a sequence of values representing the PCI device number and function number of a PCI module and each parent PCI bridge that routes the module to the host PCI bridge (bus 0). Each value is represented as \"dev[.func]\", where the function number is listed only if it is non-zero. When a PXI slot path includes multiple values, the values are comma-separated."
    "The string format of the attribute value looks like this:"
    "device1[.function1][,device2[.function2]][,...]"
    "An example string is \"5.1,12,8\". In this case, there is a PCI-to-PCI bridge on device 8 on the root bus. On its secondary bus, there is another PCI-to-PCI bridge on device 12. On its secondary bus, there is an instrument on device 5, function 1. The example string value describes this instrument's slot path."
    PxiSlotpath(ReadOnlyString)
);

/// PXI slot `LBus`
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PxiSlotLBus {
    StarTrigBus0 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_0,
    StarTrigBus1 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_1,
    StarTrigBus2 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_2,
    StarTrigBus3 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_3,
    StarTrigBus4 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_4,
    StarTrigBus5 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_5,
    StarTrigBus6 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_6,
    StarTrigBus7 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_7,
    StarTrigBus8 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_8,
    StarTrigBus9 = bindings::VI_PXI_LBUS_STAR_TRIG_BUS_9,
    StarTrigController = bindings::VI_PXI_STAR_TRIG_CONTROLLER,
}
impl_attr!(
    "`VI_ATTR_PXI_SLOT_LBUS_LEFT` specifies the slot number or special feature connected to the local bus left lines of this device."
    PxiSlotLbusLeft(i16, PxiSlotLBus),

    from = |value| {
        match value {
            x if x == PxiSlotLBus::StarTrigBus0 as i16 => Some(Self(PxiSlotLBus::StarTrigBus0)),
            x if x == PxiSlotLBus::StarTrigBus1 as i16 => Some(Self(PxiSlotLBus::StarTrigBus1)),
            x if x == PxiSlotLBus::StarTrigBus2 as i16 => Some(Self(PxiSlotLBus::StarTrigBus2)),
            x if x == PxiSlotLBus::StarTrigBus3 as i16 => Some(Self(PxiSlotLBus::StarTrigBus3)),
            x if x == PxiSlotLBus::StarTrigBus4 as i16 => Some(Self(PxiSlotLBus::StarTrigBus4)),
            x if x == PxiSlotLBus::StarTrigBus5 as i16 => Some(Self(PxiSlotLBus::StarTrigBus5)),
            x if x == PxiSlotLBus::StarTrigBus6 as i16 => Some(Self(PxiSlotLBus::StarTrigBus6)),
            x if x == PxiSlotLBus::StarTrigBus7 as i16 => Some(Self(PxiSlotLBus::StarTrigBus7)),
            x if x == PxiSlotLBus::StarTrigBus8 as i16 => Some(Self(PxiSlotLBus::StarTrigBus8)),
            x if x == PxiSlotLBus::StarTrigBus9 as i16 => Some(Self(PxiSlotLBus::StarTrigBus9)),
            x if x == PxiSlotLBus::StarTrigController as i16 => Some(Self(PxiSlotLBus::StarTrigController)),

            _ => None,
        }
    }
);
impl_attr!(
    "`VI_ATTR_PXI_SLOT_LBUS_RIGHT` specifies the slot number or special feature connected to the local bus right lines of this device."
    PxiSlotLbusRight(i16, PxiSlotLBus),

    from = |value| {
        match value {
            x if x == PxiSlotLBus::StarTrigBus0 as i16 => Some(Self(PxiSlotLBus::StarTrigBus0)),
            x if x == PxiSlotLBus::StarTrigBus1 as i16 => Some(Self(PxiSlotLBus::StarTrigBus1)),
            x if x == PxiSlotLBus::StarTrigBus2 as i16 => Some(Self(PxiSlotLBus::StarTrigBus2)),
            x if x == PxiSlotLBus::StarTrigBus3 as i16 => Some(Self(PxiSlotLBus::StarTrigBus3)),
            x if x == PxiSlotLBus::StarTrigBus4 as i16 => Some(Self(PxiSlotLBus::StarTrigBus4)),
            x if x == PxiSlotLBus::StarTrigBus5 as i16 => Some(Self(PxiSlotLBus::StarTrigBus5)),
            x if x == PxiSlotLBus::StarTrigBus6 as i16 => Some(Self(PxiSlotLBus::StarTrigBus6)),
            x if x == PxiSlotLBus::StarTrigBus7 as i16 => Some(Self(PxiSlotLBus::StarTrigBus7)),
            x if x == PxiSlotLBus::StarTrigBus8 as i16 => Some(Self(PxiSlotLBus::StarTrigBus8)),
            x if x == PxiSlotLBus::StarTrigBus9 as i16 => Some(Self(PxiSlotLBus::StarTrigBus9)),
            x if x == PxiSlotLBus::StarTrigController as i16 => Some(Self(PxiSlotLBus::StarTrigController)),

            _ => None,
        }
    }
);
/*
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
    PxiRecvIntrSeq()
);

impl_attr!(
    ""
    PxiRecvIntrData()
);*/

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
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar0(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar1(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar2(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar3(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar4(ReadOnlyU32)
);

impl_attr!(
    "PXI memory base address assigned to the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemBaseBar5(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar0(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar1(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar2(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar3(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar4(ReadOnlyU32)
);

impl_attr!(
    "Memory size used by the device in the specified BAR."
    "If the value of the corresponding `VI_ATTR_PXI_MEM_TYPE_BARx` is `VI_PXI_ADDR_NONE`, the value of this attribute is undefined for the given PXI device."
    PxiMemSizeBar5(ReadOnlyU32)
);
