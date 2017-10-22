use std;

#[derive(Debug)]
pub struct Hash {
    data: std::vec::Vec<u32>,
}

impl Hash {
    pub fn new() -> Hash {
        Hash { data: vec![0; u32::max_value() as usize] }
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    pub fn put(&mut self, key: &str, value: u32) {
        let index = self.hash_func(key);
        self.data[index as usize] = value;
    }

    pub fn get(&mut self, key: &str) -> u32 {
        let index = self.hash_func(key);
        return self.data[index as usize];
    }

    fn hash_func(&mut self, key: &str) -> i32 {
        let mut hash: i32 = 5381;
        let bytes: &[u8] = key.as_bytes();
        for b in bytes {
            hash = ((hash << 5) + hash) + *b as i32;
        }
        return hash.abs();
    }
}
