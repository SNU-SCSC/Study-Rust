/* Chapter 07, Listing 7-3
 * 모듈과 그 구성 요소들은 private이 기본값
 * 한 모듈 안에 속한 변수/함수/모듈들끼리는 private이어도 참조할 수 있지만
 * 변수/함수/모듈이 private이면 해당 구성요소가 속한 모듈 밖에서 참조할 수 없음
 * 참조하고 싶으면 `pub` 키워드를 사용해 해당 구성요소를 public으로 선언해야 함
 */
mod front_of_house {
    pub mod hosting {
    // mod hosting {
        pub fn add_to_waitlist() {}
        // fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path (does not work)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (does not work)
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
