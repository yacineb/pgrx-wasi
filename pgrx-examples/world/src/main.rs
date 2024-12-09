#![allow(non_snake_case)]
#![feature(start)]

#[no_mangle]
pub fn PG_version() {
    println!("Hello I'm pglite");
}


#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {

    0
}