mod bindings;
use std::convert::TryInto;
use std::ffi::{CString};
use cty::{c_char};
use bindings::{
    io_connect_t,
    IOConnectCallScalarMethod,
    IOObjectRelease,
    IOServiceGetMatchingService,
    IOServiceMatching,
    IOServiceOpen,
    KERN_SUCCESS,
    kIOMasterPortDefault,
    mach_task_self_
};

const kGetLEDBrightnessID: u32 = 1;

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

#[no_mangle]
pub extern "C" fn getKeyboardBrightness() -> f32 {
    let inputCount: u32 = 1;
    let inputValue: u64 = 0;

    let mut outputCount: u32 = 1;
    // Dangerously assume that we'll never get more than 10 output values.
    let mut outputValues: [u64; 10] = [0; 10];

    let out_brightness: u32;

    let kr = unsafe{IOConnectCallScalarMethod(
        getDataPort(),
        kGetLEDBrightnessID,
        &inputValue,
        inputCount,
        &mut outputValues[0],
        &mut outputCount
    )};

    if kr != KERN_SUCCESS.try_into().unwrap() {
        println!("getKeyboardBrightness() error");
        return 0.0;
    }

    let mut brightness = outputValues[0];
    let fBrightness = (brightness as f32) / (0xfff as f32);
    return fBrightness;
}