#[derive(Debug)]
pub enum SunshineError {
    ProtectionError,
    AllocationFailed,
    HookNotFound,
    FreeFailed,
    TooFarAway,
}

pub(crate) type Result<T> = std::result::Result<T, SunshineError>;