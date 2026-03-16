use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system rga shared library
    // Only link on Linux targets
    let target = env::var("TARGET").unwrap();

    if target.contains("linux") {
        println!("cargo:rustc-link-lib=rga");

        // Tell cargo to look for shared libraries in the specified directory
        let lib_dir = match target.as_str() {
            "aarch64-unknown-linux-gnu" => "libs/Linux/gcc-aarch64",
            "arm-unknown-linux-gnueabihf" => "libs/Linux/gcc-armhf",
            _ => {
                eprintln!("Warning: Unknown Linux target {}, using aarch64 library", target);
                "libs/Linux/gcc-aarch64"
            }
        };

        let lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(lib_dir);
        println!("cargo:rustc-link-search=native={}", lib_path.display());
    } else {
        // On non-Linux targets (macOS for development), skip library linking
        // bindgen will still generate the bindings
        println!("cargo:warning=Building on non-Linux target '{}'. RGA library linking skipped.", target);
    }

    // The bindgen::Builder is the main entry point to bindgen
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for
        .header("wrapper.h")
        // Tell bindgen to derive Default trait where possible
        .derive_default(true)
        // Tell bindgen to derive Debug trait where possible
        .derive_debug(true)
        // Use core instead of std for FFI types
        .use_core()
        // Blocklist some types that cause issues
        .blocklist_type("__uint128_t")
        // Clang arguments
        .clang_args(&["-I", "include"])
        // Tell bindgen to use usize/isize for size_t
        .size_t_is_usize(true)
        // Finish the builder and generate the bindings
        .generate()
        // Unwrap the result and panic on failure
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=include/im2d.h");
    println!("cargo:rerun-if-changed=include/im2d_type.h");
    println!("cargo:rerun-if-changed=include/im2d_single.h");
    println!("cargo:rerun-if-changed=include/im2d_buffer.h");
    println!("cargo:rerun-if-changed=include/im2d_task.h");
    println!("cargo:rerun-if-changed=include/im2d_common.h");
    println!("cargo:rerun-if-changed=include/rga.h");
}
