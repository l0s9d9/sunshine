#![allow(warnings)]

use sunshine::HookType;

static mut ORIGINAL: Option<fn(i32, i32) -> f64> = None;

type Func = unsafe fn(i32, i32) -> f64;

#[no_mangle]
fn target(a: i32, b: i32) -> f64 {
    let c = a + b;
    let d = a * b;
    println!("Value c: {c}, d: {d}");
    
    (c + d) as _
}

#[no_mangle]
unsafe fn detour(a: i32, b: i32) -> f64 {
    println!("Hacked a: {a}, b: {b}");
    ORIGINAL.as_ref().unwrap()(a, b)
}

fn main() {
    unsafe {
        sunshine::create_hook(
            HookType::Absolute,
            target as Func,
            detour as Func,
            &mut ORIGINAL
        );
        target(1, 5);
        sunshine::remove_hook(target as Func);
        target(10, 15);
    }
}
