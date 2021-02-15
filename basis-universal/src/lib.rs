//! Bindings for Binomial LLC's basis-universal Supercompressed GPU Texture Codec
//!
//! basis-universal functionality can be grouped into two categories:
//!
//! * Encoding: Compresses and encode textures (optionally combining multiple images and mipmap
//!   layers in a single file/binary blob)
//! * Transcoding: Unpacks the texture into GPU-friendly compression formats. The final format can
//!   be chosen based on what the available GPU hardware can support.
//!
//! Encoding can be done ahead of time using a command line tool in the upstream repository.
//!
//! The encoded data can either be stored as a file or a binary blob. This data can include multiple
//! images, and each image can store multiple levels. This is commonly used for storing cube
//! textures and textures with precomputed mipmaps. This library also supports generating mipmaps
//! for you.
//!
//! Please refer to https://github.com/BinomialLLC/basis_universal for more details.

/// Support for transcoding basis-universal form to GPU-friendly formats.
pub mod transcoding;
pub use transcoding::*;

/// Support for compressing raw image data to basis-universal form
pub mod encoding;
pub use encoding::*;

pub use basis_universal_sys as sys;

/// Arbitrary data that can be attached to a basis-universal file/binary blob
#[derive(Default, Debug, Copy, Clone)]
pub struct UserData {
    pub userdata0: u32,
    pub userdata1: u32,
}

/// The default quality level used if [CompressorParams::set_etc1s_quality_level] is not called
pub const ETC1S_QUALITY_DEFAULT: u32 = sys::basisu_BASISU_DEFAULT_QUALITY as u32;
/// The minimum quality level that can be provided to [CompressorParams::set_etc1s_quality_level]
pub const ETC1S_QUALITY_MIN: u32 = sys::basisu_BASISU_QUALITY_MIN as u32;
/// The maximum quality level that can be provided to [CompressorParams::set_etc1s_quality_level]
pub const ETC1S_QUALITY_MAX: u32 = sys::basisu_BASISU_QUALITY_MAX as u32;

/// The default quality level used if [CompressorParams::set_uastc_quality_level] is not called
pub const UASTC_QUALITY_DEFAULT: u32 = sys::UastcPackFlags_PackUASTCLevelDefault as u32;
/// The minimum quality level that can be provided to [CompressorParams::set_uastc_quality_level]
pub const UASTC_QUALITY_MIN: u32 = sys::UastcPackFlags_PackUASTCLevelFastest as u32;
/// The maximum quality level that can be provided to [CompressorParams::set_uastc_quality_level]
pub const UASTC_QUALITY_MAX: u32 = sys::UastcPackFlags_PackUASTCLevelVerySlow as u32;

/// Maximum supported texture dimension
pub const TEXTURE_DIMENSION_MAX: u32 = sys::basisu_BASISU_MAX_SUPPORTED_TEXTURE_DIMENSION as u32;
/// Maximum supported image dimension
pub const IMAGE_DIMENSION_MAX: u32 = sys::basisu_BASISU_MAX_IMAGE_DIMENSION as u32;
