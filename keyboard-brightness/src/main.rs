#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryInto;
use std::env;
use std::ffi::{CString};
use cty::{c_char};

const kGetLEDBrightnessID: u32 = 1;
const kSetLEDBrightnessID: u32 = 2;

fn getDataPort() -> io_connect_t {
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
    let kr = unsafe{IOServiceOpen(serviceObject, mach_task_self_, 0, &mut dataPort)};
    unsafe{IOObjectRelease(serviceObject)};

    if kr != KERN_SUCCESS.try_into().unwrap() {
        println!("Failed to open IoService object\n");
        return 0;
    }
    return dataPort;
}

fn getKeyboardBrightness() -> f32 {
    let inputCount: u32 = 1;
    let inputValue: u64 = 0;

    let mut outputCount: u32 = 1;
    // Dangerously assume that we'll never get more than 10 output values.
    let mut outputValues: [u64; 10] = [0; 10];

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

    let brightness = outputValues[0];
    let fBrightness = (brightness as f32) / (0xfff as f32);
    return fBrightness;
}

fn setKeyboardBrightness(new_brightness: f32) {
    println!("Setting brightness: {}", new_brightness);

    let inputCount: u32 = 2;
    let inputValues: [u64; 2] = [
        0, // Unknown input
        (new_brightness * (0xfff as f32)) as u64
    ];

    let mut outputCount: u32 = 1;
    // Dangerously assume that we'll never get more than 10 output values.
    let mut outputValues: [u64; 10] = [0; 10];

    let kr = unsafe{IOConnectCallScalarMethod(
        getDataPort(),
        kSetLEDBrightnessID,
        &inputValues[0],
        inputCount,
        &mut outputValues[0],
        &mut outputCount
    )};

    if kr != KERN_SUCCESS.try_into().unwrap() {
        println!("setKeyboardBrightness() error");
        return;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: kbrightness [new brightness]");
        return;
    }
    if args.len() == 1 {
        println!("Current brightness: {}", getKeyboardBrightness());
        return;
    }
    let brightness = &args[1];
    match brightness.parse::<f32>(){
        Ok(b) => setKeyboardBrightness(b),
        Err(e) => println!("error parsing argument: {}", e),
    }
}