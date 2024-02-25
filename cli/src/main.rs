use clap::{CommandFactory, Parser};
use human_ids::{generate, Options};

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    if let Some(shell) = cli.completion {
        cli::print_completion(shell, &mut Cli::command());
        return;
    }

    let options = Options::builder()
        .separator(&cli.separator)
        .capitalize(cli.capitalize)
        .add_adverb(cli.adverb)
        .adjective_count(cli.num_adjectives)
        .build();

    println!("{}", generate(Some(options)));
}
