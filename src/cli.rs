use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "ros_rs")]
#[clap(version = "v0.1.0")]
pub struct Cli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Solve the Rosalind DNA challenge at https://rosalind.info/problems/dna/")]
    DNA {
        /// Input file from the DNA Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind RNA challenge at https://rosalind.info/problems/rna/")]
    RNA {
        /// Input file from the RNA Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
}
