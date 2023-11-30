use assert_cli::Assert;

#[test]
fn test_calculate() {
    let assert = Assert::command(&["clap_example1", "--number", "5"])
        .stdout()
        .contains("Number: 5")
        .stdout()
        .contains("Square of number: 25")
        .stdout()
        .contains("Cube of number: 126");

    match assert.execute() {
        Ok(_) => {
            // Test passed
        }
        Err(err) => {
            panic!("Test failed with error: {:?}", err);
        }
    }
}
