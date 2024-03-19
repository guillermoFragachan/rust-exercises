pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let skip_zero: u32 = 0;
    for n in 0..limit {
        for &factor in factors {
            if &factor == &skip_zero {
                break
            }
            if n % factor == 0 {
                sum += n;
                break;
            }
        }
    }
    sum
}

// let factors = &[3, 5];
// let limit = 1;
// let output = sum_of_multiples::sum_of_multiples(limit, factors);
// let expected = 0;
// assert_eq!(output, expected);