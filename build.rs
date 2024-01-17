use cc::Build;

fn main() {
    let mut cx = cc::Build::new();
    let mut cxx = cc::Build::new();
    compile_metal(&mut cx, &mut cxx);
}
fn compile_metal(cx: &mut Build, cxx: &mut Build) {
    cx.flag("-DGGML_USE_METAL").flag("-DGGML_METAL_NDEBUG");
    cxx.flag("-DGGML_USE_METAL");

    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
    println!("cargo:rustc-link-lib=framework=MetalKit");

    cx.include("./llama.cpp/ggml-metal.h")
        .file("./llama.cpp/ggml-metal.m");
}
