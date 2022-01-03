#![feature(naked_functions)]

use sunshine::HookType;

static mut ORIGINAL: Option<unsafe extern "system" fn(i32, i32) -> f64> = None;

type Func = unsafe extern "system" fn(i32, i32) -> f64;

#[no_mangle]
unsafe extern "system" fn target(a: i32, b: i32) -> f64 {
    let c = a + b;
    let d = a * b;
    println!("Value c: {c}, d: {d}");
    
    (c + d) as _
}

#[no_mangle]
unsafe extern "system" fn detour(a: i32, b: i32) -> f64 {
    println!("Hacked a: {a}, b: {b}");
    ORIGINAL.as_ref().unwrap()(a, b)
}

fn main() {
    unsafe {
        sunshine::create_hook(
            HookType::Compact,
            target as Func,
            detour as Func,
            &mut ORIGINAL
        ).unwrap();

        target(1, 5);

        sunshine::remove_hook(target as Func).unwrap();
        target(10, 15);
    }
}
