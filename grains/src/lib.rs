pub fn square(square: u32) -> u64 {
    if square < 1 || square > 64 {
        panic!("Square must be between 1 and 64");
    }

    // Compute the number of grains on the given square.
    let mut num_grains = 1;
    for _ in 1..square {
        num_grains *= 2;

        println!("{}", num_grains.to_string());
    }
    num_grains
}
pub fn total() -> u64 {
    // Compute the total number of grains on the chessboard.
    let mut total_grains = 0;
    for all in 1..=64 {
        total_grains +=  square(all);
    }
    total_grains
}