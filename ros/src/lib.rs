#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn rust_main() {
    let x = b"Hello World!";
    let color = 0x1f;
    let mut hello_color = [color; 24];
    for( i, char_byte ) in x.into_iter().enumerate(){
        hello_color[i*2] = *char_byte;
    }
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe{ *buffer_ptr = hello_color };
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
