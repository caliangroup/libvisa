//! TCPIP related attributes
use crate::bindings;

impl_attr!(
    "Specifies `HiSLIP` overlap mode"
    TcpipHislipOverlapEn(bool)
);

impl_attr!(
    "Hislip version"
    TcpipHislipVersion(ReadOnlyU32)
);

impl_attr!(
    "Maximum message size in bytes"
    TcpipHislipMaxMessageKb(u32)
);

impl_attr!(
    "Specifies whether the device is a `HiSLIP` device"
    TcpipIsHislip(ReadOnlyBool)
);

impl_attr!(
    "This is the TCPIP address of the device to which the session is connected. This string is formatted in dot notation."
    TcpipAddr(ReadOnlyString)
);

impl_attr!(
    "This specifies the host name of the device. If no host name is available, this attribute returns an empty string."
    TcpipHostname(ReadOnlyString)
);

impl_attr!(
    "This specifies the port number for a given TCPIP address. For a TCPIP SOCKET Resource, this is a required part of the address string."
    TcpipPort(ReadOnlyU16)
);

impl_attr!(
    "This specifies the LAN device name used by the VXI-11 or LXI protocol during connection."
    TcpipDeviceName(ReadOnlyString)
);

impl_attr!(
    "The Nagle algorithm is disabled when this attribute is enabled (and vice versa)"
    "The Nagle algorithm improves network performance by buffering \"send\" data until a full-size packet can be sent"
    "This attribute is enabled by default in VISA to verify that synchronous writes get flushed immediately."
    TcpipNodelay(bool)
);

impl_attr!(
    "Setting this attribute to TRUE requests that a TCP/IP provider enable the use of keep-alive packets on TCP connections."
    "After the system detects that a connection was dropped, VISA returns a lost connection error code on subsequent I/O calls on the session."
    "The time required for the system to detect that the connection was dropped is dependent on the system and is not settable."
    TcpipKeepalive(bool)
);
