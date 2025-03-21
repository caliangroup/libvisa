use crate::bindings;

/// A type of atribute that can be read from a device
pub trait AsViReadable {
    /// The VISA attribute type for this attribute
    const VI_ATTR: u32;

    /// The raw type of the attribute expected by VISA
    type RawValue;

    /// The type of the attribute value after conversion
    type Value;

    /// Build an instance of this attribute from a value
    fn from_value(value: Self::Value) -> Self
    where
        Self: Sized;

    /// Convert a raw VISA attribute value to the attribute value
    fn from_vi(value: Self::RawValue) -> Option<Self>
    where
        Self: Sized;

    /// Get a reference to the attribute value
    fn value(&self) -> &Self::Value;

    /// Convert the attribute value to a raw VISA attribute value
    fn into_value(self) -> Self::Value;

    /// Attempt to read the value of this attribute from a session
    ///
    /// # Errors
    /// Returns an error if the attribute cannot be read
    fn read(session: &crate::Session) -> Result<Self::Value, crate::error::Error>
    where
        Self: Sized,
    {
        session.get_attribute::<Self>()
    }
}

/// A type of attribute that can be written to a device
pub trait AsViWritable: AsViReadable {
    /// Convert the attribute value to a raw VISA attribute state
    fn as_vi(&self) -> bindings::ViAttrState;

    /// Attempt to write the value of this attribute to a session
    ///
    /// # Errors
    /// Returns an error if the attribute cannot be written
    fn write(session: &mut crate::Session, value: Self::Value) -> Result<(), crate::error::Error>
    where
        Self: Sized,
    {
        session.set_attribute::<Self>(value)
    }
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

/// Macro simplifying the implementation of AsViReadable/AsViWritable for an attribute
macro_rules! impl_attr {
    (
        $($docs:literal)*
        $name:ident(ReadOnlyI16)
    ) => {
        impl_attr! {
            $($docs)*
            $name(i16, $crate::attribute::ReadOnlyI16), from = |value| {
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
        }
    };

    (
        $($docs:literal)*
        $name:ident(ReadOnlyString)
    ) => {
        impl_attr! {
            $($docs)*
            $name([std::ffi::c_char; 256], String),

            from = |value| {
                let value = unsafe { std::ffi::CStr::from_ptr(value.as_ptr()) };
                Some(Self(value.to_string_lossy().into_owned()))
            }
        }
    };

    (
        $($docs:literal)*
        $name:ident(ReadOnlyU32)
    ) => {
        impl_attr! {
            $($docs)*
            $name(u32, $crate::attribute::ReadOnlyU32), from = |value| {
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
            $name(u16, $crate::attribute::ReadOnlyU16), from = |value| {
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

            /// Get the attribute type
            #[must_use]
            pub fn attribute_type() -> u32 {
                $crate::attribute::AttributeType::$name as u32
            }
        }
        impl $crate::attribute::AsViReadable for $name {
            const VI_ATTR: u32 = $crate::attribute::AttributeType::$name as u32;
            type RawValue = $raw;
            type Value = $value;

            fn from_value(value: Self::Value) -> Self {
                Self::new(value)
            }

            fn from_vi($src_id: Self::RawValue) -> Option<Self> $from

            fn value(&self) -> &Self::Value {
                &self.0
            }

            fn into_value(self) -> Self::Value {
                self.0
            }
        }

        $(
            impl $crate::attribute::AsViWritable for $name {
                fn as_vi(&self) -> bindings::ViAttrState {
                    let $dst_id = self.0;
                    $into
                }
            }
        )?
    };
}
