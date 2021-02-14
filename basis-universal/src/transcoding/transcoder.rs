use super::*;
use basis_universal_sys as sys;

// // Returns true if the transcoder texture type is an uncompressed (raw pixel) format.
// bool basis_transcoder_format_is_uncompressed(transcoder_texture_format tex_type);
//
// // Returns the # of bytes per pixel for uncompressed formats, or 0 for block texture formats.
// uint32_t basis_get_uncompressed_bytes_per_pixel(transcoder_texture_format fmt);
//
// // Returns the block width for the specified texture format, which is currently either 4 or 8 for FXT1.
// uint32_t basis_get_block_width(transcoder_texture_format tex_type);
//
// // Returns the block height for the specified texture format, which is currently always 4.
// uint32_t basis_get_block_height(transcoder_texture_format tex_type);
//
// // Returns true if the specified format was enabled at compile time.
// bool basis_is_format_supported(transcoder_texture_format tex_type, basis_tex_format fmt = basis_tex_format::cETC1S);
//
// // Validates that the output buffer is large enough to hold the entire transcoded texture.
// // For uncompressed texture formats, most input parameters are in pixels, not blocks. Blocks are 4x4 pixels.
// bool basis_validate_output_buffer_size(transcoder_texture_format target_format,
//     uint32_t output_blocks_buf_size_in_blocks_or_pixels,
//     uint32_t orig_width, uint32_t orig_height,
//     uint32_t output_row_pitch_in_blocks_or_pixels,
//     uint32_t output_rows_in_pixels,
//     uint32_t total_slice_blocks);

pub struct Transcoder(*mut sys::Transcoder);

impl Transcoder {
    pub fn new() -> Transcoder {
        unsafe { Transcoder(sys::transcoder_new()) }
    }

    pub fn get_total_images(
        &self,
        data: &[u8],
    ) -> u32 {
        unsafe { sys::transcoder_get_total_images(self.0, data.as_ptr() as _, data.len() as u32) }
    }

    pub fn get_tex_format(
        &self,
        data: &[u8],
    ) -> TextureFormat {
        unsafe {
            sys::transcoder_get_tex_format(self.0, data.as_ptr() as _, data.len() as u32).into()
        }
    }

    pub fn get_total_image_levels(
        &self,
        data: &[u8],
        image_index: u32,
    ) -> u32 {
        unsafe {
            sys::transcoder_get_total_image_levels(
                self.0,
                data.as_ptr() as _,
                data.len() as u32,
                image_index,
            )
        }
    }

    //    // Returns basic information about an image. Note that orig_width/orig_height may not be a multiple of 4.
    //    bool get_image_level_desc(const void *pData, uint32_t data_size, uint32_t image_index, uint32_t level_index, uint32_t &orig_width, uint32_t &orig_height, uint32_t &total_blocks) const;
    //
    //    // Returns information about the specified image.
    //    bool get_image_info(const void *pData, uint32_t data_size, basisu_image_info &image_info, uint32_t image_index) const;
    //
    //    // Returns information about the specified image's mipmap level.
    //    bool get_image_level_info(const void *pData, uint32_t data_size, basisu_image_level_info &level_info, uint32_t image_index, uint32_t level_index) const;
    //
    //    // Get a description of the basis file and low-level information about each slice.
    //    bool get_file_info(const void *pData, uint32_t data_size, basisu_file_info &file_info) const;

