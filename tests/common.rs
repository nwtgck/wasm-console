use wasm_console;

pub fn test_console() {
    wasm_console::log!("console.log():", true, 1.3);
    wasm_console::debug!("console.debug():", true, 1.3);
    wasm_console::info!("console.info():", true, 1.3);
    wasm_console::warn!("console.warn():", true, 1.3);
    wasm_console::error!("console.error():", true, 1.3);
    wasm_console::assert!(false, "console.assert():", "NOTE: This 'Assertion failed' is caused by console.assert(), not test failure", true, 1.3);
    wasm_console::assert!(0, "console.assert():", "NOTE: This 'Assertion failed' is caused by console.assert(), not test failure", true, 1.3);
    wasm_console::trace!("console.trace():", true, 1.3);
}
