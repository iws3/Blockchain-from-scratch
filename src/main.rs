
// struct Person {
//     name:string,
//     age:u32
// }

// let gita=Person {
//     name:String::from("GIta"),
//     age:30
// }

// impl Person {
//     fn greet(&self)->String {
//         format("Hello, my name is {}", self.name)
//     }
// }

// // we are going to be using struct to reperesent blocks in a moment
// fn main() {
//     // println!("Hello, world!");
//     let name="GIta";
//     let mut counter=0;
//     counter+=1;

//     println!("Hello, {}! You have called this program {} times.", name, counter);
// }

// BASICS OF HASHING.......

// use sha2::{Sha256, Digest};

// fn hash_string(input:&str)->String {
//     let mut hasher=Sha256::new();
//     hasher.update(input.as_bytes());
//     let result=hasher.finalize();
//     hex::encode(result)
// }

// fn main() {
//     let h1 = hash_string("cat");
//     let h2 = hash_string("bat");

//     println!("H(\"cat\") = {}", h1);
//     println!("H(\"bat\") = {}", h2);
// }

// next step.. building our first block


use sha2::{Sha256, Digest};
use chrono::Utc;

struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block { // Fixed: 'BLock' -> 'Block'
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}


fn print_block(block:&Block) {
    // Borrow. doesn;t own
    println!("INdex: {}", block.index)
}

fn main() {
    let genesis_block = Block::new(
        0,
        String::from("Genesis Block"), // Fixed: added comma
        String::from("0"),
    ); // Fixed: added semicolon

    let block=Block::new(0, String::from("data"), String::from("0"));
     print_block(&block);
     print_block(&block); 
    println!("Index: {}", genesis_block.index);
    println!("Timestamp: {}", genesis_block.timestamp);
    println!("Data: {}", genesis_block.data);
    println!("Previous Hash: {}", genesis_block.previous_hash);
    println!("Hash: {}", genesis_block.hash);
}
