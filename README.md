# swc-flow-parser-react-native

Reproduction sample for [swc-project/swc#XXXX](https://github.com/swc-project/swc/issues/XXXX) — `swc_ecma_parser` Flow parser fails to parse valid Flow syntax.

## What this is

Minimal reproduction code that collects `@flow` annotated files from `react-native@0.84.1` and parses them with `swc_ecma_parser`'s `Syntax::Flow` mode, producing a failure report.

## How it works

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
  "collected": 466,
  "failed": 81,
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
