//! See also: crates/swift-bridge-ir/src/codegen/codegen_tests/option_codegen_tests.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_reflect_option_u8(arg: Option<u8>) -> Option<u8>;
        fn rust_reflect_option_i8(arg: Option<i8>) -> Option<i8>;
        fn rust_reflect_option_u16(arg: Option<u16>) -> Option<u16>;
        fn rust_reflect_option_i16(arg: Option<i16>) -> Option<i16>;
        fn rust_reflect_option_u32(arg: Option<u32>) -> Option<u32>;
        fn rust_reflect_option_i32(arg: Option<i32>) -> Option<i32>;
        fn rust_reflect_option_u64(arg: Option<u64>) -> Option<u64>;
        fn rust_reflect_option_i64(arg: Option<i64>) -> Option<i64>;
        fn rust_reflect_option_usize(arg: Option<usize>) -> Option<usize>;
        fn rust_reflect_option_isize(arg: Option<isize>) -> Option<isize>;
        fn rust_reflect_option_f32(arg: Option<f32>) -> Option<f32>;
        fn rust_reflect_option_f64(arg: Option<f64>) -> Option<f64>;
        fn rust_reflect_option_bool(arg: Option<bool>) -> Option<bool>;

        fn rust_reflect_option_string(arg: Option<String>) -> Option<String>;

        fn rust_create_option_static_str() -> Option<&'static str>;
        fn rust_reflect_option_str(arg: Option<&str>) -> Option<&str>;

        fn run_option_tests();
    }

    extern "Swift" {
        // TODO: Change these to use the same reflect pattern that we use above when we support
        //  extern "Swift" fn optional args.
        // fn create_swift_option_u8_some() -> Option<u8>;
        // fn create_swift_option_u8_none() -> Option<u8>;
    }
}

fn run_option_tests() {
    // assert_eq!(ffi::create_swift_option_u8_some(), Some(55));
    // assert_eq!(ffi::create_swift_option_u8_none(), None);
}

use self::reflect_primitives::*;
#[rustfmt::skip]
mod reflect_primitives {
    pub fn rust_reflect_option_u8(arg: Option<u8>) -> Option<u8> { arg }
    pub fn rust_reflect_option_i8(arg: Option<i8>) -> Option<i8> { arg }
    pub fn rust_reflect_option_u16(arg: Option<u16>) -> Option<u16> { arg }
    pub fn rust_reflect_option_i16(arg: Option<i16>) -> Option<i16> { arg }
    pub fn rust_reflect_option_u32(arg: Option<u32>) -> Option<u32> { arg }
    pub fn rust_reflect_option_i32(arg: Option<i32>) -> Option<i32> { arg }
    pub fn rust_reflect_option_u64(arg: Option<u64>) -> Option<u64> { arg }
    pub fn rust_reflect_option_i64(arg: Option<i64>) -> Option<i64> { arg }
    pub fn rust_reflect_option_usize(arg: Option<usize>) -> Option<usize> { arg }
    pub fn rust_reflect_option_isize(arg: Option<isize>) -> Option<isize> { arg }
    pub fn rust_reflect_option_f32(arg: Option<f32>) -> Option<f32> { arg }
    pub fn rust_reflect_option_f64(arg: Option<f64>) -> Option<f64> { arg }
    pub fn rust_reflect_option_bool(arg: Option<bool>) -> Option<bool> { arg }   
}

fn rust_reflect_option_string(arg: Option<String>) -> Option<String> {
    arg
}

fn rust_create_option_static_str() -> Option<&'static str> {
    Some("hello")
}
fn rust_reflect_option_str(arg: Option<&str>) -> Option<&str> {
    arg
}