use super::*;

#[test]
fn test_get_bytes_per_block_or_pixel() {
    assert_eq!(
        TranscoderTextureFormat::BC1_RGB.bytes_per_block_or_pixel(),
        8
    );
}

#[test]
fn test_get_format_name() {
    assert_eq!(TranscoderTextureFormat::BC1_RGB.format_name(), "BC1_RGB");
}

#[test]
fn test_transcoder_format_has_alpha() {
    assert_eq!(TranscoderTextureFormat::BC1_RGB.has_alpha(), false);
    assert_eq!(TranscoderTextureFormat::BC7_RGBA.has_alpha(), true);
}

#[test]
fn test_get_texture_type_name() {
    assert_eq!(BasisTextureType::TextureType2D.texture_type_name(), "2D");
}

#[test]
fn test_new_transcoder() {
    let transcoder = Transcoder::new();
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_get_total_images() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(transcoder.image_count(basis_file), 1);
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_info() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();

    let file_info = transcoder.file_info(basis_file).unwrap();

    // These should all return valid results
    assert!(transcoder.image_info(basis_file, 0).is_some());
    assert!(transcoder
        .image_level_description(basis_file, 0, 0)
        .is_some());
    assert!(transcoder.image_level_info(basis_file, 0, 0).is_some());

    // These return invalid results because we are passing image index > image count
    assert!(transcoder
        .image_info(basis_file, file_info.m_total_images + 1)
        .is_none());
    assert!(transcoder
        .image_level_description(basis_file, file_info.m_total_images + 1, 0)
        .is_none());
    assert!(transcoder
        .image_level_info(basis_file, file_info.m_total_images + 1, 0)
        .is_none());

    // These return invalid results because we are passing level index > level count
    assert!(transcoder
        .image_level_description(basis_file, 0, 100)
        .is_none());
    assert!(transcoder.image_level_info(basis_file, 0, 100).is_none());

    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_get_tex_format() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(
        transcoder.basis_texture_format(basis_file),
        BasisTextureFormat::ETC1S
    );
    std::mem::drop(transcoder);

    let basis_file = include_bytes!("../../test_assets/rust-logo-uastc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(
        transcoder.basis_texture_format(basis_file),
        BasisTextureFormat::UASTC4x4
    );
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_get_total_image_levels() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(transcoder.image_level_count(basis_file, 0), 7);
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_transcode_etc() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    do_test_transcoder_transcode(basis_file, "test_assets/test_transcode_image_etc.png");
}

#[test]
fn test_transcoder_transcode_uastc() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-uastc.basis");
    do_test_transcoder_transcode(basis_file, "test_assets/test_transcode_image_uastc.png");
}

// Transcode to a variety of formats
fn do_test_transcoder_transcode(
    basis_file: &[u8],
    _out_path: &str,
) {
    let mut transcoder = Transcoder::new();
    transcoder.prepare_transcoding(basis_file).unwrap();

    transcoder
        .transcode_image_level(
            basis_file,
            TranscoderTextureFormat::ETC1_RGB,
            TranscodeParameters {
                image_index: 0,
                level_index: 0,
                ..Default::default()
            },
        )
        .unwrap();

    transcoder
        .transcode_image_level(
            basis_file,
            TranscoderTextureFormat::ASTC_4x4_RGBA,
            TranscodeParameters {
                image_index: 0,
                level_index: 0,
                ..Default::default()
            },
        )
        .unwrap();

    if transcoder.basis_texture_format(basis_file) == BasisTextureFormat::ETC1S {
        transcoder
            .transcode_image_level(
                basis_file,
                TranscoderTextureFormat::FXT1_RGB,
                TranscodeParameters {
                    image_index: 0,
                    level_index: 0,
                    ..Default::default()
                },
            )
            .unwrap();
    }

    let _result = transcoder
        .transcode_image_level(
            basis_file,
            TranscoderTextureFormat::RGBA32,
            TranscodeParameters {
                image_index: 0,
                level_index: 0,
                ..Default::default()
            },
        )
        .unwrap();
    transcoder.end_transcoding();

    //let description = transcoder.image_level_description(basis_file, 0, 0).unwrap();
    //let image = image::RgbaImage::from_raw(description.original_width, description.original_height, _result).unwrap();
    //image.save_with_format(_out_path, image::ImageFormat::Png).unwrap();

    std::mem::drop(transcoder);
}
