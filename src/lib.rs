use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
extern "C" {
    pub type Console;
    #[wasm_bindgen(getter = log, static_method_of = Console, js_class = console, js_name = console)]
    pub fn log() -> js_sys::Function;
}

pub fn js_sys_array<I: IntoIterator<Item=T>, T: Into<JsValue>>(into_iter: I) -> js_sys::Array {
    let js_arr = js_sys::Array::new();
    into_iter.into_iter().for_each(|x| {
        js_arr.push(&x.into());
    });
    js_arr
}

#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        $crate::Console::log().apply(
            &JsValue::null(),
            &$crate::js_sys_array(&[$(
                Into::<wasm_bindgen::JsValue>::into($x),
            )*])
        )
    };
}
