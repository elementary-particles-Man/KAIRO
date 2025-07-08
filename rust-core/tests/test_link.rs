/*
// ===========================
// 📄 rust-core/tests/test_link.rs
// ===========================

use libloading::{Library, Symbol};

#[test]
fn test_example_function_and_force_disconnect() {
    // Determine the correct library file name based on the OS
    #[cfg(target_os = "windows")]
    let lib_name = "rust_core.dll";
    #[cfg(target_os = "macos")]
    let lib_name = "librust_core.dylib";
    #[cfg(target_os = "linux")]
    let lib_name = "librust_core.so";

    // Construct the full path to the library
    let current_exe_path = std::env::current_exe()
        .expect("Failed to get current executable path");
    let current_dir = current_exe_path.parent()
        .expect("Failed to get parent directory");
    let lib_path = current_dir.join(lib_name);

    // Load the library
    let lib = unsafe { Library::new(lib_path).expect("Failed to load rust_core library") };

    // Get the `example_function` symbol and call it
    let example_function: Symbol<unsafe extern "C" fn()> = unsafe {
        lib.get(b"example_function")
            .expect("Failed to find example_function")
    };
    unsafe { example_function(); }

    // Get the `add_numbers` symbol and call it
    let add_numbers: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = unsafe {
        lib.get(b"add_numbers")
            .expect("Failed to find add_numbers")
    };
    let result = unsafe { add_numbers(5, 7) };
    assert_eq!(result, 12);

    // Get the `force_disconnect` symbol and call it
    let force_disconnect: Symbol<unsafe extern "C" fn()> = unsafe {
        lib.get(b"force_disconnect")
            .expect("Failed to find force_disconnect")
    };
    unsafe { force_disconnect(); }
}
*/

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

    let lib = unsafe { Library::new(lib_path).expect("Failed to load rust_core library") };

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