    /// prepare_transcoding() must be called before calling transcode_slice() or transcode_image_level().
    /// This is `start_transcoding` in the original library
    /// For ETC1S files, this call decompresses the selector/endpoint codebooks, so ideally you would only call this once per .basis file (not each image/mipmap level).
    pub fn prepare_transcoding(
        &mut self,
        data: &[u8],
    ) -> Result<(), ()> {
        transcoder_init();
        unsafe {
            if sys::transcoder_start_transcoding(self.0, data.as_ptr() as _, data.len() as u32) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Parallel with `prepare_transcoding()`, named `stop_transcoding` in the original library
    pub fn end_transcoding(&mut self) {
        unsafe {
            let result = sys::transcoder_stop_transcoding(self.0);
            // I think this function is actually infallible, so don't return a result
            debug_assert!(result);
        }
    }

    /// Returns true if prepare_transcoding() has been called.
    pub fn is_prepared_to_transcode(&self) -> bool {
        unsafe { sys::transcoder_get_ready_to_transcode(self.0) }
    }

    //    // transcode_image_level() decodes a single mipmap level from the .basis file to any of the supported output texture formats.
    //    // It'll first find the slice(s) to transcode, then call transcode_slice() one or two times to decode both the color and alpha texture data (or RG texture data from two slices for BC5).
    //    // If the .basis file doesn't have alpha slices, the output alpha blocks will be set to fully opaque (all 255's).
    //    // Currently, to decode to PVRTC1 the basis texture's dimensions in pixels must be a power of 2, due to PVRTC1 format requirements.
    //    // output_blocks_buf_size_in_blocks_or_pixels should be at least the image level's total_blocks (num_blocks_x * num_blocks_y), or the total number of output pixels if fmt==cTFRGBA32.
    //    // output_row_pitch_in_blocks_or_pixels: Number of blocks or pixels per row. If 0, the transcoder uses the slice's num_blocks_x or orig_width (NOT num_blocks_x * 4). Ignored for PVRTC1 (due to texture swizzling).
    //    // output_rows_in_pixels: Ignored unless fmt is cRGBA32. The total number of output rows in the output buffer. If 0, the transcoder assumes the slice's orig_height (NOT num_blocks_y * 4).
    //    // Notes:
    //    // - basisu_transcoder_init() must have been called first to initialize the transcoder lookup tables before calling this function.
    //    // - This method assumes the output texture buffer is readable. In some cases to handle alpha, the transcoder will write temporary data to the output texture in
    //    // a first pass, which will be read in a second pass.
    //    bool transcode_image_level(
    //            const void *pData, uint32_t data_size,
    //            uint32_t image_index, uint32_t level_index,
    //            void *pOutput_blocks, uint32_t output_blocks_buf_size_in_blocks_or_pixels,
    //            transcoder_texture_format fmt,
    //            uint32_t decode_flags = 0, uint32_t output_row_pitch_in_blocks_or_pixels = 0, basisu_transcoder_state *pState = nullptr, uint32_t output_rows_in_pixels = 0) const;
    //
    //    // Finds the basis slice corresponding to the specified image/level/alpha params, or -1 if the slice can't be found.
    //    int find_slice(const void *pData, uint32_t data_size, uint32_t image_index, uint32_t level_index, bool alpha_data) const;
    //
    //    // transcode_slice() decodes a single slice from the .basis file. It's a low-level API - most likely you want to use transcode_image_level().
    //    // This is a low-level API, and will be needed to be called multiple times to decode some texture formats (like BC3, BC5, or ETC2).
    //    // output_blocks_buf_size_in_blocks_or_pixels is just used for verification to make sure the output buffer is large enough.
    //    // output_blocks_buf_size_in_blocks_or_pixels should be at least the image level's total_blocks (num_blocks_x * num_blocks_y), or the total number of output pixels if fmt==cTFRGBA32.
    //    // output_block_stride_in_bytes: Number of bytes between each output block.
    //    // output_row_pitch_in_blocks_or_pixels: Number of blocks or pixels per row. If 0, the transcoder uses the slice's num_blocks_x or orig_width (NOT num_blocks_x * 4). Ignored for PVRTC1 (due to texture swizzling).
    //    // output_rows_in_pixels: Ignored unless fmt is cRGBA32. The total number of output rows in the output buffer. If 0, the transcoder assumes the slice's orig_height (NOT num_blocks_y * 4).
    //    // Notes:
    //    // - basisu_transcoder_init() must have been called first to initialize the transcoder lookup tables before calling this function.
    //    bool transcode_slice(const void *pData, uint32_t data_size, uint32_t slice_index,
    //                         void *pOutput_blocks, uint32_t output_blocks_buf_size_in_blocks_or_pixels,
    //                         block_format fmt, uint32_t output_block_stride_in_bytes, uint32_t decode_flags = 0, uint32_t output_row_pitch_in_blocks_or_pixels = 0, basisu_transcoder_state * pState = nullptr, void* pAlpha_blocks = nullptr,
    //                         uint32_t output_rows_in_pixels = 0, int channel0 = -1, int channel1 = -1) const;
    //
    //    static void write_opaque_alpha_blocks(
    //            uint32_t num_blocks_x, uint32_t num_blocks_y,
    //            void* pOutput_blocks, block_format fmt,
    //            uint32_t block_stride_in_bytes, uint32_t output_row_pitch_in_blocks_or_pixels);
}

impl Drop for Transcoder {
    fn drop(&mut self) {
        unsafe {
            sys::transcoder_delete(self.0);
        }
    }
}
