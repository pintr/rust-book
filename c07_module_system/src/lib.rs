//! System module examples

// The following module has been migrated to the front_of_house module
// #[allow(dead_code)]
// pub mod front_of_house {
//     // mod hosting { // Private module, cannot be used outside of front_of_house
//     pub mod hosting {
//         // fn add_to_waitlist() {} // Private function, cannot be used outside of hosting
//         pub fn add_to_waitlist() {}

//         pub fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// The front_of_house module is now imported here, with the hosting module being imported as well
// In a way, the mod keyword loads the module as it is in the current scope, meaning that the content of front_of_house is now available in this module
pub mod front_of_house;
// pub use crate::front_of_house::hosting; // Absolute path
pub use front_of_house::hosting; // Relative path
                                 // If a file is part of the module tree, it can be loeaded just by using the mod keyword
                                 // Other files in the project should refer to the already loaded module using a path to wher it was declared.

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path, starting from the crate root
    // crate::front_of_house::hosting::add_to_waitlist(); // Doesn't work anymore, because fron_of_the_house and hosting have been moved to their own modules

    // Relative path, starting from the current module
    // front_of_house::hosting::seat_at_table(); // Doesn't work anymore, because fron_of_the_house and hosting have been moved to their own modules

    hosting::add_to_waitlist(); // This wouldn't work without the `use front_of_house::hosting;` statement,
    hosting::seat_at_table();
    front_of_house::hosting::add_to_waitlist(); // This refers to the hosting module in the front_of_house module, so it works without use

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    println!("I'd like {} toast please", meal.toast);

    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.season_fruit = String::from("blueberries"); // This will throw an error because seasonal_fruit is private

    let order1 = crate::back_of_house::Appetizer::Salad; // Absolute path
    let order2 = back_of_house::Appetizer::Soup; // Relative path

    println!("Orders: {:?}, {:?}", order1, order2);
}
