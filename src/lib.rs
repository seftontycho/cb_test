use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn inc(a: c_int, cb: extern "C" fn(c_int) -> ()) {
    println!("Hello from Rust, a = {}", a);
    cb(a + 1)
}
