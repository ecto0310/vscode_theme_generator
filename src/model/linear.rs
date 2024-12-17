use num_traits::Num;

pub struct Linear<T: Num + Copy> {
    start: T,
    end: T,
}

impl<T: Num + Copy> Linear<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn v(&self, normalized: T) -> T {
        self.start + (self.end - self.start) * normalized
    }
}
