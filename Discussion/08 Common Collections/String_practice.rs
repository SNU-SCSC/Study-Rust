// Ch.8 String

// String is mutable.
// String has an ownership.
// String is encoded in UTF-8.
// Note that "str" and "String" are different.

// Creating a new String.
// Emtpy String.
/*
fn main() {
    let s = String::new();      // But, immutable empty String is useless.
    let mut s = String::new();
}
*/

// String with an inital value.
/*
fn main() {
    let data = "init";
    let s = data.to_string();
    let s = "init".to_string();
    let s = String::from("init");
}
*/

// Updating a String
// Appending with "push_str" and "push"
/*
fn main() {
    let mut s = String::from("S");
    s.push_str("C");                // The type of "C" is &str.

    let s2 = "S";
    s.push_str(s2);                 // The type of s2 is &str.
    s.push_str(&s2);                // Note that &s2 is also of type &str.
    println!("s2 is {}", s2);

    s.push('C');                    // The type of 'C' is char.
    println!("{}", s);

    let s3: char = '!';
    s.push(s3);
    println!("{}", s);              // "push" requires a char argument.
}
*/

// Concatenation
// (1) "+" Operation
/*
fn main() {
    let s1 = String::from("SC");
    let s2 = String::from("SC");
    let s3 = s1 + &s2;              // s1 becomes unavailable.
}
*/

// The implementation of "+" for a String.
// fn add(self, s: &str) -> String
// The first term: self - It does not have &.
// The second term: s: &str - It has &.
/*
fn main() {
    let s1 = String::from("1");
    let s2 = String::from("2");
    let s3 = String::from("3");
    let s = s1 + "+" + &s2 + "+" + &s3;     // In this case, using "+" is too complex.
    println!("{}", s);

    // (2) "format!" macro
    let s1 = String::from("응애");             // We need new s1 based on the above result.
    let so_gorgeous_s = format!("{}+{}+{}", s1, s2, s3);
    println!("{}", so_gorgeous_s);
    println!("아앗 너는 s1? {}", s1);           // Macro uses references.
}
*/

// Indexing into Strings
// The following code will produce an error.
/*
fn main() {
    let s = String::from("Hi");
    let a = s[0];                   // String cannot be indexed by an integer.
}
*/
// An index into the String’s bytes will not always correlate to a valid Unicode scalar value.

// Three ways to look at Strings from Rust’s perspective
// (1) Byte (2) Scalar value (= char) (3) Grapheme cluster

// Slicing Strings
/*
fn main() {
    let s = String::from("Hi");
    let a = &s[0..2];               // We must use &.
    // let a = s[0..2];             // It occurs an error.
    println!("{}", a);

    let b = &s[0..1];
    println!("{}", b);              // This slicing works just like indexing.

    // Slicing the String based on bytes.
    let k = String::from("한국말");
    let c = &k[0..3];               // "한" occupies 3 bytes.
    // let d = &k[0..2];
    // let e = &k[0..7];            // They occur errors.
    println!("{}", c);
}
*/
