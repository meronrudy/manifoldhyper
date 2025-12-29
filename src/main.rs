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
        Commands::Serve { host, port } => {
            if let Err(error) = api::server::serve(host.clone(), *port).await {
                eprintln!("API server failed: {}", error);
            }
        }
    }
}
