use clap::Parser;

mod api;
mod library;

/// A CLI for interacting with the Manifold Hypergraph library
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// A test command
    Test {
        /// A message to print
        #[arg(short, long)]
        message: String,
    },
    /// Run a demo workflow
    Demo {
        /// Name of the demo graph
        #[arg(long, default_value = "Demo Graph")]
        name: String,
        /// Query to run against the demo graph
        #[arg(long, default_value = "MATCH (n) RETURN n LIMIT 3")]
        query: String,
        /// Export format for the demo graph
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Run the API server
    Serve {
        /// Host to bind the API server
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind the API server
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Test { message } => {
            println!("Test command executed with message: {}", message);
        }
        Commands::Demo {
            name,
            query,
            format,
        } => {
            run_demo(name, query, format);
        }
        Commands::Serve { host, port } => {
            if let Err(error) = api::server::serve(host.clone(), *port).await {
                eprintln!("API server failed: {}", error);
            }
        }
    }
}

fn run_demo(name: &str, query: &str, format: &str) {
    println!("Running demo workflow...");

    let graph = match library::create_graph(name.to_string()) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("Failed to create graph: {:?}", error);
            return;
        }
    };
    println!("Created graph: {} ({})", graph.name, graph.id);

    let query_result = match library::run_query(graph.id.clone(), query.to_string()) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to run query: {:?}", error);
            return;
        }
    };
    println!("Query result: {}", query_result.result);

    let export_result = match library::export_graph(graph.id, format.to_string()) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to export graph: {:?}", error);
            return;
        }
    };
    println!(
        "Exported graph in {} format: {}",
        export_result.format, export_result.payload
    );
}
