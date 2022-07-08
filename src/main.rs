use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use anyhow::Result;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RShortConfig {
    api_key: String,
}

impl Default for RShortConfig {
    fn default() -> Self {
        RShortConfig {
            api_key: "".to_string()
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(subcommand)]
    Story(StorySubcommand),
    Config{
        #[clap(value_parser)]
        key: String,
    }
}

#[derive(Subcommand, Debug)]
enum StorySubcommand {
    Search {
        #[clap(value_parser)]
        query: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let mut cfg: RShortConfig = confy::load("rshort")?;

    if cfg.api_key.len() == 0 && !matches!(cli.command, Commands::Config { key: _ })  {
        println!("No API key configured. Provide one using 'rshort config'");
        return Ok(());
    }


    match &cli.command {
        Commands::Story(subcommand) => {
            match &subcommand {
                StorySubcommand::Search { query } => {
                    let search_result = story::search_stories(query, &cfg).await?;
                    if search_result.is_empty() {
                        println!("Search returned no results!");
                        return Ok(());
                    }
                    println!("Search returned {} results", search_result.len());
                    println!("==========================");
                    for story in search_result {
                        story.print_line();
                    }
                    println!("==========================");
                }
            };
        },
        Commands::Config { key } => {
            cfg.api_key = key.to_string();
            confy::store("rshort", cfg)?;
        }
    }

    Ok(())
}
