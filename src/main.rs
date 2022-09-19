use std::env;
use std::process::exit;
use std::fs;
use colored::Colorize;

enum ErrorType{
    Error,
    Warning,
}

fn colorfull_err(context: &str, message: &str, error_type: ErrorType){
    match error_type{
        ErrorType::Error => {
            println!("{}: {}\n\t{}", "Error".bold().red(), context, message);
            exit(1);
        },
        ErrorType::Warning => {
            println!("{}: {}\n\t{}", "Warning".bold().yellow(), context, message);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if at least one arg has been provided
    if args.len() < 2{
        colorfull_err("parsing arguments", "Not enough arguments provided", ErrorType::Error);
    }

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
    // else take the path provided with the flag
    }else if args.len() > 3 && (args[1] == "-f" || args[1] == "--file"){
        let contents = fs::read_to_string(&args[2]);
        if contents.is_err(){
            colorfull_err(&format!("reading file \"{}\"", args[2]), &contents.as_ref().err().unwrap().to_string(), ErrorType::Error);
        }
        env_content = contents.unwrap();

    }else{
        colorfull_err("parsing arguments", "not enough arguments", ErrorType::Error);
    }
}
