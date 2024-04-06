// Letter                           Value
// A, E, I, O, U, L, N, R, S, T       1
// D, G                               2
// B, C, M, P                         3
// F, H, V, W, Y                      4
// K                                  5
// J, X                               8
// Q, Z                               10

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score: u64 = 0;
    for c in word.chars() {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => score += 1,
            'D' | 'G' => score += 2,
            'B' | 'C' | 'M' | 'P' => score += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => score += 4,
            'K' => score += 5,
            'J' | 'X' => score += 8,
            'Q' | 'Z' => score += 10,
            _ => (),
        }
    }

    return score
}
