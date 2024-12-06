mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // đường dẫn tuyệt đối
    crate::front_of_house::hosting::add_to_waitlist();

    // đường dẫn tương đối
    front_of_house::hosting::add_to_waitlist();
}