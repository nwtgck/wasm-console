use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
extern "C" {
    pub type Console;

    #[wasm_bindgen(variadic, js_namespace = console, js_name = assert)]
    pub fn console_assert(args: &js_sys::Array);
}

pub mod __export {
    use wasm_bindgen::JsValue;
    use std::iter::FromIterator;
    use web_sys::console;
    use js_sys::Array;

    pub fn console_log(args: &[JsValue]) {
        console::log(&Array::from_iter(args));
    }

    pub fn console_debug(args: &[JsValue]) {
        console::debug(&Array::from_iter(args));
    }

    pub fn console_info(args: &[JsValue]) {
        console::info(&Array::from_iter(args));
    }

    pub fn console_warn(args: &[JsValue]) {
        console::warn(&Array::from_iter(args));
    }

    pub fn console_error(args: &[JsValue]) {
        console::error(&Array::from_iter(args));
    }

    pub fn console_assert(args: &[JsValue]) {
        super::console_assert(&Array::from_iter(args));
    }

    pub fn console_trace(args: &[JsValue]) {
        console::trace(&Array::from_iter(args));
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

#[macro_export]
macro_rules! assert {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_assert(
            &$crate::__js_value_array!($($x),*)
        )
    };
}

#[macro_export]
macro_rules! trace {
    ( $( $x:expr ),* ) => {
        $crate::__export::console_trace(
            &$crate::__js_value_array!($($x),*)
        )
    };
}
