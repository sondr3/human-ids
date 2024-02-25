use std::{
    fs::File,
    io::Error,
    path::{Path, PathBuf},
};

use clap::{CommandFactory, ValueEnum};
use clap_complete::generate_to;
use clap_mangen::Man;

include!("src/cli.rs");

fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn build_shell_completion(outdir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "human-ids", outdir)?;
    }

    Ok(())
}

fn build_manpages(path: &Path) -> Result<(), Error> {
    let app = Cli::command();

    let file = Path::new(&path).join("human-ids.1");
    let mut file = File::create(file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = get_cargo_target_dir()?;
    let out_dir = out_dir
        .ancestors()
        .nth(1)
        .unwrap()
        .to_owned()
        .join("assets");
    std::fs::create_dir_all(&out_dir)?;

    build_shell_completion(&out_dir)?;
    build_manpages(&out_dir)?;

    Ok(())
}
