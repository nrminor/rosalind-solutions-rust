use clap::Parser;
use color_eyre::eyre::Result;
use ros_rs::{
    cli::{Cli, Commands},
    dna::solve_dna,
    gc::solve_gc,
    hamm::solve_hamm,
    prot::solve_prot,
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
        Some(Commands::GC { input_file }) => solve_gc(input_file)?,
        Some(Commands::PROT { input_file }) => solve_prot(input_file)?,
        Some(Commands::FIB { input_file }) => {
            eprintln!("Solution to FIB involving {:?} coming soon!", input_file);
        }
        Some(Commands::IPRB { input_file }) => {
            eprintln!("Solution to IPRB involving {:?} coming soon!", input_file);
        }
        None => {
            eprintln!("No solutions requested.",);
            std::process::exit(0);
        }
    }

    Ok(())
}
