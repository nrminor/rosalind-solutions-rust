use color_eyre::eyre::Result;
use ros_rs::dna::{count_bases, format_and_print_answer};

#[test]
fn test_with_sample_data() -> Result<()> {
    let sample_seq = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let expected_answer = "20 12 17 21";
    let my_tally = count_bases(sample_seq)?;
    let my_answer = format_and_print_answer(&my_tally)?;
    assert_eq!(my_answer, expected_answer);
    Ok(())
}
