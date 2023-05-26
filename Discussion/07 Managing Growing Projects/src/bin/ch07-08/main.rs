/* Reference: Chapter 07, Listing 7-8
 * 모듈 밖에서는 모듈 안을 참조할 수 없지만, 반대는 가능함
 * `super` 키워드를 사용하여 상위 모듈을 참조할 수 있음
 */
fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn main() {
    back_of_house::fix_incorrect_order();
}
