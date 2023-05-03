// https://doc.rust-lang.org/stable/book/ch05-00-structs.html

fn main() {
    practice5_1();
    // practice5_2();
    // practice5_3();
}

fn practice5_1() {
    // 구조체 정의
    struct User {
        // 구조체의 field 정의
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 인스턴스 생성
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 인스턴스 필드 값 접근
    println!("user1's email: {}\n\n", user1.email);

    // 일반적으로 생성한 인스턴스는 필드 수정 불가
    // user1.username = String::from("junhee"); // 에러 발생

    // 특정 필드만 변경 가능 x => 모든 필드 변경 가능 or 변경 불가능

    // mut 인스턴스 생성(필드 변경 가능)
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("준희"),
        active: true,
        sign_in_count: 1,
    };
    println!("변경전 user2's username: {}", user2.username);
    user2.username = String::from("원준희");
    println!("변경후 user2's username: {}\n\n", user2.username);

    // 함수로 인스턴스 생성
    // 기본으로 설정할 필드 값이 있을 경우 편리
    fn build_user(email: String, username: String) -> User {
        // active와 sign_in_count는 기본값으로 설정
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    let user3 = build_user(String::from("hello@naver.com"), String::from("커피"));
    println!("user3's username: {}\n\n", user3.username);

    // field init shorthand 로 위 함수 간단하게
    fn build_user_by_field_init_shorthand(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let user4 =
        build_user_by_field_init_shorthand(String::from("hello@naver.com"), String::from("초코"));
    println!("user4's username: {}\n\n", user4.username);

    // 기존 인스턴스에서 Struct Update Syntax로 인스턴스 생성
    // let user5 = user4; 는 ownership이 move되서 user4 접근 불가
    // ..{인스턴스}는 맨 뒤에 있어야함
    let user5 = User {
        username: String::from("도넛"),
        ..user4
    };
    println!("user5's username: {}\n\n", user5.username);

    // 튜플 구조체
    // 필드의 이름이 명시되지 않음
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black의 첫번째 속성: {}\n\n", black.0);

    // 유사 유닛 구조체(unit-like structs)
    // trait에 사용됨
    struct Empty;
}

fn practice5_2() {
    // derive 어노테이션으로 Debug trait 추가
    #[derive(Debug)]
    struct Coffee {
        price: i32,
        rating: f64,
    }

    let americano = Coffee {
        price: 3000,
        rating: 4.5,
    };

    // 인스턴스 출력 불가
    // std::fmt::Display trait이 없어서
    // println!("Americano is {}", americano);

    // Debug 포맷으로 출력
    println!("Americano is {:?}\n\n", americano);

    // dbg! 메크로 사용
    // dbg!는 ownership을 가져갔다 반환함
    dbg!(&americano);
    println!("\n\nAmericano is {}", americano.price);
}

fn practice5_3() {
    struct Rectangle {
        height: u32,
        width: u32,
    }

    // impl: 구조체(struct), 열거형(enum), 트레이트(trait) 등의 타입에 대한 구현(implementation)을 정의하는 키워드
    impl Rectangle {
        // method 정의
        // &self 파라미터는 해당 인스턴스를 의미
        fn area(&self) -> u32 {
            self.height * self.width
        }
    }

    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    println!("rect1's area: {}", rect1.area());
    println!("rect1's width: {}\n\n", rect1.width);

    // 파라미터에 &self 대신 그냥 self를 사용하면 ownership을 가져온다
    impl Rectangle {
        fn area_move_ownership(self) -> u32 {
            self.height * self.width
        }
    }
    println!(
        "rect1's area (no return ownership): {}",
        rect1.area_move_ownership()
    );
    // println!("rect1's width: {}", rect1.width);
    print!("\n\n");

    // associated function
    // &self를 파라미터로 사용하지 않아서 method가 아니다
    // ex) String::from()
    impl Rectangle {
        fn create_squre(width: u32) -> Rectangle {
            Rectangle {
                width,
                height: width,
            }
        }
    }
    let square1 = Rectangle::create_squre(30);
    println!("square1's area: {}", square1.area());
}
