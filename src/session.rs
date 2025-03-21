//! Session handling for the VISA library
#![expect(
    clippy::cast_possible_truncation,
    reason = "Needed for compatibility with the VISA library"
)]

use crate::{
    attribute::{self, AccessMode},
    bindings,
    error::Error,
    event, ResourceManager,
};
use std::{future, path::Path, vec};

//------------ Async IO signal table -----------------------------------

lazy_static::lazy_static! {
    static ref IO_SIGNAL_TABLE: std::sync::Mutex<std::collections::HashSet<(bindings::ViSession, bindings::ViJobId)>>
        = std::sync::Mutex::new(std::collections::HashSet::new());
}

fn signal_done(session: bindings::ViSession, job_id: bindings::ViJobId) {
    let mut table = IO_SIGNAL_TABLE.lock().unwrap();
    table.insert((session, job_id));
}

fn check_signal_done(session: bindings::ViSession, job_id: bindings::ViJobId) -> bool {
    let mut table = IO_SIGNAL_TABLE.lock().unwrap();
    table.remove(&(session, job_id))
}

//----------------------------------------------------------------------

/// Options for opening a session
#[derive(Debug, Default, Clone, Copy)]
pub struct SessionOptions {
    /// Timeout for opening the session
    pub timeout: std::time::Duration,

    /// Used to acquire an exclusive lock immediately upon opening a session; if a lock cannot be acquired, the session is closed and an error is returned
    pub exclusive_lock: bool,

    /// Used to configure attributes to values specified by some external configuration utility
    /// NI-VISA currently supports `VI_LOAD_CONFIG` only on Serial INSTR sessions.
    pub load_config: bool,
}

/// A session to a resource
#[derive(Debug, Clone, Copy, Default)]
pub struct Session {
    vi: bindings::ViSession,
}
impl Session {
    /// Open a session to a resource
    ///
    /// # Arguments
    /// `rm`: Resource manager, obtained by calling `ResourceManager::open`
    /// `name`: Resource name, obtained by calling `ResourceManager::find_resources`
    /// `mode`: Access mode for the session
    /// `open_timeout`: Timeout for opening the session
    ///
    /// # Errors
    /// Will return an error if the session cannot be opened
    pub fn new(rm: &ResourceManager, name: &str, options: SessionOptions) -> Result<Self, Error> {
        let mut mode = AccessMode::NoLock as u32;
        if options.exclusive_lock {
            mode |= AccessMode::ExclusiveLock as u32;
        }
        if options.load_config {
            mode |= AccessMode::LoadConfig as u32;
        }

        let name = std::ffi::CString::new(name)?;
        let open_timeout = u32::try_from(options.timeout.as_millis())
            .map_err(|_| Error::from_msg("Timeout too large"))?;

        let mut vi = bindings::ViSession::default();
        Error::wrap_binding(None, || unsafe {
            bindings::viOpen(rm.session_id(), name.as_ptr(), mode, open_timeout, &mut vi)
        })?;

        Ok(Self { vi })
    }

    /// Get the raw session identifier
    #[must_use]
    pub fn session_id(&self) -> bindings::ViSession {
        self.vi
    }

    /// Get the resource identifier
    ///
    /// # Errors
    /// Will return an error if the device does not respond to the IDN query
    pub fn idn(&mut self) -> Result<String, Error> {
        self.query("*IDN?")
    }

    /// Get a writer for the session
    #[must_use]
    pub fn writer(&self) -> std::io::BufWriter<Self> {
        std::io::BufWriter::new(*self)
    }

    /// Get a reader for the session
    #[must_use]
    pub fn reader(&self) -> std::io::BufReader<Self> {
        std::io::BufReader::new(*self)
    }

    /// Reads the entire available data from the session into a string
    ///
    /// # Errors
    /// Will return an error if the data cannot be read
    pub fn read_string(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        <Self as std::io::Read>::read_to_string(self, &mut buf)?;
        Ok(buf)
    }

