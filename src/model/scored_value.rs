pub trait Scoreable {
    fn calc_score(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct ScoredValue<T: Scoreable> {
    value: T,
    score: f32,
}

impl<T: Scoreable> ScoredValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            score: value.calc_score(),
            value,
        }
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn take(self) -> T {
        self.value
    }
}
