fn main() {

// default로 모든 변수는 immutable이라 아래와 같이 두 번 할당할 경우 에러 발생
/* 
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); // cannot assign twice to immutable variable
*/

// mut로 변수를 설정하면 에러가 발생하지 않는다
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


// 상수 선언방식
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


// Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

/*
    let mut spaces = "   ";
    spaces = spaces.len();
    // mut을 붙일 경우 type이 변화하면 에러발생
    // mut 변수는 값을 할당할 때 처음 값을 할당한 타입으로 고정!!!
    // 왠만하면 let (불변) 변수를 사용하자!!! 
*/

// Rust는 statically typed language: compile time에 모든 변수들의 타입을 알아야 한다.
    let guess: u32 = "42".parse().expect("Not a number!");


    let a: u8 = 200;
    let b: u8 = 200;
    // let d: u8 = 300;
    let c: u8 = a*b;
    println!("{c}");

// 자료구조 - 튜플
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

// 자료구조 - 어레이
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


// 함수 정의해서 사용하는 예시
    let x = plus_one(5);

    println!("The value of x is: {x}");

}

fn plus_one(x: i32) -> i32 {
    x + 1
}