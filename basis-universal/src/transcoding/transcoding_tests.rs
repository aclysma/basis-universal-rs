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
fn test_get_block_format_name() {
    assert_eq!(
        TranscoderBlockFormat::ASTC_4x4.block_format_name(),
        "ASTC_4x4"
    );
}

#[test]
fn test_transcoder_format_has_alpha() {
    assert_eq!(TranscoderTextureFormat::BC1_RGB.has_alpha(), false);
    assert_eq!(TranscoderTextureFormat::BC7_RGBA.has_alpha(), true);
}

#[test]
fn test_get_basisu_texture_format() {
    assert_eq!(
        TranscoderTextureFormat::BC1_RGB.texture_format(),
        TextureFormat::BC1
    );
}

#[test]
fn test_get_texture_type_name() {
    assert_eq!(TextureType::TextureType2D.texture_type_name(), "2D");
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
    assert_eq!(transcoder.get_total_images(basis_file), 1);
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_get_tex_format() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(transcoder.get_tex_format(basis_file), TextureFormat::ETC1);
    std::mem::drop(transcoder);

    let basis_file = include_bytes!("../../test_assets/rust-logo-uastc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(transcoder.get_tex_format(basis_file), TextureFormat::ETC1S);
    std::mem::drop(transcoder);
}

#[test]
fn test_transcoder_get_total_image_levels() {
    let basis_file = include_bytes!("../../test_assets/rust-logo-etc.basis");
    let transcoder = Transcoder::new();
    assert_eq!(transcoder.get_total_image_levels(basis_file, 0), 7);
    std::mem::drop(transcoder);
}