    /// Write a string to the session
    ///
    /// # Errors
    /// Will return an error if the data cannot be written
    pub fn write_string(&mut self, buf: &str) -> Result<(), Error> {
        <Self as std::io::Write>::write(self, buf.as_bytes())?;
        Ok(())
    }

    /// Write a list of strings to the session
    ///
    /// # Errors
    /// Will return an error if the data cannot be written
    pub fn write_all(&mut self, buf: &[&str]) -> Result<(), Error> {
        for s in buf {
            self.write_string(s)?;
        }
        Ok(())
    }

    /// Write a query to the session and parse the response as a value
    ///
    /// # Errors
    /// Will return an error if the query cannot be written or the response cannot be parsed
    pub fn query<T>(&mut self, cmd: &str) -> Result<T, Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.write_string(cmd)?;
        self.read_string()?
            .trim()
            .parse()
            .map_err(|e| Error::from_msg(format!("{cmd}: {e:?}")))
    }

    /// Clear the session
    ///
    /// # Errors
    /// Will return an error if the session cannot be cleared
    pub fn clear(&self) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe { bindings::viClear(self.vi) })
    }

    /// Close the session
    ///
    /// # Errors
    /// Will return an error if the session cannot be closed
    pub fn close(self) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe { bindings::viClose(self.vi) })
    }

    /// Set the size of the read buffer
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_read_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(self.vi, bindings::VI_READ_BUF as u16, size as u32)
        })
    }

    /// Set the size of the write buffer
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_write_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(self.vi, bindings::VI_WRITE_BUF as u16, size as u32)
        })
    }

    /// Set the size of the read/write buffers
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_rw_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(
                self.vi,
                bindings::VI_READ_BUF as u16 | bindings::VI_WRITE_BUF as u16,
                size as u32,
            )
        })
    }

    /// Set the size of the input buffer
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_io_in_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(self.vi, bindings::VI_IO_IN_BUF as u16, size as u32)
        })
    }

    /// Set the size of the output buffer
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_io_out_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(self.vi, bindings::VI_IO_OUT_BUF as u16, size as u32)
        })
    }

    /// Set the size of the input/output buffers
    ///
    /// # Errors
    /// Will return an error if the buffer size cannot be set
    pub fn set_io_buffer(&self, size: usize) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetBuf(
                self.vi,
                bindings::VI_IO_IN_BUF as u16 | bindings::VI_IO_OUT_BUF as u16,
                size as u32,
            )
        })
    }

    /// Writes data to the write buffer instead of sending it immediately
    ///
    /// # Errors
    /// Will return an error if the data cannot be written
    pub fn buffer_write(&self, buf: &[u8]) -> Result<(), Error> {
        let mut written = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viBufWrite(self.vi, buf.as_ptr(), buf.len() as u32, &mut written)
        })?;
        Ok(())
    }

    /// Reads data from the read buffer instead of receiving it immediately
    ///
    /// # Errors
    /// Will return an error if the data cannot be read
    pub fn buffer_read(&self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0u8; len];
        let mut read = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viBufRead(self.vi, buf.as_mut_ptr(), len as u32, &mut read)
        })?;
        buf.resize(read as usize, 0);
        Ok(buf)
    }

    /// Manually flushes the specified buffers associated with formatted I/O operations and/or serial communication.
    ///
    /// # Errors
    /// Will return an error if the buffer cannot be flushed
    pub fn flush_read_buffer(&self, discard: bool) -> Result<(), Error> {
        self.flush_inner(if discard {
            bindings::VI_READ_BUF_DISCARD
        } else {
            bindings::VI_READ_BUF
        })
    }

    /// Manually flushes the specified buffers associated with formatted I/O operations and/or serial communication.
    ///
    /// # Errors
    /// Will return an error if the buffer cannot be flushed
    pub fn flush_write_buffer(&self, discard: bool) -> Result<(), Error> {
        self.flush_inner(if discard {
            bindings::VI_WRITE_BUF_DISCARD
        } else {
            bindings::VI_WRITE_BUF
        })
    }

    /// Manually flushes the specified buffers associated with formatted I/O operations and/or serial communication.
    ///
    /// # Errors
    /// Will return an error if the buffer cannot be flushed
    pub fn flush_io_in_buffer(&self, discard: bool) -> Result<(), Error> {
        self.flush_inner(if discard {
            bindings::VI_IO_IN_BUF_DISCARD
        } else {
            bindings::VI_IO_IN_BUF
        })
    }

    /// Manually flushes the specified buffers associated with formatted I/O operations and/or serial communication.
    ///
    /// # Errors
    /// Will return an error if the buffer cannot be flushed
    pub fn flush_io_out_buffer(&self, discard: bool) -> Result<(), Error> {
        self.flush_inner(if discard {
            bindings::VI_IO_OUT_BUF_DISCARD
        } else {
            bindings::VI_IO_OUT_BUF
        })
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn flush_inner(&self, mask: u32) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viFlush(self.vi, mask as u16)
        })
    }

    /// The `viAssertTrigger()` operation sources a software or hardware trigger dependent on the interface type.
    ///
    /// Software Triggers for 488.2 Instruments (GPIB, VXI, TCPIP, and USB)  
    /// This operation sends an IEEE-488.2 software trigger to the addressed device. For software triggers, `VI_TRIG_PROT_DEFAULT` is the only valid protocol.  
    /// The bus-specific details are:
    ///
    /// For a GPIB device, VISA addresses the device to listen and then sends the GPIB GET command.  
    /// For a VXI device, VISA sends the Word Serial Trigger command.  
    /// For a USB device, VISA sends the TRIGGER message ID on the Bulk-OUT pipe.  
    /// Software Triggers for Non-488.2 Instruments (Serial INSTR, TCPIP SOCKET, and USB RAW)  
    /// If `VI_ATTR_IO_PROT` is `VI_PROT_4882_STRS`, this operations sends "*TRG\n" to the device; otherwise, this operation is not valid.  
    /// For software triggers, `VI_TRIG_PROT_DEFAULT` is the only valid protocol.  
    ///
    /// Hardware Triggering for VXI  
    /// For hardware triggers to VXI instruments, `VI_ATTR_TRIG_ID` must first be set to the desired trigger line to use;  
    /// this operation performs the specified trigger operation on the previously selected trigger line.  
    /// For VXI hardware triggers, `VI_TRIG_PROT_DEFAULT` is equivalent to `VI_TRIG_PROT_SYNC`.
    ///
    /// Trigger Reservation for PXI  
    /// For PXI instruments, this operation reserves or releases (unreserves) a trigger line for use in external triggering.  
    /// For PXI triggers, `VI_TRIG_PROT_RESERVE` and `VI_TRIG_PROT_UNRESERVE` are the only valid protocols.
    ///
    /// # Errors
    /// Will return an error if the trigger cannot be asserted
    pub fn assert_trigger(&self, protocol: TriggerProtocol) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viAssertTrigger(self.vi, protocol as u16)
        })
    }

    /// Status Bytes for 488.2 Instruments (GPIB, VXI, TCPIP, and USB)  
    /// This operation reads a service request status from a message-based device. The bus-specific details are:
    ///
    /// For a GPIB device, the status is read by serial polling the device.  
    /// For a VXI device, VISA sends the Word Serial Read STB query.  
    /// For a USB device, this function sends the `READ_STATUS_BYTE` command on the control pipe.  
    /// Status Bytes for Non-488.2 Instruments (Serial INSTR, TCPIP SOCKET, and USB RAW)  
    /// A message is sent in response to a service request to retrieve status information.  
    /// If `VI_ATTR_IO_PROT` is `VI_PROT_4882_STRS`, the device is sent the string "*STB?\n", and then the device's status byte is read;  
    /// Otherwise, this operation is not valid.
    ///
    /// Although the status output is a 16-bit value, the upper 8 bits are always 0. The lower 8 bits contain the actual status byte.  
    /// For 488.2 instruments, this is the 488.2-defined status byte.
    ///
    /// The IEEE 488.2 standard defines several bit assignments in the status byte. For example, if bit 6 of the status is set, the device is requesting service.  
    /// In addition to setting bit 6 when requesting service, 488.2 devices also use two other bits to specify their status.  
    /// Bit 4, the Message Available bit (MAV), is set when the device is ready to send previously queried data.  
    /// Bit 5, the Event Status bit (ESB), is set if one or more of the enabled 488.2 events occurs.  
    /// These events include power-on, user request, command error, execution error, device dependent error, query error, request control, and operation complete.  
    /// The device can assert SRQ when ESB or MAV are set, or when a manufacturer-defined condition occurs.  
    /// Manufacturers of 488.2 devices use the remaining lower-order bits to communicate the reason for the service request or to summarize the device state.
    ///
    /// # Errors
    /// Will return an error if the status byte cannot be read
    pub fn read_status(&self) -> Result<u8, Error> {
        let mut status: u16 = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viReadSTB(self.vi, &mut status)
        })?;

        // Upper 8 bits are always 0, safe to cast to u8
        Ok(status as u8)
    }

    //=========================================================================
    // Attributes
    //=========================================================================

    /// Read an attribute from the session
    /// See the `attribute` module for a list of available attributes
    ///
    /// # Example
    /// ```ignore
    /// let name: RsrcName = session.get_attribute()?;
    /// ```
    ///
    /// # Errors
    /// Will return an error if the attribute cannot be read
    pub fn get_attribute<T>(&self) -> Result<T::Value, Error>
    where
        T: attribute::AsViReadable,
    {
        unsafe { self.get_attribute_raw(T::VI_ATTR) }
    }

    /// Set an attribute for the session
    /// See the `attribute` module for a list of available attributes
    ///
    /// # Example
    /// ```ignore
    /// session.set_attribute(SendEndEn(true))?;
    /// ```
    ///
    /// # Errors
    /// Will return an error if the attribute cannot be set
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_attribute<T>(&self, attribute: T) -> Result<(), Error>
    where
        T: attribute::AsViReadable + attribute::AsViWritable,
    {
        let value = attribute.as_vi();
        unsafe { self.set_attribute_raw(T::VI_ATTR, value) }
    }

    /// Get an attribute from the session
    /// See the `attribute` module for a list of available attributes
    ///
    /// # Safety
    /// This function is unsafe because it does not check the validity of the attribute
    /// being read. Use `get_attribute` instead.
    ///
    /// # Errors
    /// Will return an error if the attribute cannot be read
    pub unsafe fn get_attribute_raw<T>(&self, attr: bindings::ViAttr) -> Result<T, Error> {
        let mut value: T = std::mem::zeroed::<T>();
        Error::wrap_binding(Some(self.vi), || unsafe {
            let value = (&raw mut value).cast::<std::ffi::c_void>();
            bindings::viGetAttribute(self.vi, attr, value)
        })?;
        Ok(value)
    }

    /// Set an attribute for the session
    /// See the `attribute` module for a list of available attributes
    ///
    /// # Safety
    /// This function is unsafe because it does not check the validity of the attribute
    /// or the value being set. Use `set_attribute` instead.
    ///
    /// # Errors
    /// Will return an error if the attribute cannot be set
    pub unsafe fn set_attribute_raw(
        &self,
        attr: bindings::ViAttr,
        value: bindings::ViAttrState,
    ) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viSetAttribute(self.vi, attr, value)
        })
    }

    //=========================================================================
    // Session locking
    //=========================================================================

    /// Create an exclusive lock on the session
    ///
    /// # Errors
    /// Will return an error if the lock cannot be acquired
    pub fn lock(&self, lock_timeout: std::time::Duration) -> Result<(), Error> {
        let lock_timeout = lock_timeout.as_millis() as u32;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viLock(
                self.vi,
                bindings::VI_EXCLUSIVE_LOCK,
                lock_timeout,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        })
    }

    /// Create a shared lock on the session
    ///
    /// # Errors
    /// Will return an error if the lock cannot be acquired
    pub fn lock_shared(
        &self,
        lock_timeout: std::time::Duration,
        requested_key: &str,
    ) -> Result<String, Error> {
        let lock_timeout = lock_timeout.as_millis() as u32;
        let requested_key = std::ffi::CString::new(requested_key)?;
        let mut actual_key = vec![std::ffi::c_char::default(); 256];

        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viLock(
                self.vi,
                bindings::VI_SHARED_LOCK,
                lock_timeout,
                requested_key.as_ptr(),
                actual_key.as_mut_ptr(),
            )
        })?;

        //
        // Turn the c_char vec back into a string
        let key = unsafe { std::ffi::CStr::from_ptr(actual_key.as_ptr()) }
            .to_str()
            .map_err(|_| Error::from_msg("Invalid lock key"))?;
        Ok(key.to_string())
    }

    /// Unlock the session
    ///
    /// # Errors
    /// Will return an error if the session cannot be unlocked
    pub fn unlock(&self) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe { bindings::viUnlock(self.vi) })
    }

    //=========================================================================
    // Event handling
    //=========================================================================

    /// The `viEnableEvent()` operation enables notification of an event identified by the eventType parameter for mechanisms specified in the mechanism parameter.  
    /// The specified session can be enabled to queue events by specifying `VI_QUEUE`.  
    /// Applications can enable the session to invoke a callback function to execute the handler by specifying `VI_HNDLR`.  
    /// The applications are required to install at least one handler to be enabled for this mode.  
    /// Specifying `VI_SUSPEND_HNDLR` enables the session to receive callbacks, but the invocation of the handler is deferred to a later time.  
    /// Successive calls to this operation replace the old callback mechanism with the new callback mechanism.  
    /// Specifying `VI_ALL_ENABLED_EVENTS` for the eventType parameter refers to all events which have previously been enabled on this session,  
    /// making it easier to switch between the two callback mechanisms for multiple events.  
    /// NI-VISA does not support enabling both the queue and the handler for the same event type on the same session.  
    /// If you need to use both mechanisms for the same event type, you should open multiple sessions to the resource.
    ///
    /// # Errors
    /// Will return an error if the event cannot be enabled
    pub fn enable_event(
        &self,
        event_type: event::Event,
        mechanism: event::HandlingMechanism,
        filter: bindings::ViEventFilter,
    ) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viEnableEvent(self.vi, event_type as u32, mechanism as u16, filter)
        })
    }

    /// The `viDisableEvent()` operation disables servicing of an event identified by the eventType parameter for the mechanisms specified in the mechanism parameter.  
    /// This operation prevents new event occurrences from being added to the queue(s). However, event occurrences already existing in the queue(s) are not flushed.  
    /// Use `viDiscardEvents()` if you want to discard events remaining in the queue(s).  
    /// Specifying `VI_ALL_ENABLED_EVENTS` for the eventType parameter allows a session to stop receiving all events.  
    /// The session can stop receiving queued events by specifying `VI_QUEUE`.  
    /// Applications can stop receiving callback events by specifying either `VI_HNDLR` or `VI_SUSPEND_HNDLR`.  
    /// Specifying `VI_ALL_MECH` disables both the queuing and callback mechanisms.
    ///
    /// # Errors
    /// Will return an error if the event cannot be disabled
    pub fn disable_event(
        &self,
        event_type: event::Event,
        mechanism: event::HandlingMechanism,
    ) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viDisableEvent(self.vi, event_type as u32, mechanism as u16)
        })
    }

    /// The `viDiscardEvents()` operation discards all pending occurrences of the specified event types and mechanisms from the specified session.  
    /// Specifying `VI_ALL_ENABLED_EVENTS` for the eventType parameter discards events of every type that is enabled for the given session.  
    /// The information about all the event occurrences which have not yet been handled is discarded.  
    /// This operation is useful to remove event occurrences that an application no longer needs.  
    /// The discarded event occurrences are not available to a session at a later time.  
    /// This operation does not apply to event contexts that have already been delivered to the application.
    ///
    /// # Errors
    /// Will return an error if the events cannot be discarded
    pub fn discard_events(
        &self,
        event_type: event::Event,
        mechanism: event::HandlingMechanism,
    ) -> Result<(), Error> {
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viDiscardEvents(self.vi, event_type as u32, mechanism as u16)
        })
    }

    /// The `viInstallHandler()` operation allows applications to install handlers on sessions.  
    /// You can use the `define_event_handler` macro to define a handler if you don't need to pass user data.
    ///
    /// The handler specified in the handler parameter is installed along with any previously installed handlers for the specified event.  
    /// Applications can specify a value in the userHandle parameter that is passed to the handler on its invocation.  
    /// VISA identifies handlers uniquely using the handler reference and this value.  
    /// VISA allows applications to install multiple handlers for an eventType on the same session.  
    /// You can install multiple handlers through multiple invocations of the `viInstallHandler()` operation,  
    /// where each invocation adds to the previous list of handlers.  
    /// If more than one handler is installed for an eventType, each of the handlers is invoked on every occurrence of the specified event(s).  
    /// VISA specifies that the handlers are invoked in Last In First Out (LIFO) order.
    ///
    /// # Errors
    /// Will return an error if the handler cannot be installed
    pub fn add_event_handler<'data, H: event::HandlerWithData>(
        &'data self,
        event_type: event::Event,
        user_data: Option<&'data mut H::Data>,
    ) -> Result<(), Error> {
        let context = match user_data {
            Some(mut user_data) => {
                let data = &raw mut user_data;
                data.cast::<std::ffi::c_void>()
            }
            None => std::ptr::null_mut(),
        };

        let handler = H::into();
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viInstallHandler(self.vi, event_type as u32, handler, context)
        })
    }

    /// The `viUninstallHandler()` operation allows applications to uninstall handlers for events on sessions.  
    /// Applications should also specify the value in the userHandle parameter that was passed while installing the handler.  
    /// VISA identifies handlers uniquely using the handler reference and this value.  
    /// All the handlers, for which the handler reference and the value matches, are uninstalled.  
    /// Specifying `VI_ANY_HNDLR` as the value for the handler parameter causes the operation to uninstall all  
    /// the handlers with the matching value in the userHandle parameter.
    ///
    /// # Errors
    /// Will return an error if the handler cannot be removed
    pub fn remove_event_handler<'data, H: event::HandlerWithData>(
        &'data self,
        event_type: event::Event,
        user_data: Option<&'data mut H::Data>,
    ) -> Result<(), Error> {
        let context = match user_data {
            Some(mut user_data) => {
                let data = &raw mut user_data;
                data.cast::<std::ffi::c_void>()
            }
            None => std::ptr::null_mut(),
        };

        let handler = H::into();
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viUninstallHandler(self.vi, event_type as u32, handler, context)
        })
    }

    /// Waits for an occurrence of the specified event for a given session.  
    /// The `viWaitOnEvent()` operation suspends the execution of a thread of an application and waits for an event of the  
    /// type specified by inEventType for a time period specified by timeout.  
    /// You can wait only for events that have been enabled with the `viEnableEvent()` operation.  
    /// Refer to individual event descriptions for context definitions.  
    /// If the specified inEventType is `VI_ALL_ENABLED_EVENTS`, the operation waits for any event that is enabled for the given session.  
    /// If the specified timeout value is `VI_TMO_INFINITE`, the operation is suspended indefinitely.  
    /// If the specified timeout value is `VI_TMO_IMMEDIATE`, the operation is not suspended; therefore, this value can be used to dequeue events from an event queue.  
    /// When the outContext handle returned from a successful invocation of `viWaitOnEvent()` is no longer needed, it should be passed to `viClose()`.  
    /// If a session's event queue becomes full and a new event arrives, the new event is discarded.  
    /// The default event queue size (per session) is 50, which is sufficiently large for most  applications.  
    /// If an application expects more than 50 events to arrive without having been handled, it can modify the value of the  
    /// attribute `VI_ATTR_MAX_QUEUE_LENGTH` to the required size.
    ///
    /// # Errors
    /// Will return an error if the event cannot be waited on
    pub fn wait_on_event(
        &self,
        in_event_type: event::Event,
        timeout: std::time::Duration,
    ) -> Result<bindings::ViEvent, Error> {
        let mut context: bindings::ViEvent = bindings::ViEvent::default();
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viWaitOnEvent(
                self.vi,
                in_event_type as u32,
                timeout.as_millis() as u32,
                std::ptr::null_mut(),
                &mut context,
            )
        })?;

        Ok(context)
    }

    //=========================================================================
    // Async and file I/O
    //=========================================================================

    /// Create an asynchronous read task that can be awaited or terminated
    ///
    /// # Errors
    /// Will return an error if the data cannot be read from the session
    pub fn read_async(
        &self,
        bytes: usize,
        timeout: std::time::Duration,
    ) -> Result<AsyncTask, Error> {
        let mut jobid = bindings::ViJobId::default();
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viReadAsync(self.vi, std::ptr::null_mut(), bytes as u32, &mut jobid)
        })?;

        self.add_event_handler::<AsyncTask>(event::Event::IoCompletion, Some(&mut jobid))?;

        Ok(AsyncTask {
            session: self.vi,
            job_id: jobid,
            started: std::time::Instant::now(),
            timeout,
        })
    }

    /// Create an asynchronous write task that can be awaited or terminated
    ///
    /// # Errors
    /// Will return an error if the data cannot be written to the session
    pub fn write_async(
        &self,
        buf: &[u8],
        timeout: std::time::Duration,
    ) -> Result<AsyncTask, Error> {
        let mut jobid = bindings::ViJobId::default();
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viWriteAsync(self.vi, buf.as_ptr(), buf.len() as u32, &mut jobid)
        })?;

        self.add_event_handler::<AsyncTask>(event::Event::IoCompletion, Some(&mut jobid))?;

        Ok(AsyncTask {
            session: self.vi,
            job_id: jobid,
            started: std::time::Instant::now(),
            timeout,
        })
    }

    /// Take data from a file and write it out synchronously.
    /// If size is None, the entire file is read.
    ///
    /// # Errors
    /// Will return an error if the data cannot be written to the session
    pub fn write_from_file(&self, filename: &Path, size: Option<usize>) -> Result<(), Error> {
        let size = if let Some(size) = size {
            size
        } else {
            std::fs::metadata(filename)?.len() as usize
        };

        let filename = filename.to_string_lossy();
        let filename = std::ffi::CString::new(filename.as_bytes())?;

        let mut written = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viWriteFromFile(self.vi, filename.as_ptr(), size as u32, &mut written)
        })
    }

    /// Take data from the session and write it out to a file
    ///
    /// # Errors
    /// Will return an error if the data cannot be written to the file
    pub fn read_to_file(&self, filename: &Path, size: usize) -> Result<(), Error> {
        let filename = filename.to_string_lossy();
        let filename = std::ffi::CString::new(filename.as_bytes())?;

        let mut written = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viReadToFile(self.vi, filename.as_ptr(), size as u32, &mut written)
        })
    }
}

