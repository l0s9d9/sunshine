#[derive(Debug)]
pub enum SunshineError {
    AllocationFailed,
    ProtectionFailed,
    QueryFailed,
    FreeFailed,
    TooFarAway,
    NotEnoughSpace,
    HookNotFound,
}

pub(crate) type Result<T> = std::result::Result<T, SunshineError>;