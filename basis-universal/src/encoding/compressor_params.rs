use super::*;
use crate::{BasisTextureFormat, UserData};
use basis_universal_sys as sys;
pub use basis_universal_sys::ColorU8;

pub struct CompressorParams(pub *mut sys::CompressorParams);

impl CompressorParams {
    pub fn new() -> Self {
        unsafe {
            let mut params = CompressorParams(sys::compressor_params_new());
            params.set_default_options();
            params
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            sys::compressor_params_clear(self.0);
            self.set_default_options();
        }
    }

    fn set_default_options(&mut self) {
        // Set a default quality level. Leaving this unset results in undefined behavior
        self.set_quality_level(Some(128));

        // The library by default prints to stdout, but since this is a lib we should disable that
        // by default
        self.set_print_status_to_stdout(false);
    }

    //
    // These function are used to load image data into the compressor
    //
    pub fn source_image_mut(
        &mut self,
        image_index: u32,
    ) -> CompressorImageRef {
        unsafe {
            CompressorImageRef(sys::compressor_params_get_or_create_source_image(
                self.0,
                image_index,
            ))
        }
    }

    pub fn resize_source_image_list(
        &mut self,
        size: u32,
    ) {
        unsafe {
            sys::compressor_params_resize_source_image_list(self.0, size as _);
        }
    }

    pub fn clear_source_image_list(&mut self) {
        unsafe {
            sys::compressor_params_clear_source_image_list(self.0);
        }
    }

    //
    // These set parameters for compression
    //
    pub fn set_print_status_to_stdout(
        &mut self,
        print_status_to_stdout: bool,
    ) {
        unsafe { sys::compressor_params_set_status_output(self.0, print_status_to_stdout) }
    }

    pub fn set_quality_level(
        &mut self,
        quality_level: Option<u32>,
    ) {
        unsafe {
            sys::compressor_params_set_quality_level(
                self.0,
                quality_level.map(|x| x as i32).unwrap_or(-1),
            );
        }
    }

    pub fn set_use_global_codebook(
        &mut self,
        use_global_codebook: bool,
    ) {
        unsafe {
            sys::compressor_params_set_global_sel_pal(self.0, use_global_codebook);
        }
    }

    pub fn set_auto_use_global_codebook(
        &mut self,
        auto_use_global_codebook: bool,
    ) {
        unsafe {
            sys::compressor_params_set_auto_global_sel_pal(self.0, auto_use_global_codebook);
        }
    }

    /// Set the basis format we will compress to. See basis documentation for details. This
    /// corresponds to the -uastc flag in the basisu command line tool and the m_uastc boolean param
    /// on `basis_compressor_params` in the original library
    pub fn set_basis_format(
        &mut self,
        basis_format: BasisTextureFormat,
    ) {
        let is_uastc = match basis_format {
            BasisTextureFormat::ETC1S => false,
            BasisTextureFormat::UASTC4x4 => true,
        };

        unsafe {
            sys::compressor_params_set_uastc(self.0, is_uastc);
        }
    }

    pub fn set_generate_mipmaps(
        &mut self,
        generate_mipmaps: bool,
    ) {
        unsafe {
            sys::compressor_params_set_generate_mipmaps(self.0, generate_mipmaps);
        }
    }

    pub fn set_userdata(
        &mut self,
        userdata: UserData,
    ) {
        unsafe {
            sys::compressor_params_set_userdata(self.0, userdata.userdata0, userdata.userdata1);
        }
    }

    // set_multithreaded not implemented here as this is controlled by thread count passed to
    // `Compressor::new()`
}

impl Drop for CompressorParams {
    fn drop(&mut self) {
        unsafe {
            sys::compressor_params_delete(self.0);
        }
    }
}
