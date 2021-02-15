use basis_universal_sys as sys;
pub use basis_universal_sys::ColorU8;

// foreign_types::foreign_type! {
//     /// A Foo.
//     pub unsafe type Foo
//         : Sync + Send // optional
//     {
//         type CType = sys::basisu_image;
//         fn drop = unimplemented!();
//     }
// }

/// A reference to an image being stored by [CompressorParams](super::CompressorParams). Generally
/// used to insert the source data that is to be encoded by a [Compressor](super::Compressor).
pub struct CompressorImageRef(pub *mut sys::basisu_image);

impl CompressorImageRef {
    /// Sets the image to be completely empty (i.e. 0 width, 0 height). (This was called `clear` in
    /// the upstream API.)
    pub fn invalidate(&mut self) {
        unsafe {
            sys::image_clear(self.0);
        }
    }

    /// Resizes the image to the given width/height
    ///
    /// By default the pitch will be equal to the width. To customize this, use `resize_with_pitch`
    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        unsafe {
            sys::image_resize(self.0, width, height);
        }
    }

    /// Resize the image to the given width/height with a custom "pitch". The pitch is the
    /// offset between rows and is not needed for all formats. By default, the pitch will be equal
    /// to the width
    pub fn resize_with_pitch(
        &mut self,
        width: u32,
        height: u32,
        pitch: u32,
    ) {
        unsafe {
            sys::image_resize_with_pitch(self.0, width, height, pitch);
        }
    }

    /// Resize the image and populate it with the given data.
    ///
    /// channel_count should be the number of channels in the image (so >=1 and <= 4)
    pub fn init(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        channel_count: u8,
    ) {
        unsafe {
            sys::image_init(self.0, data.as_ptr(), width, height, channel_count as _);
        }
    }

    /// Returns the pixel value at a given x,y
    pub fn pixel_at(
        &self,
        width: u32,
        height: u32,
    ) -> Option<ColorU8> {
        unsafe {
            let mut color = ColorU8 { combined: 0 };

            if sys::image_get_pixel_at_checked(self.0, width, height, &mut color as *mut _) {
                Some(color)
            } else {
                None
            }
        }
    }

    /// Returns teh pixel value at a given x,y without doing bounds checking
    pub unsafe fn pixel_at_unchecked(
        &self,
        width: u32,
        height: u32,
    ) -> ColorU8 {
        sys::image_get_pixel_at_unchecked(self.0, width, height)
    }

    /// Returns the width of the image in pixels
    pub fn width(&self) -> u32 {
        unsafe { sys::image_get_width(self.0) }
    }

    /// Returns the height of the image in pixels
    pub fn height(&self) -> u32 {
        unsafe { sys::image_get_height(self.0) }
    }

    /// Returns the pitch of the image in pixels, which represents the offset between rows
    pub fn pitch(&self) -> u32 {
        unsafe { sys::image_get_pitch(self.0) }
    }

    /// Returns the total number of pixels in the image
    pub fn total_pixels(&self) -> u32 {
        unsafe { sys::image_get_total_pixels(self.0) }
    }

    /// Returns how many blocks wide the image is, given `w`, the width of a block in pixels
    pub fn block_width(
        &self,
        w: u32,
    ) -> u32 {
        unsafe { sys::image_get_block_width(self.0, w) }
    }

    /// Returns how many blocks high the image is, given `h`, the height of a block in pixels
    pub fn block_height(
        &self,
        h: u32,
    ) -> u32 {
        unsafe { sys::image_get_block_height(self.0, h) }
    }

    /// Returns the number of blocks required to store the image, given `w` and `h`, the width and
    /// height of a block in pixels
    pub fn total_blocks(
        &self,
        w: u32,
        h: u32,
    ) -> u32 {
        unsafe { sys::image_get_total_blocks(self.0, w, h) }
    }

    /// Returns a mutable reference to the pixel data as a slice of [ColorU8]
    pub fn pixel_data_mut(&mut self) -> &mut [ColorU8] {
        unsafe {
            let data = sys::image_get_pixel_data(self.0);
            std::slice::from_raw_parts_mut(data.pData, data.length as usize)
        }
    }

    /// Returns a mutable reference to the pixel data as a slice of u8
    pub fn pixel_data_u8_mut(&mut self) -> &mut [u8] {
        unsafe {
            let data = sys::image_get_pixel_data(self.0);
            std::slice::from_raw_parts_mut(
                data.pData as *mut u8,
                data.length as usize * std::mem::size_of::<ColorU8>(),
            )
        }
    }

    /// Returns a mutable reference to the pixel data as a slice of u32
    pub fn pixel_data_u32_mut(&mut self) -> &mut [u32] {
        debug_assert_eq!(std::mem::size_of::<u32>(), std::mem::size_of::<ColorU8>());
        unsafe {
            let data = sys::image_get_pixel_data(self.0);
            std::slice::from_raw_parts_mut(data.pData as *mut u32, data.length as usize)
        }
    }
}
