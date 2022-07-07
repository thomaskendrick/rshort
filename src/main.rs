use clap::{Parser, Subcommand};

pub mod story;

#[derive(Parser, Debug)]
#[clap(
    name = "rshort",
    version = "0.1",
    author = "Thomas Kendrick <tom@tkendrick.com>",
    about = "A simple command line application to get shortcut stories"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(subcommand)]
    Story(StorySubcommand),
}

#[derive(Subcommand, Debug)]
enum StorySubcommand {
    Search {
        #[clap(value_parser)]
        query: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Story(subcommand) => {
            match &subcommand {
                StorySubcommand::Search { query } => {
                    let search_result = story::search_stories(query).await?;
                    if search_result.is_empty() {
                        println!("Search returned no results!");
                        return Ok(());
                    }
                    for story in search_result {
                        story.print_line();
                    }
                }
            };
        }
    }

    Ok(())
}
