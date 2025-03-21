# NIST VISA library bindings
This crate provides a safe wrapper around the NIST VISA library.  
It is designed to be as simple to use as possible.

## Examples

Here is a simple example that searches for available local devices:
```rust
use libvisa::{error::Error, ResourceManager};

fn main() -> Result<(), Error> {
    // Open the resource manager and search for a matching device
    let manager = ResourceManager::new()?;
    let matches = manager.search("?*")?;

    println!("Found devices:");
    for device in matches {
        println!("  {}", device);
    }

    Ok(())
}
```

## Limitations

Safe wrappers are not yet implemented for several attributes:
- AttributeType::PxiIsExpress
- AttributeType::PxiSlotLwidth
- AttributeType::PxiMaxLwidth
- AttributeType::PxiActualLwidth
- AttributeType::PxiDstarBus
- AttributeType::PxiDstarSet
- AttributeType::PxiAllowWriteCombine
- AttributeType::PxiRecvIntrSeq
- AttributeType::PxiRecvIntrData
- AttributeType::PxiTrigBus
- AttributeType::PxiStarTrigBus
- AttributeType::PxiStarTrigLine
- AttributeType::PxiSrcTrigBus
- AttributeType::PxiDestTrigBus
- AttributeType::JobId
- AttributeType::EventType
- AttributeType::SigpStatusId
- AttributeType::RecvTrigId
- AttributeType::IntrStatusId
- AttributeType::RecvIntrLevel
- AttributeType::OperName
- AttributeType::RecvTcpipAddr
- AttributeType::UserData
- AttributeType::RetCount
- AttributeType::WinBaseAddr
- AttributeType::WinSize
- AttributeType::Is4882Compliant
- AttributeType::TrigId
- AttributeType::WinAccess
- AttributeType::RmSession
- AttributeType::ManfId
- AttributeType::MemSpace
- AttributeType::ModelCode
- AttributeType::Slot
- AttributeType::IntfInstName
- AttributeType::ImmediateServ
- AttributeType::IntfParentNum
