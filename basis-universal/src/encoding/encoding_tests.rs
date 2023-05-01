use super::*;
use crate::BasisTextureFormat;
use image::GenericImageView;

#[test]
fn test_new_compressor_params() {
    let compressor_params = CompressorParams::new();
    std::mem::drop(compressor_params);
}

#[test]
fn test_new_compressor() {
    let compressor = Compressor::default();
    std::mem::drop(compressor);
}

#[test]
fn test_compressor_init() {
    let compressor_params = CompressorParams::new();
    let mut compressor = Compressor::default();
    unsafe {
        compressor.init(&compressor_params);
    }
    std::mem::drop(compressor);
    std::mem::drop(compressor_params);
}

#[test]
fn test_compressor_params_smoketest_bindings() {
    let mut compressor_params = CompressorParams::new();

    // Call every parameter just to smoketest the bindings
    compressor_params.source_image_mut(5);
    compressor_params.resize_source_image_list(8);
    compressor_params.clear_source_image_list();
    compressor_params.set_print_status_to_stdout(false);
    compressor_params.set_etc1s_quality_level(crate::ETC1S_QUALITY_DEFAULT);
    compressor_params.set_uastc_quality_level(crate::UASTC_QUALITY_DEFAULT);
    compressor_params.set_basis_format(BasisTextureFormat::UASTC4x4);
    compressor_params.set_generate_mipmaps(true);

    compressor_params.reset();
}

#[test]
fn test_image_smoketest_bindings() {
    let mut compressor_params = CompressorParams::new();

    let mut image = compressor_params.source_image_mut(0);
    let color = image.pixel_at(50, 50);
    assert!(color.is_none());
    image.resize(100, 100);
    let color = image.pixel_at(50, 50);
    assert!(color.is_some());
    let _color = unsafe { image.pixel_at_unchecked(50, 50) };
    image.invalidate();
}

#[test]
fn test_encode_image() {
    //
    // Read the PNG file from disk
    //
    let png_file = include_bytes!("../../test_assets/rust-logo.png");
    let image_data =
        image::load_from_memory_with_format(png_file, image::ImageFormat::Png).unwrap();

    use image::ColorType;
    let channel_count = match &image_data.color() {
        ColorType::L8 => 1,
        ColorType::La8 => 2,
        ColorType::Rgb8 => 3,
        ColorType::Rgba8 => 4,
        ColorType::L16 => 1,
        ColorType::La16 => 2,
        ColorType::Rgb16 => 3,
        ColorType::Rgba16 => 4,
        ColorType::Bgr8 => 3,
        ColorType::Bgra8 => 4,
        _ => unimplemented!(),
    };

    let mut compressor_params = CompressorParams::new();
    compressor_params.set_generate_mipmaps(true);

    //
    // Set up the source image in the params
    //
    let mut compressor_image = compressor_params.source_image_mut(0);
    compressor_image.init(
        image_data.as_bytes(),
        image_data.width(),
        image_data.height(),
        channel_count,
    );

    //
    // Create the compressor
    //
    let mut compressor = Compressor::default();
    unsafe {
        compressor.init(&compressor_params);
    }
    // Drop explicitly here to verify that borrowing rules allow this and that this doesn't cause a crash
    std::mem::drop(compressor_params);

    //
    // Do the compression
    //
    unsafe {
        compressor.process().unwrap();
    }

    // By default the test shouldn't write to disk, but this is a quick way to put it on disk to
    // check that it works with basisu
    let _basis_file = compressor.basis_file();
    //std::fs::write("test_assets/test_encode_image.basis", basis_file).unwrap();

    std::mem::drop(compressor);
}
