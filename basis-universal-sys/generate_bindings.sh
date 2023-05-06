bindgen vendor/transcoding_wrapper.cpp -o src/transcoding_bindings.rs \
  --allowlist-function basis_get_bytes_per_block_or_pixel \
  --allowlist-function basis_get_format_name \
  --allowlist-function basis_get_block_format_name \
  --allowlist-function basis_transcoder_format_has_alpha \
  --allowlist-function basis_get_basisu_texture_format \
  --allowlist-function basis_get_texture_type_name \
  \
  --allowlist-function basis_transcoder_format_is_uncompressed \
  --allowlist-function basis_block_format_is_uncompressed \
  --allowlist-function basis_get_uncompressed_bytes_per_pixel \
  --allowlist-function basis_get_block_width \
  --allowlist-function basis_get_block_height \
  --allowlist-function basis_is_format_supported \
  --allowlist-function basis_validate_output_buffer_size \
  \
  --allowlist-function low_level_uastc_transcoder_new \
  --allowlist-function low_level_uastc_transcoder_delete \
  \
  --allowlist-function low_level_uastc_transcoder_transcode_slice \
  \
  --allowlist-function transcoder_new \
  --allowlist-function transcoder_delete \
  \
  --allowlist-function transcoder_validate_file_checksums \
  --allowlist-function transcoder_validate_header \
  --allowlist-function transcoder_get_texture_type \
  --allowlist-function transcoder_get_userdata \
  \
  --allowlist-function transcoder_get_total_images \
  --allowlist-function transcoder_get_tex_format \
  --allowlist-function transcoder_get_total_image_levels \
  \
  --allowlist-function transcoder_get_image_level_desc \
  --allowlist-function transcoder_get_image_info \
  --allowlist-function transcoder_get_image_level_info \
  --allowlist-function transcoder_get_file_info \
  \
  --allowlist-function transcoder_start_transcoding \
  --allowlist-function transcoder_stop_transcoding \
  --allowlist-function transcoder_get_ready_to_transcode \
  --allowlist-function transcoder_transcode_image_level \
  \
  --allowlist-function basisu_transcoder_init \
  \
  --opaque-type LowLevelUastcTranscoder \
  --opaque-type basist::block_format \
  \
  --opaque-type Transcoder \
  --opaque-type basist::basisu_transcoder_state \
  \
  -- -x c++ -std=c++14

bindgen vendor/encoding_wrapper.cpp -o src/encoding_bindings.rs \
  --allowlist-function image_clear \
  --allowlist-function image_resize_with_pitch \
  --allowlist-function image_resize \
  --allowlist-function image_init \
  --allowlist-function image_get_pixel_at_checked \
  --allowlist-function image_get_pixel_at_unchecked \
  --allowlist-function image_get_width \
  --allowlist-function image_get_height \
  --allowlist-function image_get_pitch \
  --allowlist-function image_get_total_pixels \
  --allowlist-function image_get_block_width \
  --allowlist-function image_get_block_height \
  --allowlist-function image_get_total_blocks \
  --allowlist-function image_get_pixel_data \
  \
  --allowlist-function compressor_params_new \
  --allowlist-function compressor_params_delete \
  --allowlist-function compressor_params_clear \
  \
  --allowlist-function compressor_params_get_or_create_source_image \
  --allowlist-function compressor_params_resize_source_image_list \
  --allowlist-function compressor_params_clear_source_image_list \
  \
  --allowlist-function compressor_params_set_status_output \
  --allowlist-function compressor_params_set_quality_level \
  --allowlist-function compressor_params_get_pack_uastc_flags \
  --allowlist-function compressor_params_set_pack_uastc_flags \
  --allowlist-function compressor_params_set_uastc \
  --allowlist-function compressor_params_set_perceptual \
  --allowlist-function compressor_params_set_mip_srgb \
  --allowlist-function compressor_params_set_no_selector_rdo \
  --allowlist-function compressor_params_set_no_endpoint_rdo \
  --allowlist-function compressor_params_set_rdo_uastc \
  --allowlist-function compressor_params_set_rdo_uastc_quality_scalar \
  --allowlist-function compressor_params_set_generate_mipmaps \
  --allowlist-function compressor_params_set_mip_smallest_dimension \
  --allowlist-function compressor_params_set_userdata \
  \
  --allowlist-function compressor_new \
  --allowlist-function compressor_delete \
  --allowlist-function compressor_init \
  --allowlist-function compressor_process \
  --allowlist-function compressor_get_output_basis_file \
  \
  --allowlist-function compressor_get_basis_file_size \
  --allowlist-function compressor_get_basis_bits_per_texel \
  --allowlist-function compressor_get_any_source_image_has_alpha \
  \
  --allowlist-function basisu_encoder_init \
  \
  --allowlist-var basisu::BASISU_MAX_SUPPORTED_TEXTURE_DIMENSION \
  --allowlist-var basisu::BASISU_DEFAULT_ENDPOINT_RDO_THRESH \
  --allowlist-var basisu::BASISU_DEFAULT_SELECTOR_RDO_THRESH \
  --allowlist-var basisu::BASISU_DEFAULT_QUALITY \
  --allowlist-var basisu::BASISU_DEFAULT_HYBRID_SEL_CB_QUALITY_THRESH \
  --allowlist-var basisu::BASISU_MAX_IMAGE_DIMENSION \
  --allowlist-var basisu::BASISU_QUALITY_MIN \
  --allowlist-var basisu::BASISU_QUALITY_MAX \
  --allowlist-var basisu::BASISU_MAX_ENDPOINT_CLUSTERS \
  --allowlist-var basisu::BASISU_MAX_SELECTOR_CLUSTERS \
  --allowlist-var basisu::BASISU_MAX_SLICES \
  --allowlist-var basisu::BASISU_RDO_UASTC_DICT_SIZE_DEFAULT \
  --allowlist-var basisu::BASISU_RDO_UASTC_DICT_SIZE_MIN \
  --allowlist-var basisu::BASISU_RDO_UASTC_DICT_SIZE_MAX \
  --allowlist-var basisu::TOTAL_PACK_UASTC_LEVELS \
  \
  --opaque-type CompressorParams \
  --opaque-type Compressor \
  --opaque-type basisu::image \
  \
  -- -x c++ -std=c++14
