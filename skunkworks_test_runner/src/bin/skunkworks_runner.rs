use std::fs;
use std::path::Path;

#[derive(Debug)]
struct FunctionArg {
    arg_type: String,
    arg_value: String,
    arg_out_value: String,
}

#[derive(Debug)]
struct FunctionCall {
    function_name: String,
    arguments: Vec<(String, String, String)>,
    sql_return: String,
}

fn get_function_calls(contents: String) -> Vec<FunctionCall> {
    let mut function_calls = Vec::new();
    println!("{:?}", contents);
    let blocks: Vec<&str> = contents.split("\n\n").collect();
    println!("{:?}", blocks.len());
    for block in blocks {
        let lines: Vec<&str> = block.lines().collect();
            if lines.len() < 4 {
                continue;
            }
        let first_line = lines[0];
        if !first_line.contains("ENTER") || first_line.contains("SQL_SUCCESS") || first_line.contains("SQL_SUCCESS_WITH_INFO") {
            continue;
        }
        let words: Vec<&str> = first_line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        let function_name = words.last().unwrap().to_string();
        let mut arguments = Vec::new();
        for i in 1..lines.len() {
            let line = lines[i].trim();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            // Handle case where the first part is followed by an asterisk
            let mut arg_type = parts[0].to_string();
            let mut value_index = 1;
            if parts.len() > 1 && parts[1] == "*" {
                arg_type.push_str(" *");
                value_index = 2;
            }
            if value_index >= parts.len() {
                continue;
            }
            let mut arg_value = parts[value_index].to_string();
            for part in &parts[2..] {
                if part.starts_with("<") && part.ends_with(">") {
                    arg_value = part.trim_start_matches("<").trim_end_matches(">").to_string();
                    break;
                }
            }
            arguments.push((arg_type, arg_value));
        }
        function_calls.push(FunctionCall {
            function_name,
            arguments,
        });
    }
    function_calls
}
     

fn run_test_runner() {
    println!("Running test runner...");

    let file_path = Path::new("skunkworks_test_runner/testdata/ssis_trace.txt");
    let contents = fs::read_to_string(file_path).expect("error reading file");
    let function_calls = get_function_calls(contents);
    // println!("Function calls: {:?}", function_calls);
    println!("Function calls");
    for function_call in function_calls {
        println!("{:?}", function_call);
    }
}

fn main() {
    run_test_runner()
}