impl std::io::Read for Session {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let max_bytes = buf.len();

        let mut bytes_read = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viRead(self.vi, buf.as_mut_ptr(), max_bytes as u32, &mut bytes_read)
        })
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(bytes_read as usize)
    }
}

impl std::io::Write for Session {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut bytes_written = 0;
        Error::wrap_binding(Some(self.vi), || unsafe {
            bindings::viWrite(self.vi, buf.as_ptr(), buf.len() as u32, &mut bytes_written)
        })
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(bytes_written as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Trigger protocols for `assert_trigger`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerProtocol {
    /// Valid for GPIB, Serial, TCPIP, USB, VXI
    Default = bindings::VI_TRIG_PROT_DEFAULT as u16,

    /// Valid for VXI only
    On = bindings::VI_TRIG_PROT_ON as u16,

    /// Valid for VXI only
    Off = bindings::VI_TRIG_PROT_OFF as u16,

    /// Valid for VXI only
    Sync = bindings::VI_TRIG_PROT_SYNC as u16,

    /// Valid for PXI only
    Reserve = bindings::VI_TRIG_PROT_RESERVE as u16,

    /// Valid for PXI only
    Unreserve = bindings::VI_TRIG_PROT_UNRESERVE as u16,
}

/// An asynchronous task that can be awaited or terminated
#[derive(Debug)]
pub struct AsyncTask {
    session: bindings::ViSession,
    job_id: bindings::ViJobId,
    started: std::time::Instant,
    timeout: std::time::Duration,
}
impl AsyncTask {
    /// Terminate the async task before it completes
    /// This will cause the task to return `VI_ERROR_ABORT`
    ///
    /// # Errors
    /// Will return an error if the task cannot be terminated
    pub fn terminate(mut self) -> Result<(), Error> {
        self.terminate_task()
    }

    fn terminate_task(&mut self) -> Result<(), Error> {
        // Terminate running task
        Error::wrap_binding(Some(self.session), || unsafe {
            bindings::viTerminate(self.session, 0, self.job_id)
        })?;

        // Remove event handler
        let context = (&raw mut self.job_id).cast::<std::ffi::c_void>();
        let handler = <Self as event::HandlerWithData>::into();
        Error::wrap_binding(Some(self.session), || unsafe {
            bindings::viUninstallHandler(
                self.session,
                event::Event::IoCompletion as u32,
                handler,
                context,
            )
        })
    }
}
impl future::Future for AsyncTask {
    type Output = Result<(), Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        if check_signal_done(this.session, this.job_id) {
            std::task::Poll::Ready(Ok(()))
        } else if this.started.elapsed() > this.timeout {
            std::task::Poll::Ready(Err(Error {
                status: crate::error::ErrorType::Tmo,
                description: Some("Async IO Timeout".to_string()),
            }))
        } else {
            std::task::Poll::Pending
        }
    }
}
impl event::HandlerWithData for AsyncTask {
    type Data = bindings::ViJobId;
    fn handle(
        session: bindings::ViSession,
        _: event::Event,
        _: bindings::ViEvent,
        job_id: &Self::Data,
    ) -> Result<(), Error> {
        signal_done(session, *job_id);
        Ok(())
    }
}
impl Drop for AsyncTask {
    fn drop(&mut self) {
        self.terminate_task().ok();
    }
}

/// Formats a string and sends it to the session device
/// For options, see <https://www.ni.com/docs/en-US/bundle/ni-visa-api-ref/page/ni-visa-api-ref/viprintf.html>
///
/// **IMPORTANT** %s expects null terminated c-strings!
#[macro_export]
macro_rules! printf {
    ($session:expr, $format:expr, $($arg:expr),* $(,)?) => {
        {
            let session: &$crate::Session = $session;
            let format: &str = $format;

            match std::ffi::CString::new(format) {
                Ok(cstr) => {
                    $crate::error::Error::wrap_binding(Some(session.session_id()), || $crate::variadic_unsafe_nightmare_spaghetti!(
                        $crate::bindings::viPrintf,
                        args = [session.session_id(), cstr.as_ptr()],
                        va_args = [$($arg),*]
                    ))
                },
                Err(e) => Err(e.into())
            }
        }
    };
}

/// Reads formatted data from the session device
/// For options, see <https://www.ni.com/docs/en-US/bundle/ni-visa-api-ref/page/ni-visa-api-ref/viscanf.html>
///
/// **IMPORTANT** %s expects null terminated c-strings!
#[macro_export]
macro_rules! scanf {
    ($session:expr, $format:expr, $($arg:path),*) => {
        {
            let session: &$crate::Session = $session;
            let format: &str = $format;

            match std::ffi::CString::new(format) {
                Ok(cstr) => {
                    $crate::error::Error::wrap_binding(Some(session.session_id()), || $crate::variadic_unsafe_nightmare_spaghetti!(
                        $crate::bindings::viScanf,
                        args = [session.session_id(), cstr.as_ptr()],
                        va_args = [$(&mut $arg),*]
                    ))
                },
                Err(e) => Err(e.into())
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macros() {
        let rm = ResourceManager::new().unwrap();
        let session = Session::new(&rm, "GPIB0::1::INSTR", SessionOptions::default()).unwrap();

        printf!(&session, "Hello, World! %d %d", 5, 6).unwrap();

        let mut id = 0;
        scanf!(&session, "%d", id).unwrap();
    }
}
