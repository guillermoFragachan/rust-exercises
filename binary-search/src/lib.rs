pub fn find(array: &[i32], key: i32) -> Option<usize> {
    
    match array.binary_search(&key) {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}
