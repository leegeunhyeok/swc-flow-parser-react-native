# swc-flow-parser-react-native

SWC Flow parser validation tool for React Native source files.

## Purpose

[swc-project/swc#11685](https://github.com/swc-project/swc/pull/11685) (merged 2026-03-16) added Flow parsing support to `swc_ecma_parser`. This tool validates that the new parser can correctly handle React Native's Flow source files from `node_modules/react-native/Libraries`.

## How it works

Two-stage pipeline:

1. **collect** — Walks `../node_modules/react-native/Libraries`, finds `.js` / `.js.flow` files containing `@flow`, copies them to `./flow/` with flattened filenames (`path/to/file.js` → `path_to_file.js`)
2. **parse** — Reads all files in `./flow/`, parses each with `Syntax::Flow` (jsx + all + enums enabled), writes a `report.json` summarizing successes and failures

## Prerequisites

- [mise](https://mise.jdx.dev/) — `mise install` to set up Node.js
- Rust toolchain (stable)

## Usage

```sh
mise trust && mise install

# Setup
just setup

# Collect flow files, then parse them
just run-all

# Or run each stage separately
just collect   # Stage 1: collect flow files → ./flow/
just parse     # Stage 2: parse and produce report.json
```

## Output

`report.json` contains:

```json
{
  "collected": 123,
  "failed": 4,
  "details": [
    {
      "source": "Components_View_View.js",
      "reason": "..."
    }
  ]
}
```

- `collected` — total number of files parsed
- `failed` — number of files that failed to parse
- `details` — list of failures with file name and error reason
