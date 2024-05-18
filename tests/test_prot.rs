use color_eyre::eyre::Result;
use ros_rs::prot::translate;

#[test]
fn test_with_sample_data() -> Result<()> {
    let sample_data = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
    let expected_answer = "MAMAPRTEINSTRING";

    let my_answer = translate(sample_data.as_bytes())?.replace('*', "");

    assert_eq!(
        &my_answer, expected_answer,
        "\nMismatch between my answer and the expected answer:\n{}\n{}.",
        my_answer, expected_answer
    );

    Ok(())
}
