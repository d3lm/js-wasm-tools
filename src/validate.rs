use js_sys::Object;
use serde::Deserialize;
use std::mem;
use wasm_bindgen::prelude::*;
use wasmparser::{BinaryReaderError, FuncValidatorAllocations, Parser, ValidPayload, Validator, WasmFeatures};

use crate::serialize::Types;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
interface Features {
  mutable_global?: boolean;
  saturating_float_to_int?: boolean;
  sign_extension?: boolean;
  reference_types?: boolean;
  multi_value?: boolean;
  bulk_memory?: boolean;
  simd?: boolean;
  relaxed_simd?: boolean;
  threads?: boolean;
  tail_call?: boolean;
  deterministic_only?: boolean;
  multi_memory?: boolean;
  exceptions?: boolean;
  memory64?: boolean;
  extended_const?: boolean;
  component_model?: boolean;
}

type ValueType = 'i32' | 'i64' | 'f32' | 'f64' | 'v128' | 'funcref' | 'externref';

interface Types {
  types: Array<{ params: ValueType[]; results: ValueType[] }>;
  functions: Array<{ params: ValueType[]; results: ValueType[] }>;
  globals: Array<{ content_type: string; mutable: boolean }>;
  memories: Array<{ memory64: boolean; shared: boolean; initial: number; maximum?: number }>;
  tables: Array<{ element_type: string; initial: number; maximum?: number }>;
  elements: ValueType[],
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(extends = Object, typescript_type = "Features")]
  pub type ExternFeatures;

  #[wasm_bindgen(extends = Object, typescript_type = "Types")]
  pub type ExternTypes;
}

#[derive(Deserialize)]
pub struct Features {
  pub mutable_global: Option<bool>,
  pub saturating_float_to_int: Option<bool>,
  pub sign_extension: Option<bool>,
  pub reference_types: Option<bool>,
  pub multi_value: Option<bool>,
  pub bulk_memory: Option<bool>,
  pub simd: Option<bool>,
  pub relaxed_simd: Option<bool>,
  pub threads: Option<bool>,
  pub tail_call: Option<bool>,
  pub deterministic_only: Option<bool>,
  pub multi_memory: Option<bool>,
  pub exceptions: Option<bool>,
  pub memory64: Option<bool>,
  pub extended_const: Option<bool>,
  pub component_model: Option<bool>,
  pub component_model_values: Option<bool>,
  pub component_model_nested_names: Option<bool>,
  pub floats: Option<bool>,
  pub function_references: Option<bool>,
  pub memory_control: Option<bool>,
  pub gc: Option<bool>,
}

impl Into<WasmFeatures> for Features {
  fn into(self) -> WasmFeatures {
    WasmFeatures {
      mutable_global: self.mutable_global.unwrap_or_default(),
      saturating_float_to_int: self.saturating_float_to_int.unwrap_or_default(),
      sign_extension: self.sign_extension.unwrap_or_default(),
      reference_types: self.reference_types.unwrap_or_default(),
      multi_value: self.multi_value.unwrap_or_default(),
      bulk_memory: self.bulk_memory.unwrap_or_default(),
      simd: self.simd.unwrap_or_default(),
      relaxed_simd: self.relaxed_simd.unwrap_or_default(),
      threads: self.threads.unwrap_or_default(),
      tail_call: self.tail_call.unwrap_or_default(),
      multi_memory: self.multi_memory.unwrap_or_default(),
      exceptions: self.exceptions.unwrap_or_default(),
      memory64: self.memory64.unwrap_or_default(),
      extended_const: self.extended_const.unwrap_or_default(),
      component_model: self.component_model.unwrap_or_default(),
      component_model_values: self.component_model_values.unwrap_or_default(),
      component_model_nested_names: self.component_model_nested_names.unwrap_or_default(),
      floats: self.floats.unwrap_or_default(),
      function_references: self.function_references.unwrap_or_default(),
      memory_control: self.memory_control.unwrap_or_default(),
      gc: self.gc.unwrap_or_default(),
    }
  }
}

#[wasm_bindgen]
pub fn validate(bytes: Vec<u8>, features: Option<ExternFeatures>) -> Result<ExternTypes, JsValue> {
  let features = match features {
    Some(features) => {
      if features.is_undefined() {
        WasmFeatures::default().into()
      } else {
        match serde_wasm_bindgen::from_value::<Features>(features.into()) {
          Ok(features) => features.into(),
          Err(error) => {
            web_sys::console::error_1(&format!("{}, using default features", error.to_string()).into());
            WasmFeatures::default().into()
          }
        }
      }
    }
    None => WasmFeatures::default().into(),
  };

  let mut validator = Validator::new_with_features(features);
  let mut functions_to_validate = Vec::new();

  let mut last_types: Option<Types> = None;

  for payload in Parser::new(0).parse_all(&bytes) {
    match validator.payload(&payload.map_err(cast_error)?).map_err(cast_error)? {
      ValidPayload::End(types) => {
        last_types = Some(Types(types));
      }
      ValidPayload::Func(validator, body) => functions_to_validate.push((validator, body)),
      _ => {}
    }
  }

  let mut allocs = FuncValidatorAllocations::default();

  for (to_validate, body) in functions_to_validate {
    let mut validator = to_validate.into_validator(mem::take(&mut allocs));

    validator.validate(&body).map_err(cast_error)?;

    allocs = validator.into_allocations();
  }

  let types = serde_wasm_bindgen::to_value(&last_types.unwrap())?;

  Ok(types.into())
}

fn cast_error(error: BinaryReaderError) -> JsValue {
  error.to_string().into()
}
