use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use color_eyre::eyre::{eyre, Result};
use derive_new::new;

#[derive(new)]
pub struct NucleotideCounts {
    #[new(value = "0")]
    pub a: usize,
    #[new(value = "0")]
    pub t: usize,
    #[new(value = "0")]
    pub g: usize,
    #[new(value = "0")]
    pub c: usize,
    #[new(value = "0")]
    pub u: usize,
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn read_rosalind_first_line(input_file: &Path) -> Result<String> {
    let file = File::open(input_file)?;
    let mut file_lines = BufReader::new(file).lines();

    if let Some(Ok(first_line)) = file_lines.next() {
        Ok(first_line)
    } else {
        Err(eyre!("First line of file could not be parsed."))
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn count_bases(sequence: &str) -> Result<NucleotideCounts> {
    let bases = sequence.chars().flat_map(char::to_uppercase);
    let mut counts = NucleotideCounts::new();
    bases.into_iter().for_each(|base| match base {
        'A' => counts.a += 1,
        'T' => counts.t += 1,
        'C' => counts.c += 1,
        'G' => counts.g += 1,
        'U' => counts.u += 1,
        _ => (),
    });

    Ok(counts)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn format_and_print_answer(counts: &NucleotideCounts) -> Result<String> {
    let my_answer = format!("{} {} {} {}", counts.a, counts.c, counts.g, counts.t);
    println!("Our final tally of bases is:");
    println!("----------------------------");
    println!("Adenines: {}", counts.a);
    println!("Thymines: {}", counts.t);
    println!("Guanines: {}", counts.g);
    println!("Cytosines: {}", counts.c);

    println!("\n\nAnd to format like the problem asked:");
    println!("{}", my_answer);

    Ok(my_answer)
}

pub fn solve_dna(input_file: &Path) -> Result<()> {
    let dna_line = read_rosalind_first_line(input_file)?;
    let my_tally = count_bases(&dna_line)?;
    let _ = format_and_print_answer(&my_tally)?;

    Ok(())
}
