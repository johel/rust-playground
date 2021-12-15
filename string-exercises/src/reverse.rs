pub fn reverse1(input: &str) -> String {
    let list:Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    for i in 0..list.len() {
        result.push(list[list.len() - i -1]);
    }

    return String::from_iter(result);

}
