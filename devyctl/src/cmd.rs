use crate::{
    config::Config,
    tools::{seed::*, *},
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cmd {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Seed {
        #[command(subcommand)]
        action: SeedAction,
    },
    Changes,
    Release,
}

#[derive(Subcommand, Debug)]
pub enum SeedAction {
    Generate {
        /// Name of the individual.
        #[arg(long, short)]
        name: String,
        /// Name of the blog.
        #[arg(long, short)]
        blog_name: String,
        /// Subject of the blog.
        #[arg(long, short)]
        subject: String,
        /// Number of posts to generate.
        #[arg(long, short)]
        post_count: usize,
    },
    Format,
}

impl Cmd {
    pub async fn exec(self, config: Config) -> Result<(), anyhow::Error> {
        match self.command {
            Some(Commands::Seed { action }) => match action {
                SeedAction::Generate {
                    name,
                    blog_name,
                    subject: blog_subject,
                    post_count,
                } => {
                    generate(
                        &config.openai_api_key,
                        Record {
                            name,
                            blog_name,
                            blog_subject,
                            post_count,
                        },
                    )
                    .await?
                }
                SeedAction::Format => format().await?,
            },
            Some(Commands::Changes) => changes(),
            Some(Commands::Release) => release(),
            None => println!("No command provided"),
        }

        Ok(())
    }
}
