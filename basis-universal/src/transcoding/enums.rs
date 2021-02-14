use basis_universal_sys as sys;
use std::ffi::CStr;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum BasisTextureType {
    /// An arbitrary array of 2D RGB or RGBA images with optional mipmaps, array size = # images, each image may have a different resolution and # of mipmap levels
    TextureType2D = sys::basist_basis_texture_type_cBASISTexType2D,
    /// An array of 2D RGB or RGBA images with optional mipmaps, array size = # images, each image has the same resolution and mipmap levels
    TextureType2DArray = sys::basist_basis_texture_type_cBASISTexType2DArray,
    /// an array of cubemap levels, total # of images must be divisable by 6, in X+, X-, Y+, Y-, Z+, Z- order, with optional mipmaps
    TextureTypeCubemapArray = sys::basist_basis_texture_type_cBASISTexTypeCubemapArray,
    /// An array of 2D video frames, with optional mipmaps, # frames = # images, each image has the same resolution and # of mipmap levels
    TextureTypeVideoFrames = sys::basist_basis_texture_type_cBASISTexTypeVideoFrames,
    /// A 3D texture with optional mipmaps, Z dimension = # images, each image has the same resolution and # of mipmap levels
    TextureTypeVolume = sys::basist_basis_texture_type_cBASISTexTypeVolume,
}

impl Into<sys::basist_basis_texture_type> for BasisTextureType {
    fn into(self) -> sys::basist_basis_texture_type {
        self as sys::basist_basis_texture_type
    }
}

impl From<sys::basist_basis_texture_type> for BasisTextureType {
    fn from(value: sys::basist_basis_texture_type) -> Self {
        unsafe { std::mem::transmute(value as u32) }
    }
}

