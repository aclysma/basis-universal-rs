pub mod transcoding;
pub use transcoding::*;

pub mod encoding;
pub use encoding::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct UserData {
    pub userdata0: u32,
    pub userdata1: u32,
}
