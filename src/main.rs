use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, FlowSyntax, Parser, StringInput, Syntax};
use walkdir::WalkDir;

#[derive(Serialize)]
struct Report {
    collected: u32,
    failed: u32,
    details: Vec<ParseResult>,
}

#[derive(Serialize)]
struct ParseResult {
    source: String,
    reason: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "collect" => collect(),
        "parse" => parse(),
        _ => {
            eprintln!("Usage: cargo run -- <collect|parse>");
            std::process::exit(1);
        }
    }
}

fn collect() {
    let libraries_dir = PathBuf::from("node_modules/react-native/Libraries");
    if !libraries_dir.exists() {
        eprintln!("Error: {} not found", libraries_dir.display());
        std::process::exit(1);
    }

    let flow_dir = PathBuf::from("flow");
    if flow_dir.exists() {
        fs::remove_dir_all(&flow_dir).expect("failed to clean flow/");
    }
    fs::create_dir_all(&flow_dir).expect("failed to create flow/");

    let mut count: usize = 0;

    for entry in WalkDir::new(&libraries_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !(name.ends_with(".js") || name.ends_with(".js.flow")) {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if !content.contains("@flow") {
            continue;
        }

        let relative = path.strip_prefix(&libraries_dir).unwrap().to_str().unwrap();
        let normalized = relative.replace('/', "_");
        fs::write(flow_dir.join(&normalized), &content).expect("failed to write file");
        count += 1;
    }

    println!("Collected {count} flow files");
}

fn parse() {
    let flow_dir = Path::new("flow");
    if !flow_dir.exists() {
        eprintln!("Error: flow/ directory not found. Run 'collect' first.");
        std::process::exit(1);
    }

    let cm: Lrc<SourceMap> = Default::default();
    let syntax = Syntax::Flow(FlowSyntax {
        // Enabled all Flow features
        jsx: true,
        enums: true,
        components: true,
        decorators: true,
        pattern_matching: true,
        // `@flow` directive checks
        all: true,
        require_directive: false,
    });

    let mut entries: Vec<fs::DirEntry> = fs::read_dir(flow_dir)
        .expect("failed to read flow/")
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut details: Vec<ParseResult> = Vec::new();
    let collected = entries.len() as u32;
    let mut fail = 0u32;

    for entry in &entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        let source = fs::read_to_string(&path).expect("failed to read file");
        let fm = cm.new_source_file(FileName::Custom(name.clone()).into(), source);

        let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
        let mut parser = Parser::new_from(lexer);

        if let Err(e) = parser.parse_module() {
            details.push(ParseResult {
                source: name,
                reason: format!("{:?}", e),
            });
            fail += 1;
        }
    }

    let report = Report {
        collected,
        failed: fail,
        details,
    };
    let json = serde_json::to_string_pretty(&report).expect("failed to serialize");
    fs::write("report.json", json).expect("failed to write report.json");

    let success = collected - fail;
    println!(
        "Parsed {collected} files: {success} succeeded, {fail} failed (Failure rate: {}%)",
        (fail as f64 / collected as f64 * 100.0).round()
    );
    println!("Results written to report.json");
}
