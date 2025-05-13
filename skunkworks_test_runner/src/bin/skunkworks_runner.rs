use std::fs;
use std::path::Path;

fn run_test_runner() {
    println!("Running test runner...");

    let file_path = Path::new("skunkworks_test_runner/testdata/ssis_trace.txt");

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("Successfully read ssis_trace.txt");
            println!("File contents:");
            println!("{}", contents);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            let alt_path = Path::new("testdata/ssis_trace.txt");
            match fs::read_to_string(alt_path) {
                Ok(contents) => {
                    println!("Successfully read ssis_trace.txt using alternate path");
                    println!("File contents:");
                    println!("{}", contents);
                }
                Err(e) => println!("Error reading file with alternate path: {}", e),
            }
        }
    }
}

fn main() {
    run_test_runner()
}
