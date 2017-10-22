extern crate hashmap;

use hashmap::Hash;

fn main() {
    println!("starting hashmap");
    let mut hash: Hash = Hash::new();
    hash.put("akey", 666);
    println!("existing key akey is {}", hash.get("akey"));
    println!("non existing key akey2 is {}", hash.get("akey2"));
}
