# swc-flow-parser-react-native

Reproduction sample for `swc_ecma_parser` Flow parser issue.

## Purpose

Demonstrates that `swc_ecma_parser`'s Flow parsing support (`Syntax::Flow`) fails on valid Flow syntax found in `react-native@0.84.1`.

## How it works

1. **collect**: Walks `../node_modules/react-native/Libraries`, finds files containing `@flow`, copies them to `./flow/` with normalized filenames (`path/to/file.js` → `path_to_file.js`)
2. **parse**: Reads all files in `./flow/`, parses each with `Syntax::Flow(FlowSyntax { jsx, all, enums })`, writes results to `report.json`

## Usage

```sh
cargo run -- collect   # Stage 1: collect flow files
cargo run -- parse     # Stage 2: parse and produce report.json
```

## Key dependencies

- `swc_ecma_parser` v36 with `flow` + `typescript` features
- `swc_common` v19 for SourceMap
