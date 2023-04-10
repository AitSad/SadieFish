use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

// This test assumes that the SadieFish binary has been built and is located in "./target/debug/sadiefish"

#[test]
fn test_sadiefish() {
    // Create a temporary file with some input data
    let mut input_file = File::create("input.txt").unwrap();
    input_file.write_all(b"Hello, SadieFish!").unwrap();

    // Run the SadieFish command with the input file as input
    let mut child = Command::new("./target/debug/sadiefish")
        .stdin(Stdio::from(File::open("input.txt").unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Read the output of the command
    let mut output = String::new();
    child.stdout.take().unwrap().read_to_string(&mut output).unwrap();

    // Assert that the output is what we expect
    assert_eq!(output.trim(), "Hello, SadieFish!");
}
