/*!

This implementation was heavily influenced by (and indeed copies much of)
the implementation at https://github.com/dweb0/protein-translate, which uses array indexing
to achieve considerably better performance than a match (my first idea). It's also a bit
faster than using various hashmaps.

My implementation uses the same system, but switches out the original implementation's use
of pre-allocated strings, for loops, and mutability for iterators, structural pattern matching,
and immutability, making it more reminiscent of the functional programming style. It also makes
more liberal use of algebraic data types to handle errors, including indexing errors.

Another potential idea is to make the ASCII_TO_INDEX matrix sparse given that 4's are always
ignored, but I have not implemented that yet.

*/

use std::path::Path;

use crate::dna::read_rosalind_first_line;
use color_eyre::eyre::{ensure, eyre, Result};

/// https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi
/// U is equivalent to T here
///
/// The 1st index picks the 4x4 block
/// The 2nd index picks the row
/// the 3rd index picks the column
static AA_TABLE_CANONICAL: [[[char; 4]; 4]; 4] = [
    [
        ['K', 'N', 'K', 'N'], // AAA, AAC, AAG, AAU/AAT
        ['T', 'T', 'T', 'T'], // ACA, ACC, ACG, ACU/ACT
        ['R', 'S', 'R', 'S'], // AGA, AGC, AGG, AGU/AGT
        ['I', 'I', 'M', 'I'], // AUA/ATA, AUC/ATC, AUG/ATG, AUU/ATT
    ],
    [
        ['Q', 'H', 'Q', 'H'], // CAA, CAC, CAG, CAU/CAT
        ['P', 'P', 'P', 'P'], // CCA, CCC, CCG, CCU/CCT
        ['R', 'R', 'R', 'R'], // CGA, CGC, CGG, CGU/CGT
        ['L', 'L', 'L', 'L'], // CUA/CTA, CUC/CTC, CUG/CTG, CUU/CTT
    ],
    [
        ['E', 'D', 'E', 'D'], // GAA, GAC, GAG, GAU/GAT
        ['A', 'A', 'A', 'A'], // GCA, GCC, GCG, GCU/GCT
        ['G', 'G', 'G', 'G'], // GGA, GGC, GGG, GGU/GGT
        ['V', 'V', 'V', 'V'], // GUA/GTA, GUC/GTC, GUG/GTG, GUU/GTT
    ],
    [
        ['*', 'Y', '*', 'Y'], // UAA/TAA, UAC/TAC, UAG/TAG, UAU/TAT
        ['S', 'S', 'S', 'S'], // UCA/TCA, UCC/TCC, UCG/TCG, UCU/TCT
        ['*', 'C', 'W', 'C'], // UGA/TGA, UGC/TGC, UGG/TGG, UGU/TGT
        ['L', 'F', 'L', 'F'], // UUA/TTA, UUC/TTC, UUG/TTG, UUU/TTT
    ],
];

const CODON_SIZE: usize = 3;

/// Maps an ASCII character to array index
///
/// A = 65, a = 97  => 0
/// C = 67, c = 99  => 1
/// G = 71, g = 103 => 2
/// T = 84, t = 116 => 3
/// U = 85, u = 117 => 3
static ASCII_TO_INDEX: [usize; 128] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 0-15
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 16-31
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 32-47
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 48-63
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, // 64-79    (65 = A, 67 = C, 71 = G)
    4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 80-95    (84 = T, 85 = U)
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, // 96-111   (97 = a, 99 = c, 103 = g)
    4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 112-127  (116 = t, 117 = u)
];

/// .
///
/// # Errors
///
/// This function will return an error if .
fn ascii_to_index<const N: usize>(codon: &[u8]) -> Result<[usize; N]> {
    // make sure the codon is the proper length
    ensure!(
        codon.len() == CODON_SIZE,
        "The codon, {:?}, was not correctly split.",
        codon
    );

    // make sure all bytes in that codon are valid
    ensure!(
        codon.iter().all(|&byte| byte <= 128),
        "Invalid byte found in the provided codon {:?}",
        codon
    );

    // attempt to retrieve the indices
    let indices_attempt = codon
        .iter()
        .map(|&base| ASCII_TO_INDEX[base as usize])
        .collect::<Vec<_>>()
        .try_into();

    // return the indices if the attempt was sucessful. Otherwise, return a useful error
    match indices_attempt {
        Ok(indices) => Ok(indices),
        Err(_) => Err(eyre!(
            "Error collecting the three indices for codon {:?}",
            codon
        )),
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
///
/// # Examples
///
/// ```
/// // use ros_rs::prot::translate;
///
/// // assert_eq!(translate(seq), );
/// ```
pub fn translate(seq: &[u8]) -> Result<String> {
    let peptide: String = seq
        .chunks_exact(CODON_SIZE)
        .map(|codon| {
            // check for non-ASCII-encoded characters
            if !codon.iter().all(|&base| base.is_ascii()) {
                eprintln!(
                    "Some of the bases in {:?} are not valid ASCII. Calling 'X' for ambiguous amino acid.",
                    String::from_utf8_lossy(codon)
                );
                return Ok('X');
            }

            // if encodings are valid, pull out the index for each base
            let [base1, base2, base3] = ascii_to_index::<CODON_SIZE>(codon)?;

            // early return if any of those indices are 4
            if base1 == 4 || base2 == 4 || base3 == 4 {
                return Ok('X');
            }
            let amino_acid = AA_TABLE_CANONICAL[base1][base2][base3];

            // otherwise, return the amino acid
            Ok(amino_acid)
        })
        .filter_map(|codon: Result<char>| codon.ok())
        .collect();

    Ok(peptide)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
///
/// # Examples
///
/// ```
/// // use ros_rs::prot::solve_prot;
///
/// // assert_eq!(solve_prot(input_file), );
/// ```
pub fn solve_prot(input_file: &Path) -> Result<()> {
    let sequence = read_rosalind_first_line(input_file)?;
    let translation = translate(sequence.as_bytes())?;

    let without_stops = translation.replace('*', "");

    eprintln!(
        "The translation of the provided sequence is:\n\n{:?}",
        without_stops
    );
    Ok(())
}
