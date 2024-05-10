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
    #[clap(about = "Solve the Rosalind REVC challenge at https://rosalind.info/problems/revc/")]
    REVC {
        /// Input file from the REVC Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind HAMM challenge at https://rosalind.info/problems/hamm/")]
    HAMM {
        /// Input file from the HAMM Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind SUBS challenge at https://rosalind.info/problems/subs/")]
    SUBS {
        /// Input file from the SUBS Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind GC challenge at https://rosalind.info/problems/gc/")]
    GC {
        /// Input file from the GC Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind PROT challenge at https://rosalind.info/problems/prot/")]
    PROT {
        /// Input file from the PROT Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind FIB challenge at https://rosalind.info/problems/fib/")]
    FIB {
        /// Input file from the FIB Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
    #[clap(about = "Solve the Rosalind IPRB challenge at https://rosalind.info/problems/iprb/")]
    IPRB {
        /// Input file from the IPRB Rosalind challenge
        #[arg(short, long, required = true)]
        input_file: PathBuf,
    },
}
