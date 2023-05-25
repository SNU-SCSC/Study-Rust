/* Reference: Chapter 07, Listing 7-12
 * `use` 키워드를 사용하여 shortcut을 설정할 수 있음
 * 해당 shortcut은 `use`가 사용된 모듈 안에서 사용할 수 있음
 */
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // super::hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
}

fn main() {
    customer::eat_at_restaurant();
}
