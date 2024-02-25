use std::io;

use clap::{Command, Parser};
use clap_complete::{generate, Generator, Shell};

#[derive(Parser, Debug)]
#[command(name = "human-ids", about, version, author)]
pub struct Cli {
    /// Generate shell completion scripts
    ///
    /// Generates shell completion scripts for the specified shell.
    #[clap(long, value_enum)]
    pub completion: Option<Shell>,

    /// The separator to use between words
    #[clap(long, short, default_value = "-")]
    pub separator: String,

    /// Capitalize each word
    #[clap(long, short)]
    pub capitalize: bool,

    /// Add an adverb
    #[clap(long, short, default_value = "false")]
    pub adverb: bool,

    /// The number of adjectives to use
    #[clap(long, short, default_value = "1")]
    pub num_adjectives: usize,
}

pub fn print_completion<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
