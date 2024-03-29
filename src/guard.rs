use faithe::types::protection_flags::PAGE_PROTECTION_FLAGS;
use crate::{Ptr, SunshineError};

pub(crate) struct ProtectionGuard(Ptr, usize, PAGE_PROTECTION_FLAGS);

impl ProtectionGuard {
    pub fn new(address: Ptr, size: usize, protection: PAGE_PROTECTION_FLAGS) -> crate::Result<Self> {
        let old = faithe::internal::virtual_protect(address as _, size, protection)
            .map_err(|_| SunshineError::ProtectionFailed)?;

        Ok(Self(address, size, old))
    }
}

impl Drop for ProtectionGuard {
    fn drop(&mut self) {
        faithe::internal::virtual_protect(self.0 as _, self.1, self.2).unwrap();
    }
}