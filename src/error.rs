#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    AreaIntConversionError(#[from] std::num::TryFromIntError),
    #[error("Window capture failed.")]
    WindowCaptureFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
