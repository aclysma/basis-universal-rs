use super::*;
use basis_universal_sys as sys;
pub use basis_universal_sys::ColorU8;

/// Error codes that can be returned when encoding basis-universal data with a [Compressor]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum CompressorErrorCode {
    cECFailedReadingSourceImages =
        sys::basisu_basis_compressor_error_code_cECFailedReadingSourceImages,
    cECFailedValidating = sys::basisu_basis_compressor_error_code_cECFailedValidating,
    cECFailedEncodeUASTC = sys::basisu_basis_compressor_error_code_cECFailedEncodeUASTC,
    cECFailedFrontEnd = sys::basisu_basis_compressor_error_code_cECFailedFrontEnd,
    cECFailedFontendExtract = sys::basisu_basis_compressor_error_code_cECFailedFontendExtract,
    cECFailedBackend = sys::basisu_basis_compressor_error_code_cECFailedBackend,
    cECFailedCreateBasisFile = sys::basisu_basis_compressor_error_code_cECFailedCreateBasisFile,
    cECFailedWritingOutput = sys::basisu_basis_compressor_error_code_cECFailedWritingOutput,
    cECFailedUASTCRDOPostProcess =
        sys::basisu_basis_compressor_error_code_cECFailedUASTCRDOPostProcess,
}

impl Into<sys::basisu_basis_compressor_error_code> for CompressorErrorCode {
    fn into(self) -> sys::basisu_basis_compressor_error_code {
        self as sys::basisu_basis_compressor_error_code
    }
}

impl From<sys::basisu_basis_compressor_error_code> for CompressorErrorCode {
    fn from(value: sys::basisu_basis_compressor_error_code) -> Self {
        unsafe { std::mem::transmute(value as u32) }
    }
}

/// Used to encode raw image data to basis-universal form
pub struct Compressor(pub *mut sys::Compressor);

unsafe impl Send for Compressor {}

impl Compressor {
    /// total_thread_count is passed directly to basisu::job_pool
    /// total_thread_count is the TOTAL number of job pool threads, including the calling thread! So 2=1 new thread, 3=2 new threads, etc.
    ///
    /// Call `encoder_init`
    pub fn new(total_thread_count: u32) -> Self {
        encoder_init();
        unsafe {
            assert!(total_thread_count > 0);
            Compressor(sys::compressor_new(total_thread_count as _))
        }
    }

    /// Configure the compressor to compress images. `CompressorParams` includes both the image data
    /// and parameters that affect compression (such as quality or whether mipmaps should be
    /// generated)
    ///
    /// # Safety
    ///
    /// Passing invalid parameters may cause undefined behavior. (The underlying C++ library does
    /// not thoroughly validate parameters)
    pub unsafe fn init(
        &mut self,
        params: &CompressorParams,
    ) -> bool {
        sys::compressor_init(self.0, params.0)
    }

    /// Encodes the images as configured when calling `init()`
    ///
    /// # Safety
    ///
    /// Compressing with invalid parameters may cause undefined behavior. (The underlying C++
    /// library does not thoroughly validate parameters)
    pub unsafe fn process(&mut self) -> Result<(), CompressorErrorCode> {
        let result = sys::compressor_process(self.0);
        if result == sys::basisu_basis_compressor_error_code_cECSuccess {
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /// Access the compressed data. May be empty if `process()` was not yet called
    pub fn basis_file(&self) -> &[u8] {
        unsafe {
            let result = sys::compressor_get_output_basis_file(self.0);
            std::slice::from_raw_parts(result.pData, result.length as usize)
        }
    }

    /// Return the size of the encoded basis-universal data
    pub fn basis_file_size(&self) -> u32 {
        unsafe { sys::compressor_get_basis_file_size(self.0) }
    }

    /// Returns the number of bits required per texel
    pub fn bits_per_texel(&self) -> f64 {
        unsafe { sys::compressor_get_basis_bits_per_texel(self.0) }
    }

    /// Returns if any source image has alpha
    pub fn any_source_image_has_alpha(&self) -> bool {
        unsafe { sys::compressor_get_any_source_image_has_alpha(self.0) }
    }
}

impl Default for Compressor {
    fn default() -> Self {
        Compressor::new(1)
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe {
            sys::compressor_delete(self.0);
        }
    }
}
