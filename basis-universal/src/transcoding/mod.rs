use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

mod enums;
pub use enums::*;

mod transcoder;
pub use transcoder::*;

#[cfg(test)]
mod transcoding_tests;

static TRANSCODER_INIT_CALLED: AtomicBool = AtomicBool::new(false);
lazy_static::lazy_static! {
    static ref TRANSCODER_INIT_LOCK: Mutex<()> = Mutex::default();
}

/// The underlying C++ library requires that transcoder_init() has been called before a .basis file
/// can be encoded. This function allows a user to do this early in the application explicitly. It
/// is protected by a lock and AtomicBool flag so it is safe and cheap to call multiple times, and
/// correctly handles multiple threads trying to initialize at the same time.
pub fn transcoder_init() {
    // Early out if it has been initialized
    if !TRANSCODER_INIT_CALLED.load(Ordering::Acquire) {
        // Lock and check again to ensure that exactly one thread runs the init code and that
        // all other threads wait for it to complete and don't re-run it.
        let lock = TRANSCODER_INIT_LOCK.lock().unwrap();
        if !TRANSCODER_INIT_CALLED.load(Ordering::Acquire) {
            // Run the init code
            #[cfg(not(target_arch = "wasm32"))]
            unsafe {
                sys::basisu_encoder_init();
            }
            #[cfg(target_arch = "wasm32")]
            basis_universal_wasm::initialize_basis();
            TRANSCODER_INIT_CALLED.store(true, Ordering::Release);
        }
        std::mem::drop(lock);
    }
}
