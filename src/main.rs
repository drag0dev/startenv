use std::env;
use std::process::exit;
use std::fs;
use colored::Colorize;
use std::process::Command;

enum ErrorType{
    Error,
    Warning,
    Success,
}

fn colorfull_err(context: &str, message: &str, error_type: ErrorType){
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
        colorfull_err("setting env variables, skipping", "empty line", ErrorType::Warning);
        return false;
    }
    if !var.contains("=") {
        colorfull_err(&format!("setting env variables, skipping ({})", index), "missing =", ErrorType::Warning);
        return false;
    }
    if var.matches("=").count() != 1 {
        colorfull_err(&format!("setting env variables, skipping ({})", index), "mutliple =", ErrorType::Warning);
        return false;
    }

    let mut iter = var.split("=");
    let name = iter.nth(0).unwrap();
    if name.chars().filter(|&c| c.is_digit(10) || c.is_alphabetic() || c == '_').count() != name.chars().count(){
        colorfull_err(&format!("setting env variables, skipping ({})", index), "var name can only consist of letter, number, and _", ErrorType::Warning);
        return false;
    }
    if name.chars().filter(|&c| c.is_uppercase()).count() != name.chars().count(){
        colorfull_err(&format!("setting env variables ({})", index), "var name is not uppercase", ErrorType::Warning);
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if at least one arg has been provided
    if args.len() < 2{
        colorfull_err("parsing arguments", "Not enough arguments provided", ErrorType::Error);
    }

    let mut process_index = 0;
    let mut env_content: String = "".to_string();
    // no path is provided, try opening .env in current direcory
    if args[1] != "-f" && args[1] != "--file"{
        let current_dir = env::current_dir();
        if current_dir.is_err(){
            // invalid working directory
            colorfull_err("getting current working directory", &current_dir.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        let path = current_dir.unwrap();
        let path = path.to_str().unwrap();
        let contents = fs::read_to_string(format!("{}/.env", path));
        if contents.is_err(){
            colorfull_err("reading .env file in the current working directory", &contents.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        env_content = contents.unwrap();
        process_index = 1;
    // else take the path provided with the flag
    }else if args.len() > 3 && (args[1] == "-f" || args[1] == "--file"){
        let contents = fs::read_to_string(&args[2]);
        if contents.is_err(){
            colorfull_err(&format!("reading file \"{}\"", args[2]), &contents.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        env_content = contents.unwrap();
        process_index = 3;

    }else{
        colorfull_err("parsing arguments", "not enough arguments", ErrorType::Error);
    }

    println!("{}", "--------------------------------------".blue());
    let mut process = Command::new(&args[process_index]);
    let mut acc = 0;
    // attach all env vars
    for (i, line) in env_content.lines().enumerate(){
        if !check_var(&line, i+1){
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

    colorfull_err("", &format!("starting process with {} vars", acc), ErrorType::Success);
    println!("{}", "--------------------------------------".blue());
    let res = process.status();
    if res.is_err(){
        colorfull_err("starting the process", &res.as_ref().err().unwrap().to_string(), ErrorType::Error);
    }
}
