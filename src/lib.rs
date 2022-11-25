use wasm_bindgen::prelude::*;

mod parse;
mod serialize;
mod validate;

pub use crate::parse::*;
pub use crate::validate::*;

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}
