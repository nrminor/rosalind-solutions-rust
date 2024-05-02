use std::path::Path;

use crate::dna::read_rosalind_first_line;
use color_eyre::eyre::Result;

pub fn get_reverse_complement(sequence: &str) -> Result<String> {
    let revc = sequence
        .chars()
        .flat_map(|base| match base {
            'A' => Some('T'),
            'T' => Some('A'),
            'G' => Some('C'),
            'C' => Some('G'),
            'U' => Some('A'),
            _ => None,
        })
        .rev()
        .collect::<String>();

    Ok(revc)
}

pub fn solve_revc(input_file: &Path) -> Result<()> {
    let sequence = read_rosalind_first_line(input_file)?;
    let my_answer = get_reverse_complement(&sequence)?;
    eprintln!("The reverse complement is:\n\n{}", my_answer);
    Ok(())
}
