/* Reference: Chapter 07, Listing 7-13
 * `as` 키워드를 사용하여 alias를 만들 수 있음
 */
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}

use crate::front_of_house::hosting::add_to_waitlist as my_add_to_waitlist;

pub fn drink_at_restaurant() {
    my_add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
    drink_at_restaurant();
}
