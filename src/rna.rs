use std::path::Path;

use crate::dna::read_rosalind_first_line;
use color_eyre::eyre::Result;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn transcribe(dna_seq: &str) -> Result<String> {
    Ok(dna_seq.replace('T', "U"))
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn solve_rna(input_file: &Path) -> Result<()> {
    let dna_line = read_rosalind_first_line(input_file)?;
    let my_answer = transcribe(&dna_line)?;
    eprintln!("My answer is:\n\n{}", my_answer);
    Ok(())
}
