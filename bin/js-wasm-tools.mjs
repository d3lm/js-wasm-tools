#!/usr/bin/env node

import fs from 'node:fs';
import { parseArgs } from 'node:util';
import initWasmTools, * as wasmTools from '../dist/js_wasm_tools.js';

class TextFormatter {
  #styles = [];

  get bold() {
    this.#styles.push('\x1b[1m');
    return this;
  }

  get underline() {
    this.#styles.push('\x1b[4m');
    return this;
  }

  format(text) {
    return this.#styles.join('') + text + '\x1b[0m';
  }
}

const formatter = new TextFormatter();

const { values, positionals } = parseArgs({
  args: process.argv.slice(2),
  allowPositionals: true,
  options: {
    wat: {
      type: 'boolean',
      short: 't',
    },
    output: {
      type: 'string',
      short: 'o',
    },
    help: {
      type: 'boolean',
      short: 'h',
    },
  },
});

_runCLI();

async function _runCLI() {
  if (values.help) {
    _printHelp(positionals[0]);
    return;
  }

  const bytes = fs.readFileSync('../dist/js_wasm_tools_bg.wasm');

  await initWasmTools(bytes);

  switch (positionals[0]) {
    case 'parse': {
      _parseFile(positionals[1], { wat: values.wat, output: values.output });
      break;
    }
    case 'print': {
      _parseFile(positionals[1], { wat: true, output: values.output });
      break;
    }
    case 'help': {
      _printHelp();
      break;
    }
    default: {
      _exitWithError(`Invalid command '${positionals[0]}'`);
    }
  }
}

function _readFile(filePath) {
  try {
    return fs.readFileSync(filePath);
  } catch (error) {
    _exitWithError(`Failed to read from '${filePath}'`);
  }
}

function _parseFile(filePath, options) {
  const rawBytes = _readFile(filePath);

  try {
    const wasmBytes = wasmTools.parseBytes(rawBytes);
    _handleOutput(wasmBytes, options);
  } catch {
    _exitWithError(`Failed to parse bytes`);
  }
}

function _handleOutput(bytes, { wat, output }) {
  const outputBytes = wat ? wasmTools.printBytes(bytes) : bytes;

  if (output) {
    fs.writeFileSync(output, outputBytes);
    return;
  }

  process.stdout.write(outputBytes + '\n');
}

function _exitWithError(message) {
  console.error(`Error: ${message}`);
  process.exit(1);
}

function _printHelp(command) {
  if (command) {
    switch (command) {
      case 'parse': {
        console.log(
          [
            `${formatter.bold.format(`${formatter.underline.format('Usage:')} js-wasm-tools parse [OPTIONS] [INPUT]`)}`,
            '',
            `${formatter.bold.format('Arguments:')}`,
            '  [INPUT]  Input file to process.',
            '',
            `${formatter.bold.format('Options:')}`,
            '  -o, --output  Where to place the output. If not provided then stdout is used.',
            '  -t, --wat     Output the text format of WebAssembly instead of the binary format.',
          ].join('\n')
        );

        return;
      }
      case 'print': {
        console.log(
          [
            `${formatter.bold.format(`${formatter.underline.format('Usage:')} js-wasm-tools print [OPTIONS] [INPUT]`)}`,
            '',
            `${formatter.bold.format('Arguments:')}`,
            '  [INPUT]  Input file to process.',
            '',
            `${formatter.bold.format('Options:')}`,
            '  -o, --output  Where to place the output. If not provided then stdout is used.',
          ].join('\n')
        );

        return;
      }
      default: {
        // unknown command so we fall through
        break;
      }
    }
  }

  console.log(
    [
      `${formatter.bold.format(`${formatter.underline.format('Usage:')} js-wasm-tools <COMMAND>`)}`,
      '',
      `${formatter.bold.underline.format('Commands:')}`,
      '  parse  Parse a file',
      '  print  Print the textual form of a WebAssembly binary',
    ].join('\n')
  );
}
