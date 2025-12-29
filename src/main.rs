use clap::Parser;

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
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Test { message } => {
            println!("Test command executed with message: {}", message);
        }
    }
}
