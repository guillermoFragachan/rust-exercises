pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    if len > digits.len() {
        return vector;
    }

    let bytes = digits.as_bytes();
    for i in 0..(digits.len() - len + 1) {
        let slice = &bytes[i..(i + len)];
        let string = std::str::from_utf8(slice).unwrap();
        vector.push(string.to_string());
    }

    vector
}