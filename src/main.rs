use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use tree_sitter::{Language, Query, QueryCursor};

#[derive(Parser)]
#[command(name = "ghost", about = "Tree-sitter file skeletonizer for Claude Code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract function/class signatures with line numbers
    Signature { file: String },
    /// Read a specific line range from a file
    Read {
        file: String,
        /// Line range in format start-end (e.g. 10-25)
        #[arg(long)]
        lines: String,
    },
}

struct LangConfig {
    language: Language,
    query_str: &'static str,
    is_python: bool,
}

fn get_lang_config(ext: &str) -> Option<LangConfig> {
    match ext {
        "py" => Some(LangConfig {
            language: tree_sitter_python::language(),
            query_str: "(function_definition) @item (class_definition) @item",
            is_python: true,
        }),
        "js" => Some(LangConfig {
            language: tree_sitter_javascript::language(),
            query_str: "(function_declaration) @item (class_declaration) @item (method_definition) @item",
            is_python: false,
        }),
        "ts" => Some(LangConfig {
            language: tree_sitter_typescript::language_typescript(),
            query_str: "(function_declaration) @item (class_declaration) @item (method_definition) @item (interface_declaration) @item (type_alias_declaration) @item",
            is_python: false,
        }),
        "tsx" => Some(LangConfig {
            language: tree_sitter_typescript::language_tsx(),
            query_str: "(function_declaration) @item (class_declaration) @item (method_definition) @item (interface_declaration) @item (type_alias_declaration) @item",
            is_python: false,
        }),
        _ => None,
    }
}

fn extract_sig<'a>(line: &'a str, is_python: bool) -> &'a str {
    let delim = if is_python { ':' } else { '{' };
    if let Some(pos) = line.find(delim) {
        &line[..pos]
    } else {
        line
    }
}

fn cmd_signature(path: &str) {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let Some(config) = get_lang_config(ext) else {
        println!("// Ghost: .{ext} not supported — read full file");
        return;
    };

    let source = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error reading {path}: {e}");
            std::process::exit(1);
        }
    };

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&config.language).expect("failed to load language");
    let tree = parser.parse(&source, None).expect("failed to parse file");

    let query = Query::new(&config.language, config.query_str).expect("invalid query");
    let mut cursor = QueryCursor::new();

    let source_str = std::str::from_utf8(&source).unwrap_or("");
    let lines: Vec<&str> = source_str.lines().collect();

    let filename = Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(path);
    println!("// GHOST SIGNATURE: {filename} (bodies hidden)");

    let matches = cursor.matches(&query, tree.root_node(), source.as_slice());
    for m in matches {
        for cap in m.captures {
            let row = cap.node.start_position().row;
            let line = lines.get(row).copied().unwrap_or("");
            let sig = extract_sig(line, config.is_python).trim_end();
            let suffix = if config.is_python { " ..." } else { " { ... }" };
            println!("Line {}: {sig}{suffix}", row + 1);
        }
    }
}

fn cmd_read(path: &str, range: &str) {
    let (start, end) = match range.splitn(2, '-').collect::<Vec<_>>().as_slice() {
        [s, e] => {
            let s: usize = s.parse().unwrap_or_else(|_| { eprintln!("invalid start line"); std::process::exit(1); });
            let e: usize = e.parse().unwrap_or_else(|_| { eprintln!("invalid end line"); std::process::exit(1); });
            (s, e)
        }
        _ => {
            eprintln!("Invalid range '{range}'. Use start-end (e.g. 10-25)");
            std::process::exit(1);
        }
    };

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {path}: {e}");
            std::process::exit(1);
        }
    };

    for (i, line) in source.lines().enumerate() {
        let n = i + 1;
        if n >= start && n <= end {
            println!("{line}");
        }
        if n > end {
            break;
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Signature { file } => cmd_signature(&file),
        Commands::Read { file, lines } => cmd_read(&file, &lines),
    }
}
