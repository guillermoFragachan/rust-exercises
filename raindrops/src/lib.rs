pub fn raindrops(n: u32) -> String {

    let mut stringified = String::new();
    if n % 3 == 0 {
        stringified = format!("{}{}",stringified, "Pling".to_string());
    }
    if n % 5 == 0 {
        stringified = format!("{}{}",stringified, "Plang".to_string());
    }
    if n % 7 == 0 {
        stringified = format!("{}{}",stringified, "Plong".to_string());
    }
    
    if stringified.len() == 0{
        stringified  = n.to_string();
    }
    
    return stringified
}


// if divisible by 3 pling
//if not divisible return number 
