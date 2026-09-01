pub struct ClosedRange<T: Ord> {
    pub min: T,
    pub max: T,
}

impl<T: Ord> ClosedRange<T> {
    pub fn new(min: T, max: T) -> Self {
        if min >= max {
            return Self { min: max, max: min };
        }

        Self { min: min, max: max }
    }

    pub fn is_in(&self, query: T) -> bool {
        query >= self.min && query <= self.max
    }
}
