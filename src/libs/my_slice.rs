pub trait MySlice<T> {
    fn slice(&self, start: usize, end: usize) -> &[T];

    fn own(&self);
}

// Implement the trait for arrays of any length
impl<T, const N: usize> MySlice<T> for [T; N] {
    fn slice(&self, start: usize, end: usize) -> &[T] {
        &self[start..end] // Use Rust's built-in slicing
    }
    fn own(&self) {
        print!("owned function of array");
    }
}

// Implement the trait for vectors (Vec<T>)
impl<T> MySlice<T> for Vec<T> {
    fn slice(&self, start: usize, end: usize) -> &[T] {
        &self[start..end] // Use Rust's built-in slicing
    }
    fn own(&self) {
        print!("owned function of vector");
    }
}
