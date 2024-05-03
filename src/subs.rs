use std::path::Path;

use color_eyre::eyre::Result;
use derive_new::new;
use itertools::Itertools;

use crate::hamm::parse_two_seq_lines;

#[derive(new)]
pub struct KmerWithStart<'a> {
    pub kmer: &'a str,
    pub position: usize,
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn find_kmers<'a>(sequence: &'a str, k: &'a usize) -> Result<Vec<KmerWithStart<'a>>> {
    let sequence_len = sequence.len() - 1;

    let kmers = sequence
        .char_indices()
        .filter_map(|(idx, _)| {
            let end_idx = idx + k;
            (end_idx <= sequence_len).then(|| KmerWithStart::new(&sequence[idx..end_idx], idx))
        })
        .collect();

    Ok(kmers)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn find_motif_matches(kmers: &[KmerWithStart], motif: &str) -> Result<Vec<usize>> {
    let matching_indices = kmers
        .iter()
        .filter(|kmer| kmer.kmer == motif)
        .map(|kmer| kmer.position + 1)
        .collect::<Vec<usize>>();
    Ok(matching_indices)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn solve_subs(input_file: &Path) -> Result<()> {
    let (sequence, motif) = parse_two_seq_lines(input_file)?;
    let k = &motif.len();
    let kmers = find_kmers(&sequence, k)?;
    let matches = find_motif_matches(&kmers, &motif)?;

    let my_answer = matches.iter().join(" ");

    eprintln!("Matches found at the following positions:\n\n{}", my_answer);

    Ok(())
}
