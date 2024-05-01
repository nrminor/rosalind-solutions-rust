use clap::Parser;
use color_eyre::eyre::Result;
use ros_rs::{
    cli::{Cli, Commands},
    dna::solve_dna,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::DNA { input_file }) => solve_dna(input_file)?,
        Some(Commands::RNA { input_file }) => println!(
            "The solution to RNA is not ready yet! `{}` will not be processed",
            input_file.display()
        ),
        None => {
            eprintln!("No solutions requested.",);
            std::process::exit(0);
        }
    }

    Ok(())
}
