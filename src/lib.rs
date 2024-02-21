#![feature(c_variadic)]

mod buttons;
pub use buttons::{Buttons, InputPort};
mod emulator;
pub use emulator::Emulator;
mod error;
pub use error::*;
pub mod pixels;
pub use libretro_sys;

#[cfg(feature = "use_image")]
mod fb_to_image;
#[cfg(feature = "use_image")]
pub use fb_to_image::*;
