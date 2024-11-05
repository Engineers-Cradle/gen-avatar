extern crate failure;

extern crate image;
extern crate rand;
extern crate rusttype;

pub mod avatar;
pub mod color;
pub mod error;

pub use avatar::AvatarBuilder;
pub use avatar::AvatarResult;
pub use error::Error;