import initWasmTools, * as wasmTools from 'js-wasm-tools';
import WASM_TOOLS_WASM_URL from 'js-wasm-tools/wasm_tools_js_bg.wasm?url';

await initWasmTools(WASM_TOOLS_WASM_URL);

const source = `
  (module
    (type (func (param i32 i32)))
    (type $vec (array i32))
    (memory 1 100 shared)
    (table 1 1 funcref)
    (elem (i32.const 0) func $foo)
    (func $foo (type 0)
      i32.const 1
      block $foo
        br $foo
      end
      drop
    )
    (func $simd (export "simd") (result (ref $vec))
      (array.new_default $vec (i32.const 3))
    )
  )
`;

const bytes = wasmTools.parseWat(source);

console.log(wasmTools.validate(bytes, { gc: true, reference_types: true, threads: true }));

console.log(wasmTools.printBytes(bytes));
