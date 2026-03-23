# swc-flow-parser-react-native

SWC Flow parser validation tool for React Native source files.

## Purpose

Validates that `swc_ecma_parser`'s new Flow parsing support (PR #11685, merged 2026-03-16) can correctly parse React Native's Flow source files from `node_modules/react-native/Libraries`.

## How it works

1. **collect**: Walks `../node_modules/react-native/Libraries`, finds files containing `@flow`, copies them to `./flow/` with normalized filenames (`path/to/file.js` → `path_to_file.js`)
2. **parse**: Reads all files in `./flow/`, parses each with `Syntax::Flow(FlowSyntax { jsx, all, enums })`, writes results to `results.json`

## Usage

```sh
cargo run -- collect   # Stage 1: collect flow files
cargo run -- parse     # Stage 2: parse and produce results.json
```
