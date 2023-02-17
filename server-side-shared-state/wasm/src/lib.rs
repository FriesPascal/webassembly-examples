#[link(wasm_import_module = "counter")]
extern {
    fn inc() -> i32;
}

#[no_mangle]
pub extern fn handle_request(count: i32) -> i32 {
    unsafe {
        for _ in 0..count-1 { inc(); }
        inc()
    }
}
