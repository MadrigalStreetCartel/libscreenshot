mod area_capture_provider;
mod full_capture_provider;
mod screen_capture_provider;
mod window_capture_provider;

pub use self::area_capture_provider::*;
pub use self::full_capture_provider::*;
pub use self::screen_capture_provider::*;
pub use self::window_capture_provider::*;

pub trait Provider
where
    Self: Sized,
{
    fn new() -> Self;
}
