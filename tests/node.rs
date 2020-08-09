#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test;
use wasm_bindgen_test::*;
mod common;

#[wasm_bindgen_test]
fn test_console() {
    common::test_console();
}
