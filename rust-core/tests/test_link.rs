// 📄 rust-core/tests/test_link.rs

#[test]
fn test_example_function() {
    extern "C" {
        fn example_function();
        fn add_numbers(a: i32, b: i32) -> i32;
    }

    unsafe {
        example_function(); // DLL側が正しく呼ばれるか
        let result = add_numbers(2, 3);
        assert_eq!(result, 5, "add_numbers should return 2 + 3 = 5");
    }
}

#[test]
fn test_force_disconnect() {
    extern "C" {
        fn force_disconnect();
    }

    unsafe {
        force_disconnect(); // Go側から呼ぶ想定
    }
}
