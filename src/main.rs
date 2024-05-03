use clap::Parser;
use color_eyre::eyre::Result;
use ros_rs::{
    cli::{Cli, Commands},
    dna::solve_dna,
    hamm::solve_hamm,
    revc::solve_revc,
    rna::solve_rna,
    subs::solve_subs,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::DNA { input_file }) => solve_dna(input_file)?,
        Some(Commands::RNA { input_file }) => solve_rna(input_file)?,
        Some(Commands::REVC { input_file }) => solve_revc(input_file)?,
        Some(Commands::HAMM { input_file }) => solve_hamm(input_file)?,
        Some(Commands::SUBS { input_file }) => solve_subs(input_file)?,
        None => {
            eprintln!("No solutions requested.",);
            std::process::exit(0);
        }
    }

    Ok(())
}
