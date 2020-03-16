use std::io;
use structopt::StructOpt;
use sysfs_class::{PciDevice, PciDriver, SysClass};

#[derive(Debug, StructOpt)]
enum Opt {
    List,
    Bind { device_id: String, driver_id: String },
    Unbind { device_id: String },
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::List => list(),
        Opt::Bind { device_id, driver_id } => bind(device_id, driver_id),
        Opt::Unbind { device_id } => unbind(device_id),
    }
}


fn unbind(id: String) -> io::Result<()> {
    if let Some(dev) = find_device(id.trim())? {
        let drv = dev.driver()?;
        unsafe { drv.unbind(&dev)? };
        println!("drv {} unbinded from {}", drv.id(), dev.id());
        return Ok(())
    }
    println!("device not found");
    Ok(())
}

fn bind(dev_id: String, drv_id: String) -> io::Result<()> {
    let dev = find_device(dev_id.trim())?;
    if dev.is_none() {
        println!("Device {} not found", dev_id);
        return Ok(())
    }

    let drv = find_driver(drv_id.trim())?;
    if drv.is_none() {
        println!("Driver {} not found", drv_id);
        return Ok(())
    }

    unsafe { drv.unwrap().bind(&dev.unwrap())?; }

    println!("Device {} binded to the driver {}", dev_id, drv_id);
    Ok(())
}


fn find_device(target_id: &str) -> io::Result<Option<PciDevice>> {
   for dev in PciDevice::iter() {
        let dev = dev?;
        if dev.id() == target_id {
            return Ok(Some(dev))
        }
   }

   Ok(None)
}

fn find_driver(target_id: &str) -> io::Result<Option<PciDriver>> {
    for drv in PciDriver::iter() {
        let drv = drv?;
        if drv.id() == target_id {
            return Ok(Some(drv))
        }
    }

    return Ok(None)
}

fn list() -> io::Result<()> {
    println!("Devices:");
    for dev in PciDevice::all()? {
        println!("\tPCI Device: {}", dev.id());
        if let Ok(drv) = dev.driver() {
            println!("\t\tDriver: {}", drv.id());
        }
    }
    println!("Drivers:");
    for drv in PciDriver::iter() {
        let drv = drv?;
        println!("\tPCI Driver: {}", drv.id());
    }

    Ok(())
}
