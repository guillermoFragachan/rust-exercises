


pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let mut number: usize = num as usize;
    let mut digits: Vec<usize> = Vec::new();
    println!("{} here on", number);

    while number > 0 {
        let digit = number % 10;
        number /= 10;

        println!("{} here on", number);
        digits.push(digit)
    }

    let mut total : usize = 0;

    let length = digits.len();
    for digit in digits {
        let power_of = digit.pow(length.try_into().unwrap());
        total += power_of ;
    }


    total == num.try_into().unwrap()
}
