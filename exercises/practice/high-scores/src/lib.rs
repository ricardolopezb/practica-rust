#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores{scores: Vec::from(scores)}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        return self.scores.last().copied();
    
}

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted_scores = self.scores.to_vec();
        sorted_scores.sort();
        sorted_scores.reverse();
        sorted_scores.get(0).map(|x| *x)
        
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {
            return Vec::new();
        }
        if self.scores.len() < 3 {
            let mut sorted = self.scores.to_vec();
            sorted.sort();
            sorted.reverse();
            sorted
        } else {
            let mut sorted = self.scores.to_vec();
            sorted.sort();
            sorted.reverse();
            sorted.truncate(3);
            sorted
        }
    }
}
