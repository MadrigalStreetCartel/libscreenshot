#[cfg(feature = "macos")]
mod cg_provider;

#[cfg(feature = "macos")]
pub use self::cg_provider::CGProvider;
