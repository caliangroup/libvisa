//! USB related attributes
use crate::bindings;

impl_attr!(
    "`VI_ATTR_USB_SERIAL_NUM` specifies the USB serial number of this device."
    UsbSerialNum(ReadOnlyString)
);

impl_attr!(
    "`VI_ATTR_USB_INTFC_NUM` specifies the USB interface number used by the given session."
    UsbIntfcNum(ReadOnlyI16)
);

impl_attr!(
    "`VI_ATTR_USB_PROTOCOL` specifies the USB protocol used by this USB interface."
    UsbProtocol(ReadOnlyI16)
);

impl_attr!(
    "`VI_ATTR_USB_MAX_INTR_SIZE` specifies the maximum size of data that will be stored by any given USB interrupt."
    "If a USB interrupt contains more data than this size, the data in excess of this size will be lost."
    "`VI_ATTR_USB_MAX_INTR_SIZE` is Read/Write when the corresponding session is not enabled to receive USB interrupt events"
    "When the session is enabled to receive USB interrupt events, the attribute `VI_ATTR_USB_MAX_INTR_SIZE` is Read Only."
    UsbMaxIntrSize(u16)
);

impl_attr!(
    "`VI_ATTR_USB_RECV_INTR_SIZE` contains the number of bytes of USB interrupt data that is stored."
    UsbRecvIntrSize(ReadOnlyU16)
);

/// `VI_ATTR_USB_RECV_INTR_DATA` contains the actual received data from the USB Interrupt.
///
/// The passed in data buffer must be of size at least equal to the value of `VI_ATTR_USB_RECV_INTR_SIZE`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsbRecvIntrData<const USB_RECV_INTR_SIZE: usize>(pub [u8; USB_RECV_INTR_SIZE]);
impl<const USB_RECV_INTR_SIZE: usize> UsbRecvIntrData<{ USB_RECV_INTR_SIZE }> {
    /// Create a new instance of the attribute
    #[must_use]
    pub fn new(value: [u8; USB_RECV_INTR_SIZE]) -> Self {
        Self(value)
    }

    /// Create a new instance of the attribute
    #[must_use]
    pub fn attribute_type() -> u32 {
        crate::attribute::AttributeType::UsbRecvIntrData as u32
    }

    /// Get the required size of the buffer to store the data
    ///
    /// # Errors
    /// Returns an error if the size attribute cannot be read
    pub fn get_required_size(session: &crate::Session) -> Result<usize, crate::error::Error> {
        let size = session.get_attribute::<UsbRecvIntrSize>()?;
        Ok(usize::from(size))
    }
}
impl<const USB_RECV_INTR_SIZE: usize> crate::attribute::AsViReadable
    for UsbRecvIntrData<{ USB_RECV_INTR_SIZE }>
{
    const VI_ATTR: u32 = crate::attribute::AttributeType::UsbRecvIntrData as u32;
    type RawValue = [u8; USB_RECV_INTR_SIZE];
    type Value = [u8; USB_RECV_INTR_SIZE];

    fn from_value(value: Self::Value) -> Self
    where
        Self: Sized,
    {
        Self::new(value)
    }

    fn from_vi(value: Self::RawValue) -> Option<Self> {
        Some(Self(value))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }

    fn into_value(self) -> Self::Value {
        self.0
    }
}
