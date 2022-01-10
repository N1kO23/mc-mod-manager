pub fn concat_string_array(arr: Vec<String>, separator: &str) -> String {
    let mut joined_args = String::new();
    for i in 0..arr.len() {
        joined_args.push_str(arr[i].as_str());
        if i != arr.len() - 1 {
            joined_args.push_str(separator);
        }
    }
    return joined_args;
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
    return sub;
}
