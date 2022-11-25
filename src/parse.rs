use js_sys::{Object, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
interface DesugarResult {
  bytes: Uint8Array;
  wat: string;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(extends = Object, typescript_type = "DesugarResult")]
  pub type DesugarResult;

  #[wasm_bindgen(method, setter)]
  fn set_bytes(this: &DesugarResult, value: &Uint8Array);

  #[wasm_bindgen(method, setter)]
  fn set_wat(this: &DesugarResult, value: &str);
}

#[wasm_bindgen(js_name = "desugarWat")]
pub fn desugar_wat(str: String) -> Result<DesugarResult, JsValue> {
  let bytes = wat::parse_str(str).map_err(|error| error.to_string())?;
  let wat = wasmprinter::print_bytes(&bytes).map_err(|error| error.to_string())?;

  let result = Object::new().unchecked_into::<DesugarResult>();

  result.set_bytes(&Uint8Array::from(&bytes[..]));
  result.set_wat(&wat);

  Ok(result)
}

#[wasm_bindgen(js_name = "parseWat")]
pub fn parse_wat(str: String) -> Result<Uint8Array, JsValue> {
  wat::parse_str(str)
    .map_err(|error| error.to_string().into())
    .map(|bytes| Uint8Array::from(&bytes[..]))
}

#[wasm_bindgen(js_name = "parseBytes")]
pub fn parse_bytes(bytes: Vec<u8>) -> Result<Uint8Array, JsValue> {
  wat::parse_bytes(&bytes)
    .map_err(|error| error.to_string().into())
    .map(|bytes| Uint8Array::from(&bytes[..]))
}

#[wasm_bindgen(js_name = "printBytes")]
pub fn print_bytes(bytes: Vec<u8>) -> Result<String, JsValue> {
  wasmprinter::print_bytes(&bytes).map_err(|error| error.to_string().into())
}
