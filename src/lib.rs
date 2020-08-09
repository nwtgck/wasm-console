use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
extern "C" {
    pub type Console;

    #[wasm_bindgen(static_method_of = Console, js_class = console, js_name = console, getter = log)]
    pub fn log() -> js_sys::Function;

    #[wasm_bindgen(static_method_of = Console, js_class = console, js_name = console, getter = debug)]
    pub fn debug() -> js_sys::Function;

    #[wasm_bindgen(static_method_of = Console, js_class = console, js_name = console, getter = info)]
    pub fn info() -> js_sys::Function;

    #[wasm_bindgen(static_method_of = Console, js_class = console, js_name = console, getter = warn)]
    pub fn warn() -> js_sys::Function;

    #[wasm_bindgen(static_method_of = Console, js_class = console, js_name = console, getter = error)]
    pub fn error() -> js_sys::Function;
}

pub mod __export {
    use wasm_bindgen::JsValue;
    use std::iter::FromIterator;

    pub fn console_log(args: &[JsValue]) {
        super::Console::log().apply(
            &JsValue::null(),
            &js_sys::Array::from_iter(args)
        ).unwrap();
    }

    pub fn console_debug(args: &[JsValue]) {
        super::Console::debug().apply(
            &JsValue::null(),
            &js_sys::Array::from_iter(args)
        ).unwrap();
    }

    pub fn console_info(args: &[JsValue]) {
        super::Console::info().apply(
            &JsValue::null(),
            &js_sys::Array::from_iter(args)
        ).unwrap();
    }

    pub fn console_warn(args: &[JsValue]) {
        super::Console::warn().apply(
            &JsValue::null(),
            &js_sys::Array::from_iter(args)
        ).unwrap();
    }

    pub fn console_error(args: &[JsValue]) {
        super::Console::error().apply(
            &JsValue::null(),
            &js_sys::Array::from_iter(args)
        ).unwrap();
    }
}

#[macro_export]
macro_rules! __js_value_array {
    ( $( $x:expr ),* ) => {
        [$(
            Into::<wasm_bindgen::JsValue>::into($x),
        )*]
    };
}

#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_log(
            &$crate::__js_value_array!($($x),*)
        )
    };
}

#[macro_export]
macro_rules! debug {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_debug(
            &$crate::__js_value_array!($($x),*)
        )
    };
}

#[macro_export]
macro_rules! info {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_info(
            &$crate::__js_value_array!($($x),*)
        )
    };
}

#[macro_export]
macro_rules! warn {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_warn(
            &$crate::__js_value_array!($($x),*)
        )
    };
}

#[macro_export]
macro_rules! error {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_error(
            &$crate::__js_value_array!($($x),*)
        )
    };
}