impl BasisTextureType {
    /// Returns the texture type's name in ASCII.
    pub fn texture_type_name(self) -> &'static str {
        unsafe {
            let value = sys::basis_get_texture_type_name(self.into());
            CStr::from_ptr(value).to_str().unwrap()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum BasisTextureFormat {
    ETC1S = sys::basist_basis_tex_format_cETC1S,
    UASTC4x4 = sys::basist_basis_tex_format_cUASTC4x4,
}

impl Into<sys::basist_basis_tex_format> for BasisTextureFormat {
    fn into(self) -> sys::basist_basis_tex_format {
        self as sys::basist_basis_tex_format
    }
}

impl From<sys::basist_basis_tex_format> for BasisTextureFormat {
    fn from(value: sys::basist_basis_tex_format) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl BasisTextureFormat {
    /// Returns true if the specified format was enabled at compile time.
    pub fn can_transcode_to_format(
        self,
        transcoder_texture_format: TranscoderTextureFormat,
    ) -> bool {
        unsafe { sys::basis_is_format_supported(transcoder_texture_format.into(), self.into()) }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum TranscoderTextureFormat {
    /// Opaque only, returns RGB or alpha data if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    ETC1_RGB = sys::basist_transcoder_texture_format_cTFETC1_RGB,
    /// Opaque+alpha, ETC2_EAC_A8 block followed by a ETC1 block, alpha channel will be opaque for opaque .basis files
    ETC2_RBG = sys::basist_transcoder_texture_format_cTFETC2_RGBA,

    //
    // BC1-5, BC7 (desktop, some mobile devices)
    //
    /// Opaque only, no punchthrough alpha support yet, transcodes alpha slice if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    BC1_RGB = sys::basist_transcoder_texture_format_cTFBC1_RGB,
    /// Opaque+alpha, BC4 followed by a BC1 block, alpha channel will be opaque for opaque .basis files
    BC3_RGBA = sys::basist_transcoder_texture_format_cTFBC3_RGBA,
    /// Red only, alpha slice is transcoded to output if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    BC4_R = sys::basist_transcoder_texture_format_cTFBC4_R,
    /// XY: Two BC4 blocks, X=R and Y=Alpha, .basis file should have alpha data (if not Y will be all 255's)
    BC5_RG = sys::basist_transcoder_texture_format_cTFBC5_RG,
    /// RGB or RGBA, mode 5 for ETC1S, modes (1,2,3,5,6,7) for UASTC
    BC7_RGBA = sys::basist_transcoder_texture_format_cTFBC7_RGBA,

    //
    // PVRTC1 4bpp (mobile, PowerVR devices)
    //
    /// Opaque only, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified, nearly lowest quality of any texture format.
    PVRTC1_4_RGB = sys::basist_transcoder_texture_format_cTFPVRTC1_4_RGB,
    /// Opaque+alpha, most useful for simple opacity maps. If .basis file doesn't have alpha cTFPVRTC1_4_RGB will be used instead. Lowest quality of any supported texture format.
    PVRTC1_4_RGBA = sys::basist_transcoder_texture_format_cTFPVRTC1_4_RGBA,

    //
    // ASTC (mobile, Intel devices, hopefully all desktop GPU's one day)
    //
    /// Opaque+alpha, ASTC 4x4, alpha channel will be opaque for opaque .basis files. Transcoder uses RGB/RGBA/L/LA modes, void extent, and up to two ([0,47] and [0,255]) endpoint precisions.
    ASTC_4x4_RGBA = sys::basist_transcoder_texture_format_cTFASTC_4x4_RGBA,

    //
    // ATC (mobile, Adreno devices, this is a niche format)
    //
    /// Opaque, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified. ATI ATC (GL_ATC_RGB_AMD)
    ATC_RGB = sys::basist_transcoder_texture_format_cTFATC_RGB,
    /// Opaque+alpha, alpha channel will be opaque for opaque .basis files. ATI ATC (GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD)
    ATC_RGBA = sys::basist_transcoder_texture_format_cTFATC_RGBA,

    //
    // FXT1 (desktop, Intel devices, this is a super obscure format)
    //
    /// Opaque only, uses exclusively CC_MIXED blocks. Notable for having a 8x4 block size. GL_3DFX_texture_compression_FXT1 is supported on Intel integrated GPU's (such as HD 630).
    /// Punch-through alpha is relatively easy to support, but full alpha is harder. This format is only here for completeness so opaque-only is fine for now.
    /// See the BASISU_USE_ORIGINAL_3DFX_FXT1_ENCODING macro in basisu_transcoder_internal.h.
    FXT1_RGB = sys::basist_transcoder_texture_format_cTFFXT1_RGB,

    /// Opaque-only, almost BC1 quality, much faster to transcode and supports arbitrary texture dimensions (unlike PVRTC1 RGB).
    PVRTC2_4_RGB = sys::basist_transcoder_texture_format_cTFPVRTC2_4_RGB,
    /// Opaque+alpha, slower to encode than cTFPVRTC2_4_RGB. Premultiplied alpha is highly recommended, otherwise the color channel can leak into the alpha channel on transparent blocks.
    PVRTC2_4_RGBA = sys::basist_transcoder_texture_format_cTFPVRTC2_4_RGBA,

    /// R only (ETC2 EAC R11 unsigned)
    ETC2_EAC_R11 = sys::basist_transcoder_texture_format_cTFETC2_EAC_R11,
    /// RG only (ETC2 EAC RG11 unsigned), R=opaque.r, G=alpha - for tangent space normal maps
    ETC2_EAC_RG11 = sys::basist_transcoder_texture_format_cTFETC2_EAC_RG11,

    //
    // Uncompressed (raw pixel) formats
    //
    /// 32bpp RGBA image stored in raster (not block) order in memory, R is first byte, A is last byte.
    RGBA32 = sys::basist_transcoder_texture_format_cTFRGBA32,
    /// 16bpp RGB image stored in raster (not block) order in memory, R at bit position 11
    RGB565 = sys::basist_transcoder_texture_format_cTFRGB565,
    /// 16bpp RGB image stored in raster (not block) order in memory, R at bit position 0
    BGR565 = sys::basist_transcoder_texture_format_cTFBGR565,
    /// 16bpp RGBA image stored in raster (not block) order in memory, R at bit position 12, A at bit position 0
    RGBA4444 = sys::basist_transcoder_texture_format_cTFRGBA4444,
}

impl Into<sys::basist_transcoder_texture_format> for TranscoderTextureFormat {
    fn into(self) -> sys::basist_transcoder_texture_format {
        self as sys::basist_transcoder_texture_format
    }
}

impl From<sys::basist_transcoder_texture_format> for TranscoderTextureFormat {
    fn from(value: sys::basist_transcoder_texture_format) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl TranscoderTextureFormat {
    /// For compressed texture formats, this returns the # of bytes per block. For uncompressed, it returns the # of bytes per pixel.
    /// NOTE: Previously, this function was called basis_get_bytes_per_block(), and it always returned 16*bytes_per_pixel for uncompressed formats which was confusing.
    pub fn bytes_per_block_or_pixel(self) -> u32 {
        unsafe { sys::basis_get_bytes_per_block_or_pixel(self.into()) }
    }

    /// Returns format's name in ASCII
    pub fn format_name(self) -> &'static str {
        unsafe {
            let value = sys::basis_get_format_name(self.into());
            CStr::from_ptr(value).to_str().unwrap()
        }
    }

    /// Returns true if the format supports an alpha channel.
    pub fn has_alpha(self) -> bool {
        unsafe { sys::basis_transcoder_format_has_alpha(self.into()) }
    }

    /// Returns the basisu::texture_format corresponding to the specified transcoder_texture_format.
    pub fn texture_format(self) -> TextureFormat {
        unsafe { sys::basis_get_basisu_texture_format(self.into()).into() }
    }

    /// Returns true if the transcoder texture type is an uncompressed (raw pixel) format.
    pub fn is_compressed(self) -> bool {
        unsafe { !sys::basis_transcoder_format_is_uncompressed(self.into()) }
    }

    /// Returns the # of bytes per pixel for uncompressed formats, or 0 for block texture formats.
    pub fn uncompressed_bytes_per_pixel(self) -> u32 {
        unsafe { sys::basis_get_uncompressed_bytes_per_pixel(self.into()) }
    }

    /// Returns the block width for the specified texture format, which is currently either 4 or 8 for FXT1.
    pub fn block_width(self) -> u32 {
        unsafe { sys::basis_get_block_width(self.into()) }
    }

    /// Returns the block height for the specified texture format, which is currently always 4.
    pub fn block_height(self) -> u32 {
        unsafe { sys::basis_get_block_height(self.into()) }
    }

    /// Returns true if the specified format was enabled at compile time.
    pub fn can_transcode_from_format(
        self,
        basis_texture_format: BasisTextureFormat,
    ) -> bool {
        basis_texture_format.can_transcode_to_format(self)
    }

    pub fn calculate_minimum_output_buffer_blocks_or_pixels(
        self,
        original_width: u32,
        original_height: u32,
        total_slice_blocks: u32,
        output_row_pitch_in_blocks_or_pixels: Option<u32>,
        output_rows_in_pixels: Option<u32>,
    ) -> u32 {
        // Default of 0 is fine for these values
        let mut output_row_pitch_in_blocks_or_pixels =
            output_row_pitch_in_blocks_or_pixels.unwrap_or(0);
        let mut output_rows_in_pixels = output_rows_in_pixels.unwrap_or(0);

        // Derived from implementation of basis_validate_output_buffer_size
        let minimum_output_buffer_blocks_or_pixels = if !self.is_compressed() {
            // Assume the output buffer is orig_width by orig_height
            if output_row_pitch_in_blocks_or_pixels == 0 {
                output_row_pitch_in_blocks_or_pixels = original_width;
            }

            if output_rows_in_pixels == 0 {
                output_rows_in_pixels = original_height;
            }

            output_rows_in_pixels * output_row_pitch_in_blocks_or_pixels
        } else if self == TranscoderTextureFormat::FXT1_RGB {
            let num_blocks_fxt1_x = (original_width + 7) / 8;
            let num_blocks_fxt1_y = (original_height + 3) / 4;
            let total_blocks_fxt1 = num_blocks_fxt1_x * num_blocks_fxt1_y;
            total_blocks_fxt1
        } else {
            total_slice_blocks
        };

        debug_assert!(self.validate_output_buffer_size(
            minimum_output_buffer_blocks_or_pixels,
            original_width,
            original_height,
            total_slice_blocks,
            Some(output_row_pitch_in_blocks_or_pixels),
            Some(output_rows_in_pixels),
        ));

        minimum_output_buffer_blocks_or_pixels
    }

    pub fn calculate_minimum_output_buffer_bytes(
        self,
        original_width: u32,
        original_height: u32,
        total_slice_blocks: u32,
        output_row_pitch_in_blocks_or_pixels: Option<u32>,
        output_rows_in_pixels: Option<u32>,
    ) -> u32 {
        self.calculate_minimum_output_buffer_blocks_or_pixels(
            original_width,
            original_height,
            total_slice_blocks,
            output_row_pitch_in_blocks_or_pixels,
            output_rows_in_pixels,
        ) * self.bytes_per_block_or_pixel()
    }

    pub fn validate_output_buffer_size(
        self,
        output_blocks_buf_size_in_blocks_or_pixels: u32,
        original_width: u32,
        original_height: u32,
        total_slice_blocks: u32,
        output_row_pitch_in_blocks_or_pixels: Option<u32>,
        output_rows_in_pixels: Option<u32>,
    ) -> bool {
        unsafe {
            sys::basis_validate_output_buffer_size(
                self.into(),
                output_blocks_buf_size_in_blocks_or_pixels,
                original_width,
                original_height,
                output_row_pitch_in_blocks_or_pixels.unwrap_or(0),
                output_rows_in_pixels.unwrap_or(0),
                total_slice_blocks,
            )
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum TranscoderBlockFormat {
    ETC1 = sys::basist_block_format_cETC1,           // ETC1S RGB
    ETC2_RGBA = sys::basist_block_format_cETC2_RGBA, // full ETC2 EAC RGBA8 block
    BC1 = sys::basist_block_format_cBC1,             // DXT1 RGB
    BC3 = sys::basist_block_format_cBC3,             // BC4 block followed by a four color BC1 block
    BC4 = sys::basist_block_format_cBC4,             // DXT5A (alpha block only)
    BC5 = sys::basist_block_format_cBC5,             // two BC4 blocks
    PVRTC1_4_RGB = sys::basist_block_format_cPVRTC1_4_RGB, // opaque-only PVRTC1 4bpp
    PVRTC1_4_RGBA = sys::basist_block_format_cPVRTC1_4_RGBA, // PVRTC1 4bpp RGBA
    BC7 = sys::basist_block_format_cBC7,             // Full BC7 block, any mode
    BC7_M5_COLOR = sys::basist_block_format_cBC7_M5_COLOR, // RGB BC7 mode 5 color (writes an opaque mode 5 block)
    BC7_M5_ALPHA = sys::basist_block_format_cBC7_M5_ALPHA, // alpha portion of BC7 mode 5 (cBC7_M5_COLOR output data must have been written to the output buffer first to set the mode/rot fields etc.)
    ETC2_EAC_A8 = sys::basist_block_format_cETC2_EAC_A8, // alpha block of ETC2 EAC (first 8 bytes of the 16-bit ETC2 EAC RGBA format)
    ASTC_4x4 = sys::basist_block_format_cASTC_4x4, // ASTC 4x4 (either color-only or color+alpha). Note that the transcoder always currently assumes sRGB is not enabled when outputting ASTC
    // data. If you use a sRGB ASTC format you'll get ~1 LSB of additional error, because of the different way ASTC decoders scale 8-bit endpoints to 16-bits during unpacking.
    ATC_RGB = sys::basist_block_format_cATC_RGB,
    ATC_RGBA_INTERPOLATED_ALPHA = sys::basist_block_format_cATC_RGBA_INTERPOLATED_ALPHA,
    FXT1_RGB = sys::basist_block_format_cFXT1_RGB, // Opaque-only, has oddball 8x4 pixel block size
    PVRTC2_4_RGB = sys::basist_block_format_cPVRTC2_4_RGB,
    PVRTC2_4_RGBA = sys::basist_block_format_cPVRTC2_4_RGBA,
    ETC2_EAC_R11 = sys::basist_block_format_cETC2_EAC_R11,
    ETC2_EAC_RG11 = sys::basist_block_format_cETC2_EAC_RG11,
    RGB32 = sys::basist_block_format_cRGB32, // Writes RGB components to 32bpp output pixels
    RGBA32 = sys::basist_block_format_cRGBA32, // Writes RGB255 components to 32bpp output pixels
    A32 = sys::basist_block_format_cA32,     // Writes alpha component to 32bpp output pixels
    RGB565 = sys::basist_block_format_cRGB565,
    BGR565 = sys::basist_block_format_cBGR565,
    RGBA4444_COLOR = sys::basist_block_format_cRGBA4444_COLOR,
    RGBA4444_ALPHA = sys::basist_block_format_cRGBA4444_ALPHA,
    RGBA4444_COLOR_OPAQUE = sys::basist_block_format_cRGBA4444_COLOR_OPAQUE,
    RGBA4444 = sys::basist_block_format_cRGBA4444,
}

impl Into<sys::basist_block_format> for TranscoderBlockFormat {
    fn into(self) -> sys::basist_block_format {
        self as sys::basist_block_format
    }
}

impl From<sys::basist_block_format> for TranscoderBlockFormat {
    fn from(value: sys::basist_block_format) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl TranscoderBlockFormat {
    /// Returns block format name in ASCII
    pub fn block_format_name(self) -> &'static str {
        unsafe {
            let value = sys::basis_get_block_format_name(self.into());
            CStr::from_ptr(value).to_str().unwrap()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum TextureFormat {
    Invalid = sys::basisu_texture_format_cInvalidTextureFormat,

    //
    // Block-based formats
    //
    ETC1 = sys::basisu_texture_format_cETC1,           // ETC1
    ETC1S = sys::basisu_texture_format_cETC1S, // ETC1 (subset: diff colors only, no subblocks)
    ETC2 = sys::basisu_texture_format_cETC2_RGB, // ETC2 color block (basisu doesn't support ETC2 planar/T/H modes - just basic ETC1)
    ETC2_RGBA = sys::basisu_texture_format_cETC2_RGBA, // ETC2 EAC alpha block followed by ETC2 color block
    ETC2_ALPHA = sys::basisu_texture_format_cETC2_ALPHA, // ETC2 EAC alpha block
    BC1 = sys::basisu_texture_format_cBC1,             // DXT1
    BC3 = sys::basisu_texture_format_cBC3, // DXT5 (BC4/DXT5A block followed by a BC1/DXT1 block)
    BC4 = sys::basisu_texture_format_cBC4, // DXT5A
    BC5 = sys::basisu_texture_format_cBC5, // 3DC/DXN (two BC4/DXT5A blocks)
    BC7 = sys::basisu_texture_format_cBC7,
    ASTC4x4 = sys::basisu_texture_format_cASTC4x4, // LDR only
    PVRTC1_4_RGB = sys::basisu_texture_format_cPVRTC1_4_RGB,
    PVRTC1_4_RGBA = sys::basisu_texture_format_cPVRTC1_4_RGBA,
    ATC_RGB = sys::basisu_texture_format_cATC_RGB,
    ATC_RGBA_INTERPOLATED_ALPHA = sys::basisu_texture_format_cATC_RGBA_INTERPOLATED_ALPHA,
    cFXT1_RGB = sys::basisu_texture_format_cFXT1_RGB,
    cPVRTC2_4_RGBA = sys::basisu_texture_format_cPVRTC2_4_RGBA,
    cETC2_R11_EAC = sys::basisu_texture_format_cETC2_R11_EAC,
    cETC2_RG11_EAC = sys::basisu_texture_format_cETC2_RG11_EAC,
    cUASTC4x4 = sys::basisu_texture_format_cUASTC4x4,
    cBC1_NV = sys::basisu_texture_format_cBC1_NV,
    cBC1_AMD = sys::basisu_texture_format_cBC1_AMD,

    //
    // Uncompressed/raw pixels
    //
    cRGBA32 = sys::basisu_texture_format_cRGBA32,
    cRGB565 = sys::basisu_texture_format_cRGB565,
    cBGR565 = sys::basisu_texture_format_cBGR565,
    cRGBA4444 = sys::basisu_texture_format_cRGBA4444,
    cABGR4444 = sys::basisu_texture_format_cABGR4444,
}

impl Into<sys::basisu_texture_format> for TextureFormat {
    fn into(self) -> sys::basisu_texture_format {
        self as sys::basisu_texture_format
    }
}

impl From<sys::basisu_texture_format> for TextureFormat {
    fn from(value: sys::basisu_texture_format) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

bitflags::bitflags! {
    pub struct DecodeFlags: u32 {
        /// PVRTC1: decode non-pow2 ETC1S texture level to the next larger power of 2 (not implemented yet, but we're going to support it). Ignored if the slice's dimensions are already a power of 2.
        const PVRTC_DECODE_TO_NEXT_POW_2 = sys::basist_basisu_decode_flags_cDecodeFlagsPVRTCDecodeToNextPow2;

        /// When decoding to an opaque texture format, if the basis file has alpha, decode the alpha slice instead of the color slice to the output texture format.
        /// This is primarily to allow decoding of textures with alpha to multiple ETC1 textures (one for color, another for alpha).
        const TRANSCODE_ALPHA_DATA_TO_OPAQUE_FORMATS = sys::basist_basisu_decode_flags_cDecodeFlagsTranscodeAlphaDataToOpaqueFormats;

        /// Forbid usage of BC1 3 color blocks (we don't support BC1 punchthrough alpha yet).
        /// This flag is used internally when decoding to BC3.
        const BC1_FORBID_THREE_COLOR_BLOCKS = sys::basist_basisu_decode_flags_cDecodeFlagsBC1ForbidThreeColorBlocks;

        /// The output buffer contains alpha endpoint/selector indices.
        /// Used internally when decoding formats like ASTC that require both color and alpha data to be available when transcoding to the output format.
        const OUTPUT_HAS_ALPHA_INDICES = sys::basist_basisu_decode_flags_cDecodeFlagsOutputHasAlphaIndices;

        const HIGH_QULITY = sys::basist_basisu_decode_flags_cDecodeFlagsHighQuality;
    }
}
