use libvisa::{attribute, error::Error, ResourceManager, Session};
use std::time::Duration;

fn main() -> Result<(), Error> {
    // Open the resource manager and search for a matching device
    let manager = ResourceManager::new()?;
    let mut matches = manager.search("USB?*INSTR")?;
    let resource = matches.next().ok_or(Error::default())?;

    // Open a session to the device, set a timeout
    let mut session = Session::new(&manager, resource.as_ref(), Default::default())?;
    session.set_attribute(attribute::TmoValue(Duration::from_millis(500)))?;

    // Process IDN string
    // This will be a comma separated list in the form
    // [IGNORE],[MODEL],[SERIAL?]
    session.write_string("*IDN?")?;
    let idn = session.read_string()?;
    let idn = idn.split(',').collect::<Vec<_>>();
    let model = idn.get(1).ok_or(Error::default())?;
    let serial = idn.get(2);

    // Scope specific commands
    session.write_all(&["*RST", "*CLS", ":AUTOSCALE"])?;

    println!("Model: {}", model);
    if let Some(serial) = serial {
        println!("Serial: {}", serial);
    }

    let task = session.read_async(256, std::time::Duration::from_secs(1))?;
    task.terminate()?;

    Ok(())
}
