#[cfg(test)]
mod subs_tests {
    use color_eyre::eyre::Result;
    use itertools::Itertools;
    use ros_rs::subs::{find_kmers, find_motif_matches};

    #[test]
    fn test_with_sample_data() -> Result<()> {
        let sequence = "GATATATGCATATACTT";
        let motif = "ATAT";
        let expected_answer = "2 4 10";
        let k = &motif.len();
        let kmers = find_kmers(sequence, k)?;
        let match_positions = find_motif_matches(&kmers, motif)?;
        let my_answer = match_positions.iter().join(" ");

        assert_eq!(my_answer, expected_answer);
        Ok(())
    }
}
