use serde::{Serialize, Serializer};
use wasmparser::{types::ComponentCoreTypeId, types::Types as ExternalTypes, ValType};

pub struct Types(pub(crate) ExternalTypes);

#[derive(Serialize)]
struct SerializedTypes {
  types: Vec<FuncType>,
  functions: Vec<FuncType>,
  globals: Vec<GlobalType>,
  memories: Vec<MemoryType>,
  tables: Vec<TableType>,
  elements: Vec<String>,
}

#[derive(Serialize)]
struct FuncType {
  params: Vec<String>,
  results: Vec<String>,
}

#[derive(Serialize)]
struct GlobalType {
  content_type: String,
  mutable: bool,
}

#[derive(Serialize)]
pub struct MemoryType {
  memory64: bool,
  shared: bool,
  initial: u64,
  maximum: Option<u64>,
}

#[derive(Serialize)]
pub struct TableType {
  element_type: String,
  initial: u32,
  maximum: Option<u32>,
}

struct ValueType<'a>(pub(crate) &'a ValType);

impl<'a> Into<String> for ValueType<'a> {
  fn into(self) -> String {
    match self.0 {
      ValType::I32 => String::from("i32"),
      ValType::I64 => String::from("i64"),
      ValType::F32 => String::from("f32"),
      ValType::F64 => String::from("f64"),
      ValType::V128 => String::from("v128"),
      ValType::Ref(ref_type) => ref_type.to_string(),
    }
  }
}

impl Serialize for Types {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let types = &self.0;

    let serialized_types = SerializedTypes {
      types: serialize_types(types),
      functions: serialize_functions(types),
      globals: serialize_globals(types),
      memories: serialize_memories(types),
      tables: serialize_tables(types),
      elements: serialize_elements(types),
    };

    serialized_types.serialize(serializer)
  }
}

fn serialize_elements(types: &ExternalTypes) -> Vec<String> {
  (0..types.element_count() as u32)
    .map(|index| ValueType(&ValType::Ref(types.element_at(index))).into())
    .collect::<Vec<String>>()
}

fn serialize_tables(types: &ExternalTypes) -> Vec<TableType> {
  (0..types.table_count() as u32)
    .map(|index| {
      let table = types.table_at(index);

      TableType {
        initial: table.initial,
        maximum: table.maximum,
        element_type: ValueType(&ValType::Ref(table.element_type)).into(),
      }
    })
    .collect::<Vec<TableType>>()
}

fn serialize_memories(types: &ExternalTypes) -> Vec<MemoryType> {
  (0..types.memory_count() as u32)
    .map(|index| {
      let memory = types.memory_at(index);

      MemoryType {
        memory64: memory.memory64,
        shared: memory.shared,
        initial: memory.initial,
        maximum: memory.maximum,
      }
    })
    .collect::<Vec<MemoryType>>()
}

fn serialize_globals(types: &ExternalTypes) -> Vec<GlobalType> {
  (0..types.global_count() as u32)
    .map(|index| {
      let global = types.global_at(index);

      GlobalType {
        content_type: ValueType(&global.content_type).into(),
        mutable: global.mutable,
      }
    })
    .collect::<Vec<GlobalType>>()
}

fn serialize_functions(types: &ExternalTypes) -> Vec<FuncType> {
  (0..types.core_function_count() as u32)
    .map(|index| {
      let function_type_id = types.core_function_at(index);
      let function = types[function_type_id].unwrap_func();

      FuncType {
        params: serialize_val_types(function.params()),
        results: serialize_val_types(function.results()),
      }
    })
    .collect::<Vec<FuncType>>()
}

fn serialize_types(types: &ExternalTypes) -> Vec<FuncType> {
  (0..types.type_count() as u32)
    .map(|index| {
      let type_id = match types.core_type_at(index) {
        ComponentCoreTypeId::Sub(sub_type) => sub_type,
        ComponentCoreTypeId::Module(_) => panic!("type is expected to be a sub type"),
      };

      let function = types[type_id].unwrap_func();

      FuncType {
        params: serialize_val_types(function.params()),
        results: serialize_val_types(function.results()),
      }
    })
    .collect::<Vec<FuncType>>()
}

fn serialize_val_types(val_types: &[ValType]) -> Vec<String> {
  val_types
    .iter()
    .map(|val_type| ValueType(val_type).into())
    .collect::<Vec<String>>()
}
