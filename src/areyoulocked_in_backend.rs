pub(crate) type IoResult<T> = std::io::Result<T>;
pub(crate) type BackendError = Box<dyn std::error::Error>;
pub(crate) type BackendResult<T> = Result<T, BackendError>;
