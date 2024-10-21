use std::collections::LinkedList;

// Define the key-value pair type
type KeyValue<K, V> = (K, V);

// Hash Table Structure
pub struct HashTable<K, V> {
    buckets: Vec<LinkedList<KeyValue<K, V>>>,
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    // Create a new HashTable with a given number of buckets
    pub fn new(size: usize) -> Self {
        let mut buckets: Vec<LinkedList<(K, V)>> = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(LinkedList::new());
        }
        Self { buckets, size }
    }

    // Simple hash function
    fn hash(&self, key: &K) -> usize {
        use std::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;

        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.size
    }

    // Insert key-value pair
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket: &mut LinkedList<(K, V)> = &mut self.buckets[index];
        
        // Check if key exists, if it does, update the value
        for kv in bucket.iter_mut() {
            if kv.0 == key {
                kv.1 = value;
                return;
            }
        }

        // Otherwise, add the new key-value pair
        bucket.push_back((key, value));
    }

    // Get the value associated with a key
    pub fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        for kv in bucket.iter() {
            if kv.0 == *key {
                return Some(kv.1.clone());
            }
        }
        None
    }

    // Remove a key-value pair
}

fn main() {
    let mut table = HashTable::new(10);
    
    // Insert some key-value pairs
    table.insert("apple", 3);
    table.insert("banana", 7);
    table.insert("orange", 2);

    // Retrieve values
    if let Some(value) = table.get(&"apple") {
        println!("apple = {}", value);
    } else {
        println!("apple not found");
    }

    // Remove a key
    // table.remove(&"banana");

    if let Some(value) = table.get(&"banana") {
        println!("banana = {}", value);
    } else {
        println!("banana not found");
    }
}