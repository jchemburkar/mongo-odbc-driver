use std::fs;
use std::path::Path;

#[derive(Debug)]
struct FunctionCall {
    function_name: String,
    arguments: Vec<String>,
}

fn get_function_calls(contents: String) -> Vec<FunctionCall> {

}

fn run_test_runner() {
    println!("Running test runner...");

    let file_path = Path::new("skunkworks_test_runner/testdata/ssis_trace.txt");
    let contents = fs::read_to_string(file_path).expect("error reading file");
    let function_calls = get_function_calls(contents);
    println!("Function calls: {:?}", function_calls);
}

fn main() {
    run_test_runner()
}
