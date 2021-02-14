pub mod transcoding;
pub use transcoding::*;

pub mod encoding;
pub use encoding::*;

pub use basis_universal_sys as sys;

#[derive(Default, Debug, Copy, Clone)]
pub struct UserData {
    pub userdata0: u32,
    pub userdata1: u32,
}

const QUALITY_DEFAULT : u32 = sys::basisu_BASISU_DEFAULT_QUALITY as u32;
const QUALITY_MIN : u32 = sys::basisu_BASISU_QUALITY_MIN as u32;
const QUALITY_MAX : u32 = sys::basisu_BASISU_QUALITY_MAX as u32;
const TEXTURE_DIMENSION_MAX : u32 = sys::basisu_BASISU_MAX_SUPPORTED_TEXTURE_DIMENSION as u32;
const IMAGE_DIMENSION_MAX : u32 = sys::basisu_BASISU_MAX_IMAGE_DIMENSION as u32;
