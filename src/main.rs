use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod hypergraph;

/// A CLI for interacting with the Manifold Hypergraph library
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,

    /// Emit pretty-printed JSON
    #[arg(long, global = true)]
    pretty: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a hypergraph definition
    Create {
        /// A name for the hypergraph
        #[arg(long)]
        name: String,

        /// Comma-separated nodes (e.g. "a,b,c")
        #[arg(long, value_delimiter = ',')]
        nodes: Vec<String>,

        /// Edge definitions like "a,b,c" (repeatable)
        #[arg(long = "edge")]
        edges: Vec<String>,

        /// Optional JSON metadata blob
        #[arg(long)]
        metadata: Option<String>,
    },
    /// Load a hypergraph JSON file
    Load {
        /// Path to a hypergraph JSON file
        #[arg(long)]
        input: PathBuf,
    },
    /// Query hypergraph statistics or a node
    Query {
        /// Path to a hypergraph JSON file
        #[arg(long)]
        input: PathBuf,

        /// Optional node name to inspect
        #[arg(long)]
        node: Option<String>,
    },
    /// Export a hypergraph JSON file
    Export {
        /// Path to a hypergraph JSON file
        #[arg(long)]
        input: PathBuf,

        /// Destination path for export
        #[arg(long)]
        output: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        let error = serde_json::json!({
            "status": "error",
            "message": err.to_string(),
        });
        eprintln!("{}", format_output(&error, true));
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let output = match args.command {
        Commands::Create {
            name,
            nodes,
            edges,
            metadata,
        } => commands::create(commands::CreateArgs {
            name,
            nodes,
            edges,
            metadata,
        }),
        Commands::Load { input } => commands::load(commands::LoadArgs { input }),
        Commands::Query { input, node } => commands::query(commands::QueryArgs { input, node }),
        Commands::Export { input, output } => commands::export(commands::ExportArgs {
            input,
            output,
            pretty: args.pretty,
        }),
    }?;

    println!("{}", format_output(&output, args.pretty));
    Ok(())
}

fn format_output(value: &serde_json::Value, pretty: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    } else {
        serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
    }
}
