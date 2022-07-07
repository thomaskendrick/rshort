use clap::{Parser, Subcommand, Args};

pub mod story;

/// Simple command line app to get Shortcut stories
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(subcommand)]
    Story(StorySubcommand)
}

#[derive(Subcommand, Debug)]
enum StorySubcommand {
    Search {
        #[clap(value_parser)]
        query: String
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Story(subcommand)  => {
            match &subcommand {
                StorySubcommand::Search{query} => {
                    println!("Query: {}", query)
                }
            };
        }
    }

    Ok(())
}
