use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use yoba::{parse_program, State};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn exec_program(code: &str) -> Result<String, JsValue> {
    let program = parse_program(code).map_err(|e| format!("{}", e))?;
    let mut state = State::new();
    state.eval(&program[..])?;
    String::from_utf8(state.out).map_err(|e| format!("{}", e).into())
}
