use std::mem::transmute;

macro_rules! include_mods {
    ($($mod:ident),*) => {
        $(
            mod $mod;
            #[allow(unused_imports)]
            pub use $mod::*;
        )*
    };
}

include_mods!(guard, any, error, hooks, gateway);

cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        const IS_X64: bool = true;
        const ABSOLUTE_JUMP_SIZE: usize = 13;
        const BITNESS: u32 = 64;
    } else {
        const IS_X64: bool = false;
        const ABSOLUTE_JUMP_SIZE: usize = 5;
        const BITNESS: u32 = 32;
    }
}

pub mod jump;

pub(crate) type Ptr<T = u8> = *mut T;

pub fn create_hook(
    hook_type: HookType,
    p_target: impl AnyFnPtr,
    p_detour: impl AnyFnPtr,
    pp_original: &mut Option<impl AnyFnPtr>,
) -> Result<()> 
{
    unsafe {
        match hook_type {
            HookType::Absolute => hook_absolute(
                p_target.address() as _,
                p_detour.address() as _,
                transmute(pp_original)
            ),
        }
    }
}

// pub fn enable_hook(p_target: impl AnyFnPtr) -> Result<()> {
//     todo!()
// }

pub fn remove_hook(_p_target: impl AnyFnPtr) -> Result<()> {
    todo!()
}

// pub fn disable_hook(p_target: impl AnyFnPtr) -> Result<()> {
//     todo!()
// }
