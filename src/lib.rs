use wasm_bindgen::prelude::*;
use js_sys;
use std::iter::FromIterator;

#[wasm_bindgen]
extern "C" {
    pub type Console;
    #[wasm_bindgen(getter = log, static_method_of = Console, js_class = console, js_name = console)]
    pub fn log() -> js_sys::Function;
}

#[inline(always)]
fn console_log_with_slice(args: &[JsValue]) -> Result<JsValue, JsValue> {
    Console::log().apply(
        &JsValue::null(),
        &js_sys::Array::from_iter(args)
    )
}

#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        $crate::console_log_with_slice(&[$(
            Into::<wasm_bindgen::JsValue>::into($x),
        )*])
    };
}
