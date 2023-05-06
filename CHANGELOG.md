# Changelog

## 0.3.0
 * Updated to most recent up-stream (1.16.x)
 * `compressor_params_set_global_sel_pal`, `compressor_params_set_auto_global_sel_pal` and
   `etc1_global_selector_codebook` were removed, as they no longer exist upstream
 * Some of the bindings have been changed to i32 to match upstream C++ behavior

## 0.2.0
 * Add basis_block_format_is_uncompressed()
 * Add LowLevelUastcTranscoder, TranscoderBlockFormat
 * Fix typo HIGH_QULITY -> HIGH_QUALITY
 * Add files to be excluded when publishing

## 0.1.1
 * Add set_rdo_uastc()
 * Bump upstream code to more recent revision

## 0.1.0
 * Initial release
