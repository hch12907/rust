// check-fail

#![feature(intrinsics)]

fn a() {
    let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
    //~^ ERROR cannot coerce
}

fn b() {
    let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
    //~^ ERROR casting
}

fn c() {
    let _ = [
        std::intrinsics::copy_nonoverlapping::<i32>,
        std::intrinsics::copy::<i32>,
        //~^ ERROR cannot coerce
    ];
}

fn main() {}
