use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use color_eyre::eyre::{eyre, Result};

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn parse_two_seq_lines(input_file: &Path) -> Result<(String, String)> {
    let file = File::open(input_file)?;
    let file_lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    // check that two sequences were found
    assert_eq!(
        file_lines.len(),
        2,
        "{} sequence(s) were found in the input file, whereas 2 were expected.",
        file_lines.len()
    );

    let seq1 = match file_lines.first() {
        Some(sequence) => sequence.clone(),
        None => return Err(eyre!("Sequence in first line line could not be found.")),
    };
    let seq2 = match file_lines.get(1) {
        Some(sequence) => sequence.clone(),
        None => return Err(eyre!("Sequence in first line line could not be found.")),
    };

    // catch if the sequences are the same lengths
    assert_eq!(&seq1.len(), &seq2.len(),
            "The length of the first sequence, {}, does not match the length of the second sequence, {}",
            &seq1.len(), &seq2.len());

    Ok((seq1, seq2))
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn compute_distance(seq1: &str, seq2: &str) -> Result<usize> {
    assert_eq!(&seq1.len(), &seq2.len(),
        "The length of the first sequence, {}, does not match the length of the second sequence, {}",
        &seq1.len(), &seq2.len());

    let distance = seq1
        .chars()
        .zip(seq2.chars())
        .filter(|(base1, base2)| base1 != base2)
        .count();

    Ok(distance)
}

pub fn solve_hamm(input_file: &Path) -> Result<()> {
    let (seq1, seq2) = parse_two_seq_lines(input_file)?;
    let my_answer = compute_distance(&seq1, &seq2)?;
    eprintln!(
        "The Hamming distance between the provided sequences is:\n\n{}",
        my_answer
    );
    Ok(())
}
