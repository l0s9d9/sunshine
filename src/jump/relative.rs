use radon::types::protection_flags::PAGE_EXECUTE_READWRITE;
use crate::{ProtectionGuard, SunshineError, Ptr};
use super::Jump;

pub struct RelativeJump;
impl Jump for RelativeJump {
    const SIZE: usize = 5;

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

        b"\xE9\x00\x00\x00\x00"
            .as_ptr()
            .copy_to_nonoverlapping(head, Self::SIZE);

        let offset = if crate::IS_X64 {
            
            let offset = dst.offset_from(head.add(Self::SIZE));
            if offset > i32::MAX as _ || offset < i32::MIN as _ {
                return Err(SunshineError::TooFarAway)
            } else {
                offset
            }

        } else {
            dst.offset_from(head.add(Self::SIZE))   
        };

        *(head.add(1) as *mut isize) = offset;

        Ok(())
    }
}