#[derive(Debug)]
pub enum SunshineError {
    ProtectionError,
    AllocationFailed,
    TooFarAway,
}

pub(crate) type Result<T> = std::result::Result<T, SunshineError>;