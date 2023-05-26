/* Reference: Chapter 07, Listing 7-17
 * `pub use` 키워드를 사용하여 shortcut을 re-export할 수 있음
 */
mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub use hosting::add_to_waitlist;
    pub mod counter {
        pub fn greet() {
            super::add_to_waitlist()
        }
    }
}

pub fn eat_at_restaurant() {
    // Incorrect since `front_of_house::hosting` is private
    // front_of_house::hosting::add_to_waitlist();

    // Correct
    front_of_house::add_to_waitlist();
}

fn main() {
    front_of_house::counter::greet();
}
