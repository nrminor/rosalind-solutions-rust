# Solutions to Rosalind Problems in Rust

[![Rust CI](https://github.com/nrminor/rust-rosalind-solutions/actions/workflows/rosalind-tests.yml/badge.svg)](https://github.com/nrminor/rust-rosalind-solutions/actions/workflows/rosalind-tests.yml)

Solutions to each problem will be gradually committed here and will be accessible via a command line interface. For example, to solve the DNA problem with the input text file Rosalind gives you, run the following command:

```bash
ros_rs dna -i <ROSALIND FILE>
```

(this assumes you have compiled the source code and have the executable in your `$PATH`.)

Like with [my Python solutions to these problems](https://github.com/nrminor/py-rosalind-solutions), I have a series of goals for these solutions that might make them more complicated than the minimum viable solutions. Those goals include:

1. At minimum, practice Rust!
2. Learn to implement bioinformatic and other common algorithms myself in a memory-efficient manner.
3. Practice command-line interface design with `clap`.
4. Practice some of Rust's testing frameworks, e.g. black box unit-testing, doc-testing, testing with `proptest`, and perhaps even fuzz-testing.
5. Practice docs generation through `cargo`.
6. Practice using the Helix editor, falling back to Zed when needed.
7. Practice functional programming idioms like recursion, structural pattern match, zero side effects, and immutability.
8. Automated formatting, linting, and testing with pre-commit hooks, github actions, and `cargo clippy`.
