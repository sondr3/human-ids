use anyhow::Result;
use clap::CommandFactory;
use human_ids::cli::{self, Cli};

fn main() -> Result<()> {
    let cli = Cli::init();

    if let Some(shell) = cli.completion {
        cli::print_completion(shell, &mut Cli::command());
        return Ok(());
    }

    Ok(())
}
