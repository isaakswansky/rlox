use std::env;
use std::error::Error;
use std::process;

mod error;
mod scanner;
mod token;

mod rlox {
    use super::error::*;
    use super::scanner::*;

    pub fn run(code: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(code);
        match scanner.scan() {
            Err(ErrorType::IOError(line, msg)) => Err(Box::new(Error(line, msg))),
            Err(ErrorType::RuntimeError(line, msg)) => Err(Box::new(Error(line, msg))),
            Err(ErrorType::ScanError(line, msg)) => Err(Box::new(Error(line, msg))),
            Ok(_) => {
                for token in scanner.tokens.iter() {
                    println!("{:?}", token);
                }
                Ok(())
            }
        }
    }

    pub fn run_file(file_name: &String) -> Result<(), Box<dyn std::error::Error>> {
        let code = std::fs::read_to_string(file_name)?;
        run(code)
    }

    pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
        loop {
            print!("> ");
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            run(buffer)?;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result: Result<(), Box<dyn Error>>;

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        result = rlox::run_file(&args[1]);
    } else {
        result = rlox::run_prompt();
    }

    match result {
        Ok(_) => println!("rlox exited successfully."),
        Err(error) => {
            println!("rlox exited with an error: {}", error);
            process::exit(1);
        }
    }
    process::exit(0);
}
