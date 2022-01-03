use radon::{types::{protection_flags::PAGE_EXECUTE_READWRITE, free_types::MEM_RELEASE}, internal::virtual_free};
use crate::{Ptr, ProtectionGuard, SunshineError, ABSOLUTE_JUMP_SIZE};

mod absolute;
pub(crate) use absolute::*;

mod compact;
pub use compact::*;

pub enum HookType {
    Absolute,
    Compact,
    // Midfunc
}

pub(crate) enum Hook {
    Absolute {
        gateway: usize,
        target: usize,
        stolen_size: usize
    },
    Compact {
        gateway: usize,
        tunnel: usize,
        stolen_size: usize,
        target: usize
    }
}

impl Hook {
    pub fn address(&self) -> usize {
        match self {
            Hook::Absolute { target, .. } => *target,
            Hook::Compact { target, .. } => *target,
        }
    }

    pub unsafe fn remove(self) -> crate::Result<()> {
        match self {
            Hook::Absolute { gateway, target: p_target, stolen_size } => {
                let _guard = ProtectionGuard::new(
                    p_target as _,
                    stolen_size,
                    PAGE_EXECUTE_READWRITE
                )?;

                (p_target as Ptr).copy_from_nonoverlapping(gateway as _, stolen_size);
                virtual_free(gateway, 0, MEM_RELEASE).map_err(|_| SunshineError::FreeFailed)?;
            },
            Hook::Compact { gateway, tunnel, stolen_size, target } => {
                let g1 = ProtectionGuard::new(
                    tunnel as _,
                    ABSOLUTE_JUMP_SIZE,
                    PAGE_EXECUTE_READWRITE
                )?;

                (tunnel as Ptr).write_bytes(0xCC, ABSOLUTE_JUMP_SIZE);
                drop(g1);

                let g2 = ProtectionGuard::new(
                    target as _,
                    stolen_size,
                    PAGE_EXECUTE_READWRITE
                )?;

                (target as Ptr).copy_from_nonoverlapping(gateway as _, stolen_size);
                drop(g2);

                virtual_free(gateway, 0, MEM_RELEASE).map_err(|_| SunshineError::FreeFailed)?;
            },
        }

        Ok(())
    }
}