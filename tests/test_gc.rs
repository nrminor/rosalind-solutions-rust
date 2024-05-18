use color_eyre::eyre::{eyre, Result};
use noodles::fasta;
use ros_rs::gc::{collect_gc_contents, find_max_gc};
use std::{fs::File, io::BufReader};

#[test]
fn test_with_sample_data() -> Result<()> {
    let expected_answer = "Rosalind_0808\n60.919540";

    let fasta_reader = File::open("tests/data/test_gc.fasta")
        .map(BufReader::new)
        .map(fasta::Reader::new)?;

    let gc_contents = collect_gc_contents(fasta_reader)?;
    let max_gc = find_max_gc(&gc_contents)?;

    match max_gc {
        Some((max_id, gc_content)) => {
            let id = String::from_utf8(max_id.to_owned())?;
            let my_answer = format!("{}\n{:.2$}", id, gc_content, 6);
            println!("The GC solution is:\n\n{}", my_answer);
            assert_eq!(my_answer, expected_answer);
            Ok(())
        }
        None => Err(eyre!(
            "Unable to find a single ID with a maximum GC-content."
        )),
    }
}
