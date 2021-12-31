mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // 相对路径 相当于 ./
            self::seat_at_table()
        }

        pub fn seat_at_table() {
            // 相对路径  相当于  ../
            super::serving::server_order();
        }
    }

    pub mod serving {
        fn take_order() {
            // 绝对路径
            crate::front_of_house::hosting::add_to_waitlist();
        }

        pub(crate) fn server_order() {}

        fn take_payment() {}
    }
}
// use crate::front_of_house::{hosting, serving};
// pub use crate::front_of_house::{hosting, serving};
pub use crate::front_of_house::*;
pub fn test_fn() {
    hosting::add_to_waitlist();
    serving::server_order();
}
