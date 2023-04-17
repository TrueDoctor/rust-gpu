#![no_std]

use spirv_std::spirv;
struct B;

#[repr(C)]
struct S {
    x: u32,
    y: B,
    f: u32,
    //f: B,
}

fn f(x: &B) {}

#[spirv(fragment)]
pub fn main() {
    //let s = S { x: 5, y: B, f: B };
    //let s = S { x: 5, y: B };
    let s = S { x: 5, y: B, f: 5 };
    f(&s.y);
}
