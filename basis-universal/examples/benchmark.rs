use basis_universal::{
    BasisTextureFormat, Compressor, CompressorParams, TranscodeParameters, Transcoder,
    TranscoderTextureFormat,
};
use image::{DynamicImage, GenericImageView};
use std::io::Write;

// This is not a proper benchmark, just a quick program for feeling out how options affect
// encode/transcode time in a rough, orders-of-magnitude way. (I suggest changing the texture to
// something larger).

pub fn main() {
    //
    // Read the PNG file from disk
    //
    let source_file = include_bytes!("../test_assets/rust-logo-256x256.png");
    let source_file_format = image::ImageFormat::Png;

    let t0 = std::time::Instant::now();
    let image_data = image::load_from_memory_with_format(source_file, source_file_format).unwrap();
    let t1 = std::time::Instant::now();

    let source_file_size = source_file.len();
    let source_file_decode_time = (t1 - t0).as_secs_f64() * 1000.0;
    let uncompressed_memory_size = image_data.as_bytes().len();

    println!(
        "Using {:?} file as source, decoded {} bytes in {} ms",
        source_file_format, source_file_size, source_file_decode_time
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

    let compression_tests = vec![
        // (BasisTextureFormat::ETC1S, basis_universal::ETC1S_QUALITY_MIN, None),
        // (BasisTextureFormat::ETC1S, basis_universal::ETC1S_QUALITY_DEFAULT, None),
        // (BasisTextureFormat::ETC1S, basis_universal::ETC1S_QUALITY_MAX, None),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_MIN, None),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_DEFAULT, None),
        // (BasisTextureFormat::UASTC4x4, basis_universal::sys::UastcPackFlags_PackUASTCLevelSlower, None),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_MIN, Some(0.5)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_DEFAULT, Some(0.5)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::sys::UastcPackFlags_PackUASTCLevelSlower, Some(0.5)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_MIN, Some(1.0)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_DEFAULT, Some(1.0)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::sys::UastcPackFlags_PackUASTCLevelSlower, Some(1.0)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_MIN, Some(4.0)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::UASTC_QUALITY_DEFAULT, Some(4.0)),
        // (BasisTextureFormat::UASTC4x4, basis_universal::sys::UastcPackFlags_PackUASTCLevelSlower, Some(4.0)),
    ];

    let compressor_thread_count = 1;

    println!("source_file_size: {} KB", source_file_size / 1024);
    println!("source_file_decode_time: {}  ms", source_file_decode_time);
    println!(
        "uncompressed_memory_size: {} KB",
        uncompressed_memory_size / 1024
    );
    println!(
        "size: {}x{} channels: {}",
        image_data.width(),
        image_data.height(),
        channel_count
    );
    for (format, quality, rdo_scalar) in compression_tests {
        benchmark_encode(
            &image_data,
            channel_count,
            format,
            quality,
            rdo_scalar,
            compressor_thread_count,
        );
    }

    let transcode_tests = vec![
        (
            BasisTextureFormat::ETC1S,
            basis_universal::ETC1S_QUALITY_MIN,
            None,
        ),
        (
            BasisTextureFormat::ETC1S,
            basis_universal::ETC1S_QUALITY_DEFAULT,
            None,
        ),
        (
            BasisTextureFormat::UASTC4x4,
            basis_universal::UASTC_QUALITY_MIN,
            None,
        ),
        (
            BasisTextureFormat::UASTC4x4,
            basis_universal::UASTC_QUALITY_DEFAULT,
            None,
        ),
    ];

    for (format, quality, rdo_scalar) in transcode_tests {
        benchmark_transcode(
            &image_data,
            channel_count,
            format,
            quality,
            rdo_scalar,
            compressor_thread_count,
        );
    }
}

fn benchmark_encode(
    image_data: &DynamicImage,
    channel_count: u8,
    basis_texture_format: BasisTextureFormat,
    quality: u32,
    rdo_scalar: Option<f32>,
    compressor_thread_count: u32,
) {
    let mut compressor_params = CompressorParams::new();
    compressor_params.set_generate_mipmaps(false);
    compressor_params.set_basis_format(basis_texture_format);
    compressor_params.set_rdo_uastc(rdo_scalar);

    match basis_texture_format {
        BasisTextureFormat::ETC1S => compressor_params.set_etc1s_quality_level(quality),
        BasisTextureFormat::UASTC4x4 => compressor_params.set_uastc_quality_level(quality),
    }

    compressor_params.set_print_status_to_stdout(false);

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
    let mut compressor = Compressor::new(compressor_thread_count);
    let compression_time = unsafe {
        compressor.init(&compressor_params);
        let t0 = std::time::Instant::now();
        compressor.process().unwrap();
        let t1 = std::time::Instant::now();
        t1 - t0
    };

    println!(
        "Compression time for format {:?} quality: {} rdo: {:?} compressor thread count: {} {}ms",
        basis_texture_format,
        quality,
        rdo_scalar,
        compressor_thread_count,
        compression_time.as_secs_f32() * 1000.0
    );

    println!(
        "  basis compressed size: {} KB",
        compressor.basis_file_size() / 1024
    );

    // LZ4 compression is recommended for UASTC4x4 files
    if basis_texture_format == BasisTextureFormat::UASTC4x4 {
        let mut encoder = lz4::EncoderBuilder::new().build(Vec::new()).unwrap();
        encoder.write(compressor.basis_file()).unwrap();
        let (lz4_compressed, result) = encoder.finish();
        result.unwrap();
        println!("  lz4 compressed size: {} KB", lz4_compressed.len() / 1024);

        let t0 = std::time::Instant::now();
        let decoder = lz4::Decoder::new(&*lz4_compressed).unwrap();
        let (_, result) = decoder.finish();
        result.unwrap();
        let t1 = std::time::Instant::now();

        println!(
            "    Decompression time: {}ms",
            (t1 - t0).as_secs_f32() * 1000.0
        );
    }
}

fn benchmark_transcode(
    image_data: &DynamicImage,
    channel_count: u8,
    basis_texture_format: BasisTextureFormat,
    quality: u32,
    rdo_scalar: Option<f32>,
    compressor_thread_count: u32,
) {
    let mut compressor_params = CompressorParams::new();
    compressor_params.set_generate_mipmaps(false);
    compressor_params.set_basis_format(basis_texture_format);
    compressor_params.set_rdo_uastc(rdo_scalar);

    match basis_texture_format {
        BasisTextureFormat::ETC1S => compressor_params.set_etc1s_quality_level(quality),
        BasisTextureFormat::UASTC4x4 => compressor_params.set_uastc_quality_level(quality),
    }

    compressor_params.set_print_status_to_stdout(false);

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
    let mut compressor = Compressor::new(compressor_thread_count);
    let compression_time = unsafe {
        compressor.init(&compressor_params);
        let t0 = std::time::Instant::now();
        compressor.process().unwrap();
        let t1 = std::time::Instant::now();
        t1 - t0
    };

    println!(
        "Transcode test for format {:?} quality: {} rdo: {:?} compressor thread count: {} compression time: {}ms Compressed size: {} KB",
        basis_texture_format,
        quality,
        rdo_scalar,
        compressor_thread_count,
        compression_time.as_secs_f32() * 1000.0,
        compressor.basis_file_size() / 1024
    );

    let mut transcoder = Transcoder::new();

    let transcode_formats = vec![
        TranscoderTextureFormat::ETC1_RGB,
        TranscoderTextureFormat::BC1_RGB,
        TranscoderTextureFormat::BC7_RGBA,
        TranscoderTextureFormat::ASTC_4x4_RGBA,
        TranscoderTextureFormat::RGBA32,
    ];

    for transcode_format in transcode_formats {
        //
        // Now lets transcode it back to raw images
        //
        let t0 = std::time::Instant::now();
        transcoder
            .prepare_transcoding(compressor.basis_file())
            .unwrap();

        let result = transcoder
            .transcode_image_level(
                compressor.basis_file(),
                transcode_format,
                TranscodeParameters {
                    image_index: 0,
                    level_index: 0,
                    ..Default::default()
                },
            )
            .unwrap();
        let t1 = std::time::Instant::now();

        println!(
            "    Transcoded to {:?}: {} KB in {} ms",
            transcode_format,
            result.len() / 1024,
            (t1 - t0).as_secs_f64() * 1000.0
        );
    }
}
