use libloading::Library;

#[test]
fn test_linkage() {
    // Determine the library path relative to the workspace root.
    let lib_path = if cfg!(target_os = "windows") {
        "../../target/release/rust_core.dll"
    } else if cfg!(target_os = "macos") {
        "../../target/release/librust_core.dylib"
    } else {
        "../../target/release/librust_core.so"
    };

    let lib = Library::new(lib_path).expect("load rust_core library");
    unsafe {
        let validate: libloading::Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"validate_vov_log\0").expect("validate_vov_log symbol");
        let result = std::panic::catch_unwind(|| validate());
        assert!(result.is_ok());

        let sign: libloading::Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"generate_signature\0").expect("generate_signature symbol");
        let result2 = std::panic::catch_unwind(|| sign());
        assert!(result2.is_ok());
    }
}
