bindgen vendor/basis_universal_transcoder_wrapper.cpp -o src/transcoding_bindings.rs \
  --whitelist-function basis_get_bytes_per_block_or_pixel \
  --whitelist-function basis_get_format_name \
  --whitelist-function basis_get_block_format_name \
  --whitelist-function basis_transcoder_format_has_alpha \
  --whitelist-function basis_get_basisu_texture_format \
  --whitelist-function basis_get_texture_type_name \
  \
  --whitelist-function transcoder_new \
  --whitelist-function transcoder_delete \
  --whitelist-function transcoder_get_total_images \
  --whitelist-function transcoder_get_tex_format \
  --whitelist-function transcoder_get_total_image_levels \
  \
  --whitelist-function transcoder_start_transcoding \
  --whitelist-function transcoder_stop_transcoding \
  --whitelist-function transcoder_get_ready_to_transcode \
  \
  --whitelist-function basisu_transcoder_init \
  \
  --opaque-type Transcoder \
  -- -x c++ -std=c++14

bindgen vendor/basis_universal_encoder_wrapper.cpp -o src/encoding_bindings.rs \
  --whitelist-function image_clear \
  --whitelist-function image_resize_with_pitch \
  --whitelist-function image_resize \
  --whitelist-function image_init \
  --whitelist-function image_get_pixel_at_checked \
  --whitelist-function image_get_pixel_at_unchecked \
  --whitelist-function image_get_width \
  --whitelist-function image_get_height \
  --whitelist-function image_get_pitch \
  --whitelist-function image_get_total_pixels \
  --whitelist-function image_get_block_width \
  --whitelist-function image_get_block_height \
  --whitelist-function image_get_total_blocks \
  --whitelist-function image_get_pixel_data \
  \
  --whitelist-function compressor_params_new \
  --whitelist-function compressor_params_delete \
  --whitelist-function compressor_params_clear \
  \
  --whitelist-function compressor_params_get_or_create_source_image \
  --whitelist-function compressor_params_resize_source_image_list \
  --whitelist-function compressor_params_clear_source_image_list \
  \
  --whitelist-function compressor_params_set_status_output \
  --whitelist-function compressor_params_set_quality_level \
  --whitelist-function compressor_params_set_global_sel_pal \
  --whitelist-function compressor_params_set_auto_global_sel_pal \
  --whitelist-function compressor_params_set_uastc \
  --whitelist-function compressor_params_set_generate_mipmaps \
  \
  --whitelist-function compressor_new \
  --whitelist-function compressor_delete \
  --whitelist-function compressor_init \
  --whitelist-function compressor_process \
  --whitelist-function compressor_get_output_basis_file \
  \
  --whitelist-function basisu_encoder_init \
  \
  --opaque-type CompressorParams \
  --opaque-type Compressor \
  --opaque-type basisu::image \
  \
  -- -x c++ -std=c++14