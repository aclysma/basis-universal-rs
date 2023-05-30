// args from the basis cmake file
fn build_with_common_settings() -> cc::Build {
    let mut build = cc::Build::new();
    build
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wno-unused-local-typedefs")
        .flag_if_supported("-Wno-unused-value")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable");

    build
}

fn main() {
    // Only allow the SSE flag if we compile for x86/x86_64
    // See: https://github.com/aclysma/basis-universal-rs/pull/14#discussion_r1200978360
    let build_with_sse4_1 = cfg!(feature = "sse4_1")
        && std::env::var("CARGO_CFG_TARGET_ARCH")
            .expect("Cargo didn't set the CARGO_CFG_TARGET_ARCH env var")
            .split(',')
            .any(|arch| arch == "x86_64" || arch == "x86");

    let mut build = build_with_common_settings();

    if build_with_sse4_1 {
        build.define("BASISU_SUPPORT_SSE", "1");
    }

    build
        .cpp(true)
        //.define("BASISD_SUPPORT_KTX2_ZSTD", "0") TODO: expose in futher release
        .flag_if_supported("--std=c++11")
        .file("vendor/basis_universal/encoder/pvpngreader.cpp")
        .file("vendor/basis_universal/encoder/jpgd.cpp")
        .file("vendor/basis_universal/encoder/basisu_uastc_enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_ssim.cpp")
        .file("vendor/basis_universal/encoder/basisu_resampler.cpp")
        .file("vendor/basis_universal/encoder/basisu_resample_filters.cpp")
        .file("vendor/basis_universal/encoder/basisu_pvrtc1_4.cpp")
        .file("vendor/basis_universal/encoder/basisu_opencl.cpp")
        .file("vendor/basis_universal/encoder/basisu_kernels_sse.cpp")
        .file("vendor/basis_universal/encoder/basisu_gpu_texture.cpp")
        .file("vendor/basis_universal/encoder/basisu_frontend.cpp")
        .file("vendor/basis_universal/encoder/basisu_etc.cpp")
        .file("vendor/basis_universal/encoder/basisu_enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_comp.cpp")
        .file("vendor/basis_universal/encoder/basisu_bc7enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_basis_file.cpp")
        .file("vendor/basis_universal/encoder/basisu_backend.cpp")
        .file("vendor/basis_universal/transcoder/basisu_transcoder.cpp")
        .file("vendor/transcoding_wrapper.cpp")
        .file("vendor/encoding_wrapper.cpp")
        .compile("basisuniversal");

    // We regenerate binding code and check it in. (See generate_bindings.sh)
}
