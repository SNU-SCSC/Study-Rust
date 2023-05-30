// Ch.8 Vector

// Creating a new vector
// (1) Use Vec::new function
fn main() {
    // let v: Vec<T> = Vec::new();
    let v: Vec<char> = Vec::new();
    let v: Vec<i32> = Vec::new();
    let v: Vec<&str> = Vec::new();
    let v: Vec<String> = Vec::new();
}

// If we don't specify the type of the vector, we should use "mut".
// The following code will produce an error.
fn main() {
    let v = Vec::new();
}

// Unfortunately, the following code will also produce an error.
fn main() {
    let mut v = Vec::new();
}

// The reason is still that the compiler cannot determine the type of the vector's elements.
// Therefore, by inserting elements as follows, no error occurs.
fn main() {
    let mut v = Vec::new();
    v.push(1);
    // v.push("Hello");
}

// (2) Use vec! method
fn main() {
    let v = vec!["응", "애"];
    // let v = vec![1, 2, 3, 4];
}

// Updating a vector
// We can update a vector by using push method. Do not forget to use "mut".
fn main() {
    let mut v: Vec<&str> = Vec::new();
    v.push("응");
    v.push("애");
    let mut v = Vec::new();
    v.push("응");
    v.push("뀨");
}

// Reading elements of a vector
// There are two ways. (1) Use index. (2) Use get method.
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let first: i32 = v[0];
    let second: &i32 = &v[1];
    let third: Option<i32> = Some(v[2]);    // We must use "Some".
    let fourth: Option<&i32> = Some(&v[3]);
    let fifth: Option<&i32> = v.get(4);     // We must not use "Some".
    
    // The following codes will produce errors.
    // let third: Option<i32> = v[2];               // v[2] is i32.
    // let fourth: Option<&i32> = Some(v[3]);       // We need &.
    // let fifth: Option<&i32> = Some(v.get(4));    // v.get(4) is already Option<&i32>.
    // let fifth: &i32 = v.get(4);
}

// Out of range
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let a = v.get(100);
    // get method returns a value of type Option<&T>. So a is None.
    
    // The following code will produce an error.
    // let a = v[100];
    // This is not a compile error. This is a runtime error. This causes "panic!".
}

// Invalid reference
fn main() {
    let mut v = vec![1];
    let a = &v[0];
    v.push(3);

    // If we don't use the variable "a", no error occurs.
    // By adding the following code, the same error as described in the book occurs.
    // println!("{}", a);
}

// Consider the ownership when Strings are in a vector.
fn I_will_steal_your_ownership(s: String) {
    println!("빼앗았다. {}", s);
}
fn I_dont_need_your_ownership(s: &String) {
    println!("너의 소유권은 필요 없었다. {}", s);
}
fn main() {
    // let v = vec!["뀨잉..."]              // In this case, the type of "뀨잉..." is &str.
    let v = vec![String::from("뀨잉...")];  // To use String, we must use String::from or to_string.
    let s = String::from("r");
    I_dont_need_your_ownership(&v[0]);
    println!("{}", v[0]);
    
    // The following code will produce an error.
    // I_will_steal_your_ownership(v[0]);
    // We cannot move the ownership of an element inside a vector.
}

// Iterating over the values in a vector.
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    /*
    for i in v {                    // Impossible. We need &.
        println!("{}", i);
    }
    */
    // Using reference.
    for i in &v {
        println!("{}", i);
    }

    /*    
    for i in &mut v {               // Impossible. Note that v is immutable. 
        *i += 5;
        println!("{}", i);
    }
    */
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {               // Now, it is possible.
        *i += 5;
        println!("{}", i);
    }

    // Using iterator.
    for i in v.iter() {
        println!("{}", i);
    }
    for i in v.iter().rev() { 
        println!("{}", i);
    }

    // Using index.
    for i in 0..5 {
        println!("{}", v[i]);
    }
    for i in (0..5).rev() {
        println!("{}", &v[i]);
    }
    /*
    for i in 0.. {                  // It occur "panic!" since the size of v is 5.
        println!("{}", v[i]);
    }
    */
}

// Using an enum to store multiple types
enum Info {
    CodeName(char),
    Age(i32),
    Ability(String)
}
fn main() {
    // We can store values of multiple types in a vector.
    // Actually, to be precise, they are all of the type Info.
    let info1 = vec![
        Info::CodeName('S'),
        Info::Age(24),
        Info::Ability(String::from("바람과 함께 사라지기"))
    ];
    
    let mut info2: Vec<Info> = Vec::new();
    info2.push(Info::Age(24));
}

// When a vector goes out of scope, both the vector and its elements are dropped.
fn main() {
    let mut ans = "쀼";
    {
        let v = vec!["구", "뀨"];
        println!("비둘기가 두 번 울면?\n답: {}", v[1]);
        ans = &v[1];
    }
    // The following code will produce an error.
    // println!("비둘기가 두 번 울면?\n답: {}", v[1]);
    // But, the code below is fine.
    println!("비둘기가 두 번 울면?\n답: {}", ans);
}
