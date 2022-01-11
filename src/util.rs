use std::io::{self, stdin, stdout, Write};

pub fn concat_string_array(arr: Vec<String>, separator: &str) -> String {
    let mut joined_args = String::new();
    for i in 0..arr.len() {
        joined_args.push_str(arr[i].as_str());
        if i != arr.len() - 1 {
            joined_args.push_str(separator);
        }
    }
    joined_args
}

pub fn string_to_array(str: &str, separator: &str) -> Vec<String> {
    return str
        .split(separator)
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        .collect();
}

pub fn subvec(main: &Vec<String>, startindex: usize, endindex: usize) -> Vec<String> {
    let mut sub = Vec::new();
    for i in startindex..endindex {
        sub.push(main[i].clone());
    }
    sub
}

pub fn read_line() -> io::Result<String> {
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    match input.trim() {
        "" => Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty input")),
        _ => Ok(input.trim().to_string()),
    }
}
