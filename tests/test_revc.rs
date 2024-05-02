#[cfg(test)]
mod revc_tests {
    use color_eyre::eyre::Result;
    use ros_rs::revc::get_reverse_complement;

    #[test]
    fn test_with_sample_data() -> Result<()> {
        let sample_seq = "AAAACCCGGT";
        let expected_answer = "ACCGGGTTTT";
        let my_answer = get_reverse_complement(sample_seq)?;
        assert_eq!(my_answer, expected_answer);
        Ok(())
    }
}
