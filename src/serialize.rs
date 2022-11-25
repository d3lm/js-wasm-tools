use serde::{Serialize, Serializer};
use wasmparser::{
  types::{Type, Types as ExternalTypes},
  ValType,
};

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
      ValType::FuncRef => String::from("funcref"),
      ValType::ExternRef => String::from("externref"),
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
    .filter_map(|index| match types.element_at(index) {
      Some(element) => Some(ValueType(&element).into()),
      _ => None,
    })
    .collect::<Vec<String>>()
}

fn serialize_tables(types: &ExternalTypes) -> Vec<TableType> {
  (0..types.table_count() as u32)
    .filter_map(|index| match types.table_at(index) {
      Some(table) => Some(TableType {
        initial: table.initial,
        maximum: table.maximum,
        element_type: ValueType(&table.element_type).into(),
      }),
      _ => None,
    })
    .collect::<Vec<TableType>>()
}

fn serialize_memories(types: &ExternalTypes) -> Vec<MemoryType> {
  (0..types.memory_count() as u32)
    .filter_map(|index| match types.memory_at(index) {
      Some(memory) => Some(MemoryType {
        memory64: memory.memory64,
        shared: memory.shared,
        initial: memory.initial,
        maximum: memory.maximum,
      }),
      _ => None,
    })
    .collect::<Vec<MemoryType>>()
}

fn serialize_globals(types: &ExternalTypes) -> Vec<GlobalType> {
  (0..types.global_count() as u32)
    .filter_map(|index| match types.global_at(index) {
      Some(global) => Some(GlobalType {
        content_type: ValueType(&global.content_type).into(),
        mutable: global.mutable,
      }),
      _ => None,
    })
    .collect::<Vec<GlobalType>>()
}

fn serialize_functions(types: &ExternalTypes) -> Vec<FuncType> {
  (0..types.function_count() as u32)
    .filter_map(|index| match types.function_at(index) {
      Some(func_type) => Some(FuncType {
        params: serialize_val_types(func_type.params()),
        results: serialize_val_types(func_type.results()),
      }),
      _ => None,
    })
    .collect::<Vec<FuncType>>()
}

fn serialize_types(types: &ExternalTypes) -> Vec<FuncType> {
  (0..types.type_count() as u32)
    .filter_map(|index| match types.type_at(index, true) {
      Some(Type::Func(func_type)) => Some(FuncType {
        params: serialize_val_types(func_type.params()),
        results: serialize_val_types(func_type.results()),
      }),
      _ => None,
    })
    .collect::<Vec<FuncType>>()
}

fn serialize_val_types(val_types: &[ValType]) -> Vec<String> {
  val_types
    .iter()
    .map(|val_type| ValueType(val_type).into())
    .collect::<Vec<String>>()
}
