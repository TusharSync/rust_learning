#[allow(dead_code)]
fn main() {
    enum SortType {
        ASC,
        DESC
    }

    struct FixedArray<const N: usize> {
        data: [i32; N],
        length: usize,
    }

    impl<const N: usize> FixedArray<N> {
        fn new() -> Self {
            FixedArray {
                data: [0; N], // Initialize with zeros
                length: 0,
            }
        }
    
        fn push(&mut self, value: i32) {
            if self.length < N {
                self.data[self.length] = value;
                self.length += 1;
            } else {
                panic!("Array is full")
            }
        }
    
        fn insert(&mut self, index: usize, value: i32) {
            if self.length == N {
                panic!("Array is full");
            }
            if index > self.length {
                panic!("Index out of bounds");
            }
    
            // Shift elements to the right to make space for the new element
            for i in (index..self.length).rev() {
                self.data[i + 1] = self.data[i];
            }
            self.data[index] = value;
            self.length += 1;
        }
    
        fn get(&self) -> &[i32] {
            &self.data[..self.length]
        }
    
        fn pop(&mut self) {
            if self.length > 0 {
                self.length -= 1; // Decrease length to "remove" the last element
            } else {
                panic!("Array is empty");
            }
        }
    
        fn empty(&mut self) {
            if self.length > 0 {
                self.length = 0; // Decrease length to "remove" the last element
            } else {
                panic!("Array is empty");
            }
        }
    
        fn filter<F>(&mut self, callback: F)
        where
            F: Fn(i32) -> bool,
        {
            if self.length == 0 {
                panic!("Array is empty");
            }
    
            let mut new_length: usize = 0;
    
            for i in 0..self.length {
                if callback(self.data[i]) {
                    self.data[new_length] = self.data[i];
                    new_length += 1;
                }
            }
    
            self.length = new_length; // Update length to the new count of elements
        }
    
        fn sort(&mut self, sort_type: SortType) {
            let slice: &mut [i32] = &mut self.data[..self.length]; // Get a mutable slice of the used portion of the array
            print!("{slice:?}");
            match sort_type {
                SortType::ASC => slice.sort_unstable(), // Sort in ascending order
                SortType::DESC => slice.sort_unstable_by(|a, b| b.cmp(a)), // Sort in descending order
            }
        }
    
    }

    

    fn is_even(x: i32) -> bool {
        x % 2 == 0
    }

    let mut fixed_array: FixedArray<7> = FixedArray::<7>::new(); // Specify size here
    fixed_array.push(1);
    fixed_array.push(2);
    fixed_array.push(3);
    fixed_array.push(4);
    fixed_array.push(5);
    fixed_array.push(5);
    fixed_array.push(5);

    

    // fixed_array.filter(is_even); // Keep only even numbers
    println!("{:?}", fixed_array.get());
    fixed_array.sort(SortType::DESC);
    println!("Sorted DESC: {:?}", fixed_array.get()); // Should print [1, 2, 3, 4, 5]
    // Print the contents of the fixed array
}
