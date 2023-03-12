pub trait Vector<T: Ord> {
    fn sort_vec(self) -> Vec<T>;
    fn dedup_vec(self) -> Vec<T>;
    fn sort_and_dedup_vec(self) -> Vec<T>;
}

impl<T: Ord> Vector<T> for Vec<T> {
    fn sort_vec(mut self) -> Vec<T> {
        self.sort();
        self
    }
    fn dedup_vec(mut self) -> Vec<T> {
        self.dedup();
        self
    }
    fn sort_and_dedup_vec(mut self) -> Vec<T> {
        self.sort();
        self.dedup();
        self
    }
}
