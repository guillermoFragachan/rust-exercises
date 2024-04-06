pub fn brackets_are_balanced(string: &str) -> bool {


    let mut stringified = string.to_string();

    let stringified_len = stringified.len();

    if stringified_len == 0{
        return true 
    }

    for i in 0..stringified_len {
        let last_char = stringified.remove(stringified_len - 1);

        let first_char = stringified.remove(0);
    
      
        if checker(first_char, last_char) {
            return true
        }
    }

    return false;
}


fn checker ( first: char, last : char )  -> bool {
    let brackets = "[]".to_string();
    let curly = "()".to_string();
    let parent = "{}".to_string();

    if brackets.contains(first) && brackets.contains(last) {
        return true;
    }
    if curly.contains(first) && curly.contains(last) {
        return true;
    }
    if parent.contains(first) && parent.contains(last) {
        return true;
    }
    return false

}