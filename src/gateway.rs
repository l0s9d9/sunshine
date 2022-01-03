use radon::{internal::virtual_allocate, types::{allocation_types::{MEM_COMMIT, MEM_RESERVE}, protection_flags::PAGE_EXECUTE_READWRITE}};

use crate::{Ptr, SunshineError, jump::{AbsoluteJump, Jump}};

pub unsafe fn create_gateway(head: Ptr, size: usize, ret: Ptr) -> crate::Result<Ptr> {
    let gateway = virtual_allocate(
        0,
        size + crate::ABSOLUTE_JUMP_SIZE,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE
    ).map_err(|_| SunshineError::AllocationFailed)? as Ptr;

    gateway.copy_from(head, size);

    AbsoluteJump::write(
        gateway.add(size),
        ret
    )?;
    
    Ok(gateway)
}