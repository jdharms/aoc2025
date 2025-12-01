use std::error::Error;
use std::fs::read_to_string;

pub fn get_file_name(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() < 2 {
        Err("Not enough arguments".into())
    }
    else {
        Ok(args[1].clone())
    }
}

pub fn read_lines(path: String) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
