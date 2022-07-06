use clap::{Parser, Subcommand};

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
    Story{
        #[clap(value_parser = clap::value_parser!(u16).range(1..=9999))]
        id: u16
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Story { id } => {
            let story = story::fetch_story(id).await?;
            match story {
                Some(story) => {story.print_summary();}
                None => {println!("No story found with id {}", id);}
            }
        }
    }

    Ok(())
}
