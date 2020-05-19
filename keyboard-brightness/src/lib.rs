#[no_mangle]
pub extern "C" fn print_brightness(val: f32) {
    println!("Current brightness: {}", val)
}
