mod bindings;
use std::convert::TryInto;
use std::ffi::{CString};
use cty::{c_char};
use bindings::{
    io_connect_t,
    IOObjectRelease,
    IOServiceGetMatchingService,
    IOServiceMatching,
    IOServiceOpen,
    KERN_SUCCESS,
    kIOMasterPortDefault,
    mach_task_self_
};

#[no_mangle]
pub extern "C" fn print_brightness(val: f32) {
    println!("Current brightness: {}", val)
}

#[no_mangle]
pub extern "C" fn getDataPort() -> io_connect_t {
    let mut dataPort: io_connect_t = 0;

    // Look up a registered IOService object whose class is AppleLMUController
    let lmu_ctrl = CString::new("AppleLMUController").unwrap();
    let serviceObject = unsafe {
        IOServiceGetMatchingService(
            kIOMasterPortDefault,
            IOServiceMatching(lmu_ctrl.as_ptr() as *const c_char)
        )
    };

    if serviceObject == 0 {
        println!("Failed to connect to AppleLMUController");
        return 0;
    }

    // Create a connection to the IOService object
    // TODO: Not sure swapping mach_task_self() for mach_task_self_ was correct...
    let kr = unsafe{IOServiceOpen(serviceObject, mach_task_self_, 0, &mut dataPort)};
    unsafe{IOObjectRelease(serviceObject)};

    if kr != KERN_SUCCESS.try_into().unwrap() {
        println!("Failed to open IoService object\n");
        return 0;
    }
    return dataPort;
}
