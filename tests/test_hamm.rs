#[cfg(test)]
mod hamm_tests {
    use color_eyre::eyre::Result;

    #[test]
    fn test_with_sample_data() -> Result<()> {
        let seq1 = "GAGCCTACTAACGGGAT";
        let seq2 = "CATCGTAATGACGGCCT";
        let expected_answer = 7;

        assert_eq!(&seq1.len(), &seq2.len(),
            "The length of the first sequence, {}, does not match the length of the second sequence, {}",
            &seq1.len(), &seq2.len());

        let my_answer = seq1
            .chars()
            .zip(seq2.chars())
            .filter(|(base1, base2)| base1 != base2)
            .count();

        assert_eq!(my_answer, expected_answer);
        Ok(())
    }
}
