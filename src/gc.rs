use color_eyre::eyre::{eyre, Result};
use noodles::fasta;
use std::{cmp::Ordering, fs::File, io::BufReader, path::Path};

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn collect_gc_contents(
    mut fasta_reader: fasta::Reader<BufReader<File>>,
) -> Result<Vec<(Vec<u8>, f32)>> {
    let gc_contents = fasta_reader
        .records()
        .flatten()
        .map(|record| {
            let id = record.name().to_owned();
            let sequence = record.sequence().as_ref();
            let c_content = sequence.iter().filter(|base| *base == &b'C').count() as f32;
            let g_content = sequence.iter().filter(|base| *base == &b'G').count() as f32;
            let gc_content = ((c_content + g_content) / sequence.len() as f32) * 100.0;
            (id, gc_content)
        })
        .collect::<Vec<(Vec<u8>, f32)>>();

    Ok(gc_contents)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn find_max_gc(gc_contents: &[(Vec<u8>, f32)]) -> Result<Option<&(Vec<u8>, f32)>> {
    let max_gc = gc_contents
        .iter()
        .max_by(|(_, gc_a), (_, gc_b)| gc_a.partial_cmp(gc_b).unwrap_or(Ordering::Equal));

    Ok(max_gc)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn report_answer(max_gc: Option<&(Vec<u8>, f32)>) -> Result<()> {
    match max_gc {
        Some((max_id, gc_content)) => {
            let id = String::from_utf8(max_id.to_owned())?;
            let my_answer = format!("{}\n{:.2$}", id, gc_content, 6);
            println!("The GC solution is:\n\n{}", my_answer);
            Ok(())
        }
        None => Err(eyre!(
            "Unable to find a single ID with a maximum GC-content."
        )),
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn solve_gc(input_file: &Path) -> Result<()> {
    let fasta_reader = File::open(input_file)
        .map(BufReader::new)
        .map(fasta::Reader::new)?;
    let gc_contents = collect_gc_contents(fasta_reader)?;
    let max_gc = find_max_gc(&gc_contents)?;
    report_answer(max_gc)?;

    Ok(())
}
