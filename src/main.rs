use std::env;
use std::process::exit;
use std::fs;
use colored::Colorize;
use std::process::Command;
use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::process;

enum ErrorType{
    Error,
    Warning,
    Success,
}

fn colorful_err(context: &str, message: &str, error_type: ErrorType){
    match error_type{
        ErrorType::Error => {
            println!("{}: {}\n\t{}", "Error".bold().red(), context, message);
            exit(1);
        },
        ErrorType::Warning => {
            println!("{}: {}\n\t{}", "Warning".bold().yellow(), context, message);
        },
        ErrorType::Success => {
            println!("{}: {}", "Success".bold().green(), message);
        }
    }
}

fn check_var(var: &str, index: usize) -> bool{
    if var.len() == 0{
        colorful_err("setting env variables, skipping", "empty line", ErrorType::Warning);
        return false;
    }
    if !var.contains("=") {
        colorful_err(&format!("setting env variables, skipping ({})", index), "missing =", ErrorType::Warning);
        return false;
    }
    if var.matches("=").count() != 1 {
        colorful_err(&format!("setting env variables, skipping ({})", index), "mutliple =", ErrorType::Warning);
        return false;
    }

    let mut iter = var.split("=");
    let name = iter.nth(0).unwrap();
    if name.chars().filter(|&c| c.is_digit(10) || c.is_alphabetic() || c == '_').count() != name.chars().count(){
        colorful_err(&format!("setting env variables, skipping ({})", index), "var name can only consist of letters, numbers, and _", ErrorType::Warning);
        return false;
    }
    if name.chars().filter(|&c| c.is_uppercase()).count() != name.chars().count(){
        colorful_err(&format!("setting env variables ({})", index), "var name is not uppercase", ErrorType::Warning);
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if at least one arg has been provided
    if args.len() < 2{
        colorful_err("parsing arguments", "not enough arguments provided", ErrorType::Error);
    }

    let mut process_index = 0;
    let mut env_content: String = "".to_string();
    // no path is provided, try opening .env in current direcory
    if args[1] != "-f" && args[1] != "--file"{
        let current_dir = env::current_dir();
        if current_dir.is_err(){
            // invalid working directory
            colorful_err("getting current working directory", &current_dir.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        let path = current_dir.unwrap();
        let path = path.to_str().unwrap();
        let contents = fs::read_to_string(format!("{}/.env", path));
        if contents.is_err(){
            colorful_err("reading .env file in the current working directory", &contents.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        env_content = contents.unwrap();
        process_index = 1;
    // else take the path provided with the flag
    }else if args.len() > 3 && (args[1] == "-f" || args[1] == "--file"){
        let contents = fs::read_to_string(&args[2]);
        if contents.is_err(){
            colorful_err(&format!("reading file \"{}\"", args[2]), &contents.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        env_content = contents.unwrap();
        process_index = 3;

    }else{
        colorful_err("parsing arguments", "not enough arguments", ErrorType::Error);
    }

    println!("{}", "--------------------------------------".blue());
    let mut process = Command::new(&args[process_index]);
    let mut acc = 0;

    // attach all env vars
    let mut careful = true;
    for (i, line) in env_content.lines().enumerate(){
        if !check_var(&line, i+1){
            careful = false;
            continue;
        }
        let name = line.split("=").nth(0).unwrap();
        let value = line.split("=").nth(1).unwrap();
        process.env(name, value);
        acc += 1;
    }
    // add all other args
    for arg in args.iter().skip(process_index+1){
        process.arg(arg);
    }

    let mut proceed = true;
    if !careful {
        let mut input_str: String = String::new();
        let input = stdin();
        let mut output = stdout();
        let mut res;
        println!("\n{}", "Not all vars were formatted correctly".red().bold());
        loop {
            print!("{}: ", "Do you want to proceed (y/n)".yellow());
            res = output.flush();
            if res.is_err() {
                colorful_err("flushing input message to stdout", &format!("{}", res.err().unwrap()), ErrorType::Error);
            }
            input_str.clear();
            if input.read_line(&mut input_str).is_err() { continue; }
            input_str = input_str.to_uppercase();
            if input_str.len() != 0 {
                let response = input_str.chars().nth(0).unwrap();
                if response == 'Y' { break; }
                else if response == 'N' {
                    proceed = false;
                    break;
                }
            }
        }
    }

    if proceed {
        colorful_err("", &format!("starting process with {} vars", acc), ErrorType::Success);
        println!("{}", "--------------------------------------".blue());
        _ = process.status();
    }else {
        println!("Not proceeding, exiting...");
    }

    // no need to capture exit code of the process
    //if res.is_err(){
    //    colorfull_err("starting the process", &res.as_ref().err().unwrap().to_string(), ErrorType::Error);
    //}
}

#[cfg(test)]
mod test{
    use crate::check_var;
    #[test]
    fn empty_line(){
        assert_eq!(false, check_var("", 1));
        assert_eq!(true, check_var("VAR=var", 1));
    }
    #[test]
    fn missing_equals(){
        assert_eq!(false, check_var("var", 1));
    }
    #[test]
    fn multiple_equals(){
        assert_eq!(false, check_var("var=var=", 1));
    }
    #[test]
    fn invalid_char(){
        assert_eq!(false, check_var("VAR!=var", 1));
        assert_eq!(true, check_var("VAR1=var", 1));
        assert_eq!(true, check_var("VAR1_=var", 1));
        assert_eq!(false, check_var("VAR1_!=var", 1));
    }
}
