#[cfg(test)]
mod rna_tests {
    use color_eyre::eyre::Result;

    use ros_rs::rna::transcribe;

    #[test]
    fn test_with_sample_data() -> Result<()> {
        let sample_seq = "GATGGAACTTGACTACGTAAATT";
        let expected_answer = "GAUGGAACUUGACUACGUAAAUU";
        let my_answer = transcribe(sample_seq)?;
        assert_eq!(my_answer, expected_answer);
        Ok(())
    }
}
