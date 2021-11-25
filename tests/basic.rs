use fake65c02_sys::*;

#[allow(dead_code)]
unsafe extern "C" fn read_memory(_ctx: *mut fake65c02_t, _addr: u16) -> u8 { 0xea }

#[allow(dead_code)]
unsafe extern "C" fn write_memory(_ctx: *mut fake65c02_t, _addr: u16, _value: u8) {}

#[test]
fn new_free() {
    // Note that we have *NO* working memory banks.
    // We are only verifying that it compiles and that our
    // functions are called without segfaulting.
    unsafe {
        let mut ctx = new_fake65c02(std::ptr::null_mut());
        (*ctx).read = read_memory;
        (*ctx).write = write_memory;
        reset65c02(ctx);
        step65c02(ctx);
        free_fake65c02(ctx);
    }
}
