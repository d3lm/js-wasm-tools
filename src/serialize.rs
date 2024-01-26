use serde::{Serialize, Serializer};
use wasmparser::{
  types::ComponentCoreTypeId, types::Types as ExternalTypes, CompositeType as ExternalCompositeType, ValType,
};

pub struct Types(pub(crate) ExternalTypes);

#[derive(Serialize)]
#[serde(tag = "type")]
enum CompositeType {
  Func(FuncType),
  Struct(StructType),
  Array(FieldType),
}

#[derive(Serialize)]
struct SerializedTypes {
  types: Vec<CompositeType>,
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
struct StructType {
  fields: Vec<FieldType>,
}

#[derive(Serialize)]
struct FieldType {
  element_type: String,
  mutable: bool,
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
    .map(|index| types.element_at(index).to_string())
    .collect::<Vec<String>>()
}

fn serialize_tables(types: &ExternalTypes) -> Vec<TableType> {
  (0..types.table_count() as u32)
    .map(|index| {
      let table = types.table_at(index);

      TableType {
        initial: table.initial,
        maximum: table.maximum,
        element_type: table.element_type.to_string(),
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
        content_type: global.content_type.to_string(),
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

fn serialize_types(types: &ExternalTypes) -> Vec<CompositeType> {
  (0..types.type_count() as u32)
    .map(|index| {
      let type_id = match types.core_type_at(index) {
        ComponentCoreTypeId::Sub(sub_type) => sub_type,
        ComponentCoreTypeId::Module(_) => panic!("type is expected to be a sub type"),
      };

      match &types[type_id].composite_type {
        ExternalCompositeType::Array(array_type) => CompositeType::Array(FieldType {
          element_type: array_type.0.element_type.to_string(),
          mutable: array_type.0.mutable,
        }),
        ExternalCompositeType::Struct(struct_type) => CompositeType::Struct(StructType {
          fields: struct_type
            .fields
            .iter()
            .map({
              |field_type| FieldType {
                element_type: field_type.element_type.to_string(),
                mutable: field_type.mutable,
              }
            })
            .collect(),
        }),
        ExternalCompositeType::Func(func_type) => CompositeType::Func(FuncType {
          params: serialize_val_types(func_type.params()),
          results: serialize_val_types(func_type.results()),
        }),
      }
    })
    .collect::<Vec<CompositeType>>()
}

fn serialize_val_types(val_types: &[ValType]) -> Vec<String> {
  val_types
    .iter()
    .map(|val_type| val_type.to_string())
    .collect::<Vec<String>>()
}
