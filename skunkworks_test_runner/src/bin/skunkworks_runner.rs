use std::{collections::BTreeMap, ptr::null_mut};
use std::fs;
use std::path::Path;
use definitions::{
    Handle,
    SqlReturn,
    SQLAllocHandle
};

#[derive(Debug, Clone)]
struct FunctionArg {
    arg_type: String,
    arg_value: String,
    arg_out_value: String,
}

#[derive(Debug, Clone)]
struct FunctionCall {
    function_name: String,
    arguments: Vec<(String, String, String)>,
    sql_return: SqlReturn,
}

fn get_function_calls(contents: String) -> Vec<FunctionCall> {
    let mut function_calls: Vec<FunctionCall> = Vec::new();
    // println!("{:?}", contents);
    let blocks: Vec<&str> = contents.split("\n\n").collect();
    // println!("{:?}", blocks.len());
    for block in blocks {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 1 {
            continue;
        }
        let first_line = lines[0];
        let words: Vec<&str> = first_line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        if first_line.contains("EXIT") && (first_line.contains("SQL_SUCCESS") || first_line.contains("SQL_SUCCESS_WITH_INFO") || first_line.contains("SQL_ERROR")) {
            if !function_calls.is_empty() {
                let last_idx = function_calls.len() - 1;
                
                if let Some(return_code_idx) = first_line.find("(SQL_") {
                    let return_code_end = first_line[return_code_idx..].find(")");
                    if let Some(end_idx) = return_code_end {
                        let return_code_str = &first_line[return_code_idx + 1..return_code_idx + end_idx];
                        let sql_return = match return_code_str {
                            "SQL_SUCCESS" => SqlReturn::SUCCESS,
                            "SQL_SUCCESS_WITH_INFO" => SqlReturn::SUCCESS_WITH_INFO,
                            "SQL_ERROR" => SqlReturn::ERROR,
                            "SQL_INVALID_HANDLE" => SqlReturn::INVALID_HANDLE,
                            "SQL_STILL_EXECUTING" => SqlReturn::STILL_EXECUTING,
                            "SQL_NEED_DATA" => SqlReturn::NEED_DATA,
                            "SQL_NO_DATA" => SqlReturn::NO_DATA,
                            _ => SqlReturn::SUCCESS, // Default to SUCCESS if unknown
                        };
                        function_calls[last_idx].sql_return = sql_return;
                    }
                }
                
                let mut exit_args = Vec::new();
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
                    let line_str = lines[i].trim();
                    if let Some(paren_start) = line_str.rfind('(') {
                        if let Some(paren_end) = line_str.rfind(')') {
                            if paren_start < paren_end {
                                let paren_value = line_str[paren_start+1..paren_end].trim();
                                if !paren_value.is_empty() {
                                    arg_value = paren_value.to_string();
                                }
                            }
                        }
                    }
                    else {
                        for part in &parts[value_index+1..] {
                            if part.starts_with("<") && part.ends_with(">") {
                                arg_value = part.trim_start_matches("<").trim_end_matches(">").to_string();
                                break;
                            }
                        }
                    }
                    
                    exit_args.push((arg_type, arg_value));
                }
                
                for (i, (arg_type, arg_value)) in exit_args.iter().enumerate() {
                    if i < function_calls[last_idx].arguments.len() {
                        let (existing_type, existing_value, _) = &function_calls[last_idx].arguments[i];
                        if existing_type == arg_type {
                            function_calls[last_idx].arguments[i] = 
                                (existing_type.clone(), existing_value.clone(), arg_value.clone());
                        }
                    }
                }
            }
            continue;
        }
        if !first_line.contains("ENTER") {
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
            let line_str = lines[i].trim();
            if let Some(paren_start) = line_str.rfind('(') {
                if let Some(paren_end) = line_str.rfind(')') {
                    if paren_start < paren_end {
                        let paren_value = line_str[paren_start+1..paren_end].trim();
                        if !paren_value.is_empty() {
                            arg_value = paren_value.to_string();
                        }
                    }
                }
            }
            else {
                for part in &parts[value_index+1..] {
                    if part.starts_with("<") && part.ends_with(">") {
                        arg_value = part.trim_start_matches("<").trim_end_matches(">").to_string();
                        break;
                    }
                }
            }
            arguments.push((arg_type, arg_value, String::new()));
        }
        function_calls.push(FunctionCall {
            function_name,
            arguments,
            sql_return: SqlReturn::SUCCESS,
        });
    }
    function_calls
}
     

unsafe fn test_function_calls(function_calls: Vec<FunctionCall>) {
    unsafe {
        let mut log_address_to_handle: BTreeMap<String, Handle> = BTreeMap::new();
        
        for function_call in function_calls.iter() {
            if function_call.function_name == "SQLAllocHandle" {
                let handle_type = match function_call.arguments[0].1.as_str() {
                    "SQL_HANDLE_ENV" => 1,
                    "SQL_HANDLE_DBC" => 2,
                    "SQL_HANDLE_STMT" => 3,
                    "SQL_HANDLE_DESC" => 4,
                    _ => unreachable!()
                };
                let input_handle = match function_call.arguments[1].1.as_str() {
                    "0x0000000000000000" => null_mut(),
                    addr => {
                        println!("addr: {}", addr);
                        println!("log_address_to_handle: {:?}", log_address_to_handle);
                        *log_address_to_handle.get(addr).unwrap()
                    }
                };
                println!("{:?}", input_handle);
                log_address_to_handle.insert(function_call.arguments[2].2.clone(), null_mut() as Handle);
                let alloc_ret = SQLAllocHandle(handle_type, input_handle as *mut _, log_address_to_handle.get_mut(&function_call.arguments[2].2).unwrap());
                println!("SQLAllocHandle return code: {:?}", alloc_ret);

            }
        }
    }
}

fn run_test_runner() {
    println!("Running test runner...");

    let file_path = Path::new("skunkworks_test_runner/testdata/ssis_trace.txt");
    let contents = fs::read_to_string(file_path).expect("error reading file");
    let function_calls = get_function_calls(contents);
    
    unsafe {
        test_function_calls(function_calls);
    }
}

fn main() {
    run_test_runner()
}
