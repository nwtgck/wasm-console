use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen(inline_js = "export function console_log() { return console.log; }")]
extern "C" {
    fn console_log() -> js_sys::Function;
}

fn js_sys_array<I: IntoIterator<Item=T>, T: Into<JsValue>>(into_iter: I) -> js_sys::Array {
    let js_arr = js_sys::Array::new();
    into_iter.into_iter().for_each(|x| {
        js_arr.push(&x.into());
    });
    js_arr
}

#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        console_log().apply(
            &JsValue::null(),
            &js_sys_array(&[$(
                Into::<wasm_bindgen::JsValue>::into($x),
            )*])
        )
    };
}
