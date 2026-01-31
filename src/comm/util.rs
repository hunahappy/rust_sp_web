pub fn format_with_commas<T: Into<i128>>(num: T) -> String {
    let s = num.into().to_string();
    let mut result = String::new();

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for (i, c) in chars.iter().enumerate() {
        result.push(*c);
        let pos = len - i - 1;
        if pos % 3 == 0 && i != len - 1 {
            result.push(',');
        }
    }

    result
}
