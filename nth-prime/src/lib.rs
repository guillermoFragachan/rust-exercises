pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    while count < n {
        if is_prime(i) {
            count += 1;
            if count == n {
                return i;
            }
        }
        i += 1;
    }
    0
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in (2..n).rev() {
        if n % i == 0 {
            return false;
        }
    }
    true
}
