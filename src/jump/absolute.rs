use radon::types::protection_flags::PAGE_EXECUTE_READWRITE;
use crate::{Ptr, ProtectionGuard};
use super::Jump;

pub struct AbsoluteJump;
impl Jump for AbsoluteJump {
    const SIZE: usize = crate::ABSOLUTE_JUMP_SIZE;

    unsafe fn write(
        head: Ptr,
        dst: Ptr,
    ) -> crate::Result<()> 
    {
        let _guard = ProtectionGuard::new(
            head,
            Self::SIZE,
            PAGE_EXECUTE_READWRITE
        )?;

        if crate::IS_X64 {
            b"\x49\xBF\x00\x00\x00\x00\x00\x00\x00\x00\x41\x57\xC3"
                .as_ptr()
                .copy_to_nonoverlapping(head, Self::SIZE);

            *(head.add(2) as *mut usize) = dst as _;

        } else {
            b"\xE9\x00\x00\x00\x00"
                .as_ptr()
                .copy_to_nonoverlapping(head, Self::SIZE);
            
            *(head.add(1) as *mut isize) = dst.offset_from(head.add(Self::SIZE));
        }

        Ok(())
    }
}