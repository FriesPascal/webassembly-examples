extern {
    fn alert(i: i32);
}

#[no_mangle]
pub extern fn alert_sum(a: i32, b: i32) {
    unsafe {
        alert(a + b);
    }
}
