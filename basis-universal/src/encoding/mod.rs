use basis_universal_sys as sys;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

mod compressor_image;
pub use compressor_image::*;

mod compressor_params;
pub use compressor_params::*;

mod compressor;
pub use compressor::*;

/// A single uncompressed pixel value
pub use basis_universal_sys::ColorU8;

#[cfg(test)]
mod encoding_tests;

static ENCODER_INIT_CALLED: AtomicBool = AtomicBool::new(false);
lazy_static::lazy_static! {
    static ref ENCODER_INIT_LOCK: Mutex<()> = Mutex::default();
}

/// The underlying C++ library requires that encoder_init() has been called before a .basis file can
/// be encoded. This function allows a user to do this early in the application explicitly. It is
/// protected by a lock and AtomicBool flag so it is safe and cheap to call multiple times, and
/// correctly handles multiple threads trying to initialize at the same time.
pub fn encoder_init() {
    unsafe {
        // Early out if it has been initialized
        if !ENCODER_INIT_CALLED.load(Ordering::Acquire) {
            // Lock and check again to ensure that exactly one thread runs the init code and that
            // all other threads wait for it to complete and don't re-run it.
            let lock = ENCODER_INIT_LOCK.lock().unwrap();
            if !ENCODER_INIT_CALLED.load(Ordering::Acquire) {
                // Run the init code
                sys::basisu_encoder_init();
                ENCODER_INIT_CALLED.store(true, Ordering::Release);
            }
            std::mem::drop(lock);
        }
    }
}
