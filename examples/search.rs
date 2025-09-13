use libvisa::{error::Error, ResourceManager};

fn main() -> Result<(), Error> {
    // Open the resource manager and search for a matching device
    let manager = ResourceManager::new()?;
    let matches = manager.search("?*")?;

    println!("Found devices:");
    for device in matches {
        println!("  {}", device?);
    }

    Ok(())
}
