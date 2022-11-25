# js-wasm-tools

`js-wasm-tools` compiles some of the API of [wasm-tools](https://github.com/bytecodealliance/wasm-tools) to JavaScript and WebAssembly via [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen). This offers low level tooling for WebAssembly in JavaScript, such as parsing WAT (WebAssembly Text Format) into bytes, translating the WebAssembly binary format to text, and more.

## Install

```sh
npm install js-wasm-tools
```

## Playground

You can try it out live on [StackBlitz.com](https://stackblitz.com/edit/js-wasm-tools?file=index.js).

## Usage

Using [Vite](https://vitejs.dev/):

```js
import initWasmTools, * as wasmTools from 'js-wasm-tools';
import WASM_TOOLS_WASM_URL from 'js-wasm-tools/wasm_tools_js_bg.wasm?url';

await initWasmTools(WASM_TOOLS_WASM_URL);

const source = '(module)';

const binary = wasmTools.parseWat(source);
```

With Node.js:

```js
import initWasmTools, * as wasmTools from 'js-wasm-tools';
import fs from 'node:fs';
import path from 'node:path';
import * as url from 'url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

const bytes = fs.readFileSync(path.join(__dirname, 'node_modules/js-wasm-tools/dist/js_wasm_tools_bg.wasm'));

await initWasmTools(bytes);

const source = '(module)';

const binary = wasmTools.parseWat(source);

console.log(binary);
```

## API

### parseWat(source: string): Uint8Array

Parses a string as the WebAssembly Text format, returning the WebAssembly binary format.

### parseBytes(bytes: Uint8Array): Uint8Array

Parses bytes as either the WebAssembly Text format, or a binary WebAssembly module.

This function will attempt to interpret the given bytes as one of two options:

- A utf-8 string which is a \*.wat file to be parsed
- A binary WebAssembly file starting with `b"\0asm"`

```js
const encoder = new TextEncoder();

const bytes = wasmTools.parseWat(encoder.encode('(module)'));

expect(bytes).toEqual([0, 97, 115, 109, 1, 0, 0, 0]);
```

```js
const bytes = wasmTools.parseWat([0, 97, 115, 109, 1, 0, 0, 0]);

expect(bytes).toEqual([0, 97, 115, 109, 1, 0, 0, 0]);
```

### printBytes(bytes: Uint8Array): string

Prints a Wasm binary blob into a string which is its textual representation.

```js
const wat = wasmTools.printBytes(new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]));

expect(wat).toEqual('(module)');
```

### desugarWat(source: string): { wat: string, bytes: Uint8Array }

Parses a string as the WebAssembly Text format, and desugars the module, e.g. unfolding expressions.

```js
const { wat, bytes } = wasmTools.printBytes(`
  (module
    (func $foo
      (call $bar (i32.const 1) (i32.const 2))
    )
    (func $bar (param i32 i32))
  )
`);

expect(wat).toEqual(`
  (module
    (type (;0;) (func))
    (type (;1;) (func (param i32 i32)))
    (func $foo (;0;) (type 0)
      i32.const 1
      i32.const 2
      call $bar
    )
    (func $bar (;1;) (type 1) (param i32 i32))
  )
`);
```

### validate(bytes: Uint8Array): Types

Test whether the given buffer contains a valid WebAssembly module or component, analogous to `WebAssembly.validate` in the JS API.

Upon success returns the type information for the top-level module or component.

```js
const encoder = new TextEncoder();

const types = wasmTools.validate(
  encoder.encode(`
    (module
      (func $foo (result f32)
        f32.const 1
      )
      (func $bar (param i32 i32))
    )
  `)
);

expect(types).toEqual({
  types: [
    {
      params: [],
      results: ['f32'],
    },
    {
      params: ['i32', 'i32'],
      results: [],
    },
  ],
  functions: [
    {
      params: [],
      results: ['f32'],
    },
    {
      params: ['i32', 'i32'],
      results: [],
    },
  ],
  globals: [],
  memories: [],
  tables: [],
  elements: [],
});
```
