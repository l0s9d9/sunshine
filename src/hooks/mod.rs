use radon::{types::{protection_flags::PAGE_EXECUTE_READWRITE, free_types::MEM_RELEASE}, internal::virtual_free};
use crate::{Ptr, ProtectionGuard, SunshineError};

mod absolute;
pub(crate) use absolute::*;

pub enum HookType {
    Absolute,
    // Compact,
    // Midfunc
}

pub(crate) enum Hook {
    Absolute {
        gateway: usize,
        p_target: usize,
        stolen_size: usize
    }
}

impl Hook {
    pub fn address(&self) -> usize {
        match self {
            Hook::Absolute { p_target, .. } => *p_target,
        }
    }

    pub unsafe fn remove(self) -> crate::Result<()> {
        match self {
            Hook::Absolute { gateway, p_target, stolen_size } => {
                let _guard = ProtectionGuard::new(p_target as _, stolen_size, PAGE_EXECUTE_READWRITE)?;
                (p_target as Ptr).copy_from_nonoverlapping(gateway as _, stolen_size);
                virtual_free(gateway, 0, MEM_RELEASE).map_err(|_| SunshineError::FreeFailed)?;
            },
        }

        Ok(())
    }
}