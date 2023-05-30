// Ch.8 Hash Map

// HashMap<Key, Value>
// This is similar to C++'s "map" and Python's "dictionary".

// The following code is essential.
use std::collections::HashMap;

// Creating a new Hash Map
/*
fn main() {
    let a: HashMap<&str, i32> = HashMap::new();     // It requires Key's type and Value's type.
}
*/

// Recall creating a new vector.
// The followings are wrong.
// The compiler cannot determine the type of Key and Value.
/*
fn main() {
    let mut a = HashMap::new();
    let b: HashMap<_, _> = HashMap::new();
}
*/

// Inserting values in a Hash Map
/*
fn main() {
    // (1) Use insert.
    let mut a = HashMap::new();
    a.insert("SCSC", 438);          // The compiler can determine the type of Key and Value.

    let mut b: HashMap<_, _> = HashMap::new();
    b.insert(438, "SCSC");

    // (2) Use "zip" method and "collect" method.
    // "zip" generates a vector of tuples.
    // "collect" converts gathered data into various types of collections.
    let name = vec!["SCSC", "내마음"];
    let num = vec![438, 437];
    let 동아리방: HashMap<_, _> = name.iter().zip(num.iter()).collect();
}
*/

// Accessing values in a Hash Map
// (1) "get" method
/*
fn main() {
    let mut ten = HashMap::new();
    ten.insert(1, 10);
    ten.insert(2, 20);

    let b = 1;
    let b10 = ten.get(&b);          // The type of b10 is Option<&i32>. b10 is equal to Some(&10).
    
    let c = 3;
    let c10 = ten.get(&c);          // c10 is equal to None. 3 is an invalid key.

    // We want to use i32.
    // (i) Option<&i32> -> Option<i32> by "copied" method.
    // (ii) Option<i32> -> i32 by "unwrap" method.
    let d = 2;
    let d10 = ten.get(&d).copied().unwrap();
    println!("{}", d10);

    // None -> i32 by "unwrap_or" method.
    let e = 5;
    let e10 = ten.get(&e).copied().unwrap_or(0);
    println!("{}", e10);
}
*/

// (2) Iteration
/*
fn main() {
    let mut ten = HashMap::new();
    ten.insert(1, 10);
    ten.insert(2, 20);

    for (key, value) in &ten {
        println!("{}: {}", key, value);
    }
    for (key, value) in ten {               // In this case, the ownership of ten is moved.
        println!("{}: {}", key, value);
    }
    // ten is unavailable.
    /*
    for (key, value) in &ten {
        println!("{}: {}", key, value);
    }
    */
}
*/

// Hash Maps and Ownership
/*
fn main() {
    let mut a = HashMap::new();
    let x = String::from("아디오스");
    let y = String::from("베이비");
    println!("끝이다. 유언을 남겨라.\n{} {}...", x, y);
    
    a.insert(x, y);     // x, y are moved into a.
    // println!("끝이다. 유언을 남겨라.\n{} {}...", x, y);   // x, y is unavailable.
}
*/

// Updating a Hash Map
// Overwriting a value. One key should have one value.
/*
fn main() {
    let mut a = HashMap::new();
    a.insert(1, 10);
    a.insert(2, 20);
    a.insert(1, 100);       // Overwriting.
    println!("{:?}", a);    // This format outputs all {key : value} in a Hash Map.
}
*/

// Adding a Key and Value only iff a Key does not exist.
// "entry" method for Hash Map and "or_insert" method for Entry
/*
fn main() {
    let mut a = HashMap::new();
    a.insert(1, 10);
    a.insert(2, 20);

    a.entry(1).or_insert(100);
    a.entry(3).or_insert(30);
    println!("{:?}", a);
}
// There is one observation that can be made through println!("{:?}", a);.
// In Rust, a HashMap does not guarantee a specific order for its entries.
// It is similar to the dictionary in older versions of Python.
*/

// Updating a value based on the old value
// We utilize the fact that "entry" method returns a reference to the Entry if the key exists.
// "or_insert" method returns a reference to the value of the Entry.
/*
fn main() {
    let mut a = HashMap::new();
    a.insert(1, 10);
    a.insert(2, 20);
    a.insert(3, 30);

    for i in 1..4 {
        // let val = a.entry(i);    // Modifying an Entry is not allowed.
        let val = a.entry(i).or_insert(0);
        *val *= 10;
    }
    println!("{:?}", a);
}
*/
