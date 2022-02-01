use std::{mem::transmute, sync::Mutex};
use once_cell::sync::Lazy as SyncLazy;

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

pub(crate) static HOOKS: SyncLazy<Mutex<Vec<Hook>>> = SyncLazy::new(|| Mutex::new(Vec::with_capacity(8)));

pub fn create_hook(
    hook_type: HookType,
    p_target: impl AnyFnPtr,
    p_detour: impl AnyFnPtr,
    pp_original: &mut Option<impl AnyFnPtr>,
) -> Result<()> 
{
    let hook = unsafe {
        match hook_type {
            HookType::Absolute => hook_absolute(
                p_target.address() as _,
                p_detour.address() as _,
                transmute(pp_original)
            )?,
            HookType::Compact => hook_compact(
                p_target.address() as _,
                p_detour.address() as _,
                transmute(pp_original)
            )?,
        }
    };

    HOOKS.lock().unwrap().push(hook);

    Ok(())
}

// pub fn enable_hook(p_target: impl AnyFnPtr) -> Result<()> {
//     todo!()
// }

pub fn remove_hook(p_target: impl AnyFnPtr) -> Result<()> {
    let mut lock = HOOKS.lock().unwrap();
    let i = lock.iter().position(|hk| hk.address() == p_target.address()).ok_or(SunshineError::HookNotFound)?;
    unsafe { lock.remove(i).remove()?; }
    Ok(())
}

// pub fn disable_hook(p_target: impl AnyFnPtr) -> Result<()> {
//     todo!()
// }
