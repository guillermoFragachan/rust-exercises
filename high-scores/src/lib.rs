#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut scores_vec = scores.to_vec();
        // scores_vec.sort_by(|a, b: &u32| b.cmp(a)); // sort scores in descending order
        HighScores { scores: scores_vec }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores.iter().cloned().take(3).collect()
    }
}