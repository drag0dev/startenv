use std::env;
use std::process::exit;
use std::fs;
use colored::Colorize;
use std::process::Command;

enum ErrorType{
    Error,
    Success,
}

fn colorful_err(context: &str, message: &str, error_type: ErrorType){
    match error_type{
        ErrorType::Error => {
            println!("{}: {}\n\t{}", "Error".bold().red(), context, message);
            exit(1);
        },
        ErrorType::Success => {
            println!("{}: {}", "Success".bold().green(), message);
        }
    }
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

    // parse vars
    let vars = dotenv_parser::parse_dotenv(&env_content);
    if vars.is_err() {
        colorful_err("parsing vars", &format!("{}", vars.err().unwrap()), ErrorType::Error);
        return;
    }
    let vars = vars.unwrap();

    // attach all env vars
    for (name, value) in vars.iter() {
        process.env(name, value);
        acc += 1;
    }

    // add all other args
    for arg in args.iter().skip(process_index+1){
        process.arg(arg);
    }

    colorful_err("", &format!("starting process with {} vars", acc), ErrorType::Success);
    println!("{}", "--------------------------------------".blue());
    _ = process.status();

    // no need to capture exit code of the process
    //if res.is_err(){
    //    colorfull_err("starting the process", &res.as_ref().err().unwrap().to_string(), ErrorType::Error);
    //}
}
