use crate::{
    attribute::{self, AsViReadable},
    bindings,
    error::Error,
};

/// Resource manager session, used to find resources and open sessions
pub type ResourceManagerSession = bindings::ViSession;

/// Resource details, obtained by calling `Resource::get_details`
#[derive(Debug)]
pub struct ResourceDetails {
    /// Interface type - Corresponds to the `VI_ATTR_INTF_TYPE` attribute.
    pub interface_type: <attribute::misc::IntfType as attribute::AsViReadable>::Value,

    /// Board number - Corresponds to the `VI_ATTR_INTF_NUM` attribute.
    pub board_number: <attribute::misc::IntfNum as attribute::AsViReadable>::Value,

    /// Class - Corresponds to the `VI_ATTR_RSRC_CLASS` attribute.
    pub class: String,

    /// Expanded name - should in most cases be identical to the VISA-defined canonical resource name.
    pub expanded_name: String,

    /// Alias, allows programmatic access to user-defined aliases.
    /// Will be empty if the resource has no alias.
    pub alias: String,
}

/// Resource, obtained by calling `ResourceManager::find_resources`
#[derive(Debug)]
pub struct Resource {
    interface: String,
    rm: ResourceManagerSession,
}
impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.interface)
    }
}
impl AsRef<str> for Resource {
    fn as_ref(&self) -> &str {
        &self.interface
    }
}
impl Resource {
    const MAX_DESC: usize = 256;

    /// Returns a resource name that can be used to open a session
    #[must_use]
    pub fn interface(&self) -> &str {
        &self.interface
    }

    /// Get extended details for the resource
    ///
    /// # Errors
    /// Will return an error if the resource cannot be found, or the device returns an error
    pub fn get_details(&self) -> Result<ResourceDetails, Error> {
        let mut interface = 0u16;
        let mut board = 0u16;
        let mut class = [0i8; Self::MAX_DESC];
        let mut expanded_name = [0i8; Self::MAX_DESC];
        let mut alias = [0i8; Self::MAX_DESC];
        let name = std::ffi::CString::new(self.interface.as_str())?;

        Error::wrap_binding(Some(self.rm), || unsafe {
            bindings::viParseRsrcEx(
                self.rm,
                name.as_ptr(),
                &mut interface,
                &mut board,
                class.as_mut_ptr(),
                expanded_name.as_mut_ptr(),
                alias.as_mut_ptr(),
            )
        })?;

        let interface_type: <attribute::misc::IntfType as attribute::AsViReadable>::RawValue = 0;
        let interface_type =
            attribute::misc::IntfType::from_vi(interface_type).ok_or_else(Error::default)?;

        let board_number: <attribute::misc::IntfNum as attribute::AsViReadable>::RawValue = 0;
        let board_number =
            attribute::misc::IntfNum::from_vi(board_number).ok_or_else(Error::default)?;

        let cstr = unsafe { std::ffi::CStr::from_ptr(class.as_ptr()) };
        let class = cstr.to_string_lossy().into_owned();

        let cstr = unsafe { std::ffi::CStr::from_ptr(expanded_name.as_ptr()) };
        let expanded_name = cstr.to_string_lossy().into_owned();

        let cstr = unsafe { std::ffi::CStr::from_ptr(alias.as_ptr()) };
        let alias = cstr.to_string_lossy().into_owned();

        Ok(ResourceDetails {
            interface_type: interface_type.into_value(),
            board_number: board_number.into_value(),
            class,
            expanded_name,
            alias,
        })
    }
}

/// Iterator over search results from `ResourceManager::search`
#[derive(Debug)]
pub struct ResourceSearchResult {
    rm: ResourceManagerSession,
    len: usize,
    list: bindings::ViFindList,
    next: Option<Resource>,
}
impl ResourceSearchResult {
    const MAX_INTERFACE: usize = 4096;
    fn advance(&mut self) -> Result<(), Error> {
        if self.len == 0 {
            return Ok(());
        }

        let mut desc = [0u8; Self::MAX_INTERFACE];
        Error::wrap_binding(Some(self.list), || unsafe {
            bindings::viFindNext(self.list, desc.as_mut_ptr().cast::<i8>())
        })?;

        if let Ok(cstr) = std::ffi::CStr::from_bytes_with_nul(&desc) {
            let desc = cstr.to_string_lossy().into_owned();

            let resource = Resource {
                interface: desc,
                rm: self.rm,
            };

            self.next = Some(resource);
        } else {
            self.next = None;
        }

        self.len -= 1;
        Ok(())
    }
}
impl Iterator for ResourceSearchResult {
    type Item = Resource;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let next = self.next();
        self.advance().ok()?;
        next
    }
}

static SINGLETON_RM_ID: std::sync::Mutex<Option<bindings::ViSession>> = std::sync::Mutex::new(None);
fn get_singleton_rm_id() -> Option<bindings::ViSession> {
    *SINGLETON_RM_ID.lock().unwrap()
}
fn set_singleton_rm_id(id: bindings::ViSession) {
    *SINGLETON_RM_ID.lock().unwrap() = Some(id);
}

/// Resource manager, used to search for resources and open sessions
#[derive(Debug)]
pub struct ResourceManager(bindings::ViSession);
impl ResourceManager {
    /// Create a new resource manager
    ///
    /// If a resource manager already exists, it will be reused
    ///
    /// # Errors
    /// Will return an error if the resource manager cannot be created
    pub fn new() -> Result<Self, Error> {
        if let Some(id) = get_singleton_rm_id() {
            return Ok(Self(id));
        }

        let mut id = ResourceManagerSession::default();
        Error::wrap_binding(None, || unsafe { bindings::viOpenDefaultRM(&mut id) })?;

        set_singleton_rm_id(id);
        Ok(Self(id))
    }

    pub(crate) fn session_id(&self) -> ResourceManagerSession {
        self.0
    }

    /// Search for resources using a regular expression
    /// The search is case-insensitive
    ///
    /// Returns an iterator over identifiers that can be used to open a session
    ///
    /// # Errors
    /// Will return an error if the search fails
    pub fn search(&self, expr: &str) -> Result<ResourceSearchResult, Error> {
        let expr = std::ffi::CString::new(expr)?;
        let mut list: bindings::ViFindList = bindings::ViFindList::default();
        let mut count = 0u32;
        let mut interface = [0i8; ResourceSearchResult::MAX_INTERFACE];

        Error::wrap_binding(Some(self.0), || unsafe {
            bindings::viFindRsrc(
                self.0,
                expr.as_ptr(),
                &mut list,
                &mut count,
                interface.as_mut_ptr(),
            )
        })?;

        let cstr = unsafe { std::ffi::CStr::from_ptr(interface.as_ptr()) };
        let interface = cstr.to_string_lossy().into_owned();
        let resource = Resource {
            interface,
            rm: self.0,
        };

        Ok(ResourceSearchResult {
            rm: self.0,
            len: count as usize - 1,
            list,
            next: Some(resource),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_singleton() {
        let rm1 = ResourceManager::new().unwrap();
        let rm2 = ResourceManager::new().unwrap();

        // Make sure they point to the same instance
        assert_eq!(rm1.session_id(), rm2.session_id());
    }

    #[test]
    fn test_search() {
        let rm = ResourceManager::new().expect("Failed to create resource manager");
        let _ = rm.search("?*").expect("Failed to search for devices");
    }
}
