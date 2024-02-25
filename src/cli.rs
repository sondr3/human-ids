use std::io;

use clap::{ArgAction, Command, Parser};
use clap_complete::{generate, Generator, Shell};

#[derive(Parser, Debug)]
#[command(name = "changeset", about, version, author)]
pub struct Cli {
    /// Verbose output
    ///
    /// Controls verbosity of the application, where `-v` is DEBUG, `-vv` is TRACE.
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Generate shell completion scripts
    ///
    /// Generates shell completion scripts for the specified shell.
    #[clap(long, value_enum)]
    pub completion: Option<Shell>,
}

impl Cli {
    #[must_use]
    pub fn init() -> Self {
        Self::parse()
    }
}

pub fn print_completion<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
