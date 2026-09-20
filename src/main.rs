
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

use sha2::{Sha256, Digest};

fn hash_string(input:&str)->String {
    let mut hasher=Sha256::new();
    hasher.update(input.as_bytes());
    let result=hasher.finalize();
    hex::encode(result)
}

fn main() {
    let h1 = hash_string("cat");
    let h2 = hash_string("bat");

    println!("H(\"cat\") = {}", h1);
    println!("H(\"bat\") = {}", h2);
}

