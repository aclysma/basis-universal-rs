use basis_universal::{
    BasisTextureFormat, Compressor, CompressorParams, TranscodeParameters, Transcoder,
    TranscoderTextureFormat, UserData,
};
use image::GenericImageView;

// This example:
// - Loads a PNG file
// - Compresses it to basis-universal using UASTC basis format
// - Transcodes the compresses basis format to ASTC_4x4_RGBA and RGBA32
pub fn main() {
    //
    // Read the PNG file from disk
    //
    let png_file = include_bytes!("../test_assets/rust-logo-256x256.png");

    let t0 = std::time::Instant::now();
    let image_data =
        image::load_from_memory_with_format(png_file, image::ImageFormat::Png).unwrap();
    let t1 = std::time::Instant::now();

    println!(
        "Using PNG file as source, decoded {} bytes in {} ms",
        png_file.len(),
        (t1 - t0).as_secs_f64() * 1000.0
    );

    // We need to know how many color channels are in the image
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

    println!(
        "Going to encode {}x{} image with {} channels ({} uncompressed bytes)",
        image_data.width(),
        image_data.height(),
        channel_count,
        image_data.as_bytes().len()
    );

    //
    // Configure the compressor.. parameters chosen randomly for demonstration purposes
    //
    let mut compressor_params = CompressorParams::new();
    compressor_params.set_generate_mipmaps(true);
    compressor_params.set_basis_format(BasisTextureFormat::UASTC4x4);
    compressor_params.set_quality_level(Some(128));
    compressor_params.set_print_status_to_stdout(false);
    let userdata = UserData {
        userdata0: 100,
        userdata1: 200,
    };
    println!("Set userdata {:?}", userdata);
    compressor_params.set_userdata(userdata);

    //
    // Set the source image in the params
    //
    let mut compressor_image = compressor_params.source_image_mut(0);
    compressor_image.init(
        image_data.as_bytes(),
        image_data.width(),
        image_data.height(),
        channel_count,
    );

    //
    // Create the compressor and compress
    //
    let mut compressor = Compressor::default();
    let compression_time = unsafe {
        compressor.init(&compressor_params);
        let t0 = std::time::Instant::now();
        compressor.process().unwrap();
        let t1 = std::time::Instant::now();
        t1 - t0
    };

    // You could write it to disk like this
    let basis_file = compressor.basis_file();
    // std::fs::write("example_encoded_image.basis", basis_file).unwrap();

    let mut transcoder = Transcoder::new();
    let mip_level_count = transcoder.image_level_count(basis_file, 0);
    println!(
        "Compressed {} mip levels to {} total bytes in {} ms",
        mip_level_count,
        compressor.basis_file_size(),
        compression_time.as_secs_f64() * 1000.0
    );

    let userdata = transcoder.user_data(basis_file).unwrap();
    println!("Basis file has user data {:?}", userdata);

    //
    // Now lets transcode it back to raw images
    //
    transcoder.prepare_transcoding(basis_file).unwrap();

    let t0 = std::time::Instant::now();
    let result = transcoder
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
    let t1 = std::time::Instant::now();

    println!(
        "Transcoded mip level 0 to ASTC_4x4_RGBA: {} bytes {} ms",
        result.len(),
        (t1 - t0).as_secs_f64() * 1000.0
    );

    let t0 = std::time::Instant::now();
    let result = transcoder
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
    let t1 = std::time::Instant::now();

    println!(
        "Transcoded mip level 0 to RGBA32: {} bytes {} ms",
        result.len(),
        (t1 - t0).as_secs_f64() * 1000.0
    );

    transcoder.end_transcoding();

    let description = transcoder
        .image_level_description(basis_file, 0, 0)
        .unwrap();
    let _image = image::RgbaImage::from_raw(
        description.original_width,
        description.original_height,
        result,
    )
    .unwrap();
    // You could write it to disk like this
    //_image.save_with_format("example_transcoded_image.png", image::ImageFormat::Png).unwrap();
}
