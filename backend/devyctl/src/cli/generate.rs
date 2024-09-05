use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct Generate {
    #[command(subcommand)]
    subject: Option<Subjects>,
}

#[derive(Subcommand)]
enum Subjects {
    EncodingKey,
}

pub async fn exec(generate: Generate) {
    match generate.subject {
        Some(Subjects::EncodingKey) => {
            println!("Generating encoding key");
        }
        None => println!("No subject specified"),
    }
}
