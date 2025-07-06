// 📄 rust-core/tests/test_link.rs

/// Test the direct `extern "C"` bindings to ensure symbols are correctly exported.
#[test]
fn test_example_function_and_force_disconnect() {
    extern "C" {
        fn example_function();
        fn add_numbers(a: i32, b: i32) -> i32;
        fn force_disconnect();
    }

    unsafe {
        example_function(); // DLL側が正しく呼ばれるか
        let result = add_numbers(2, 3);
        assert_eq!(result, 5, "add_numbers should return 2 + 3 = 5");

        force_disconnect(); // Go側から呼ぶ想定
    }
}

/// Test dynamic linkage using libloading for cross-platform compatibility.
#[test]
fn test_dynamic_linkage() {
    use libloading::Library;

    // Determine the library path relative to the workspace root.
    let lib_path = if cfg!(target_os = "windows") {
        "../../target/release/rust_core.dll"
    } else if cfg!(target_os = "macos") {
        "../../target/release/librust_core.dylib"
    } else {
        "../../target/release/librust_core.so"
    };

    let lib = Library::new(lib_path).expect("Failed to load rust_core library");

    unsafe {
        let add_numbers: libloading::Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            lib.get(b"add_numbers\0").expect("add_numbers symbol");
        assert_eq!(add_numbers(10, 5), 15);

        let example_function: libloading::Symbol<unsafe extern "C" fn()> =
            lib.get(b"example_function\0").expect("example_function symbol");
        example_function();

        let force_disconnect: libloading::Symbol<unsafe extern "C" fn()> =
            lib.get(b"force_disconnect\0").expect("force_disconnect symbol");
        force_disconnect();
    }
}
