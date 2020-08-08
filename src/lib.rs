use wasm_bindgen::prelude::*;
use js_sys;
use std::iter::FromIterator;

#[wasm_bindgen]
extern "C" {
    pub type Console;
    #[wasm_bindgen(getter = log, static_method_of = Console, js_class = console, js_name = console)]
    pub fn log() -> js_sys::Function;
}

#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        $crate::Console::log().apply(
            &JsValue::null(),
            &$crate::js_sys::Array::from_iter(&[$(
                Into::<wasm_bindgen::JsValue>::into($x),
            )*])
        )
    };
}
