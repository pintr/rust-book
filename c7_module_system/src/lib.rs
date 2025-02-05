/// System module examples
#[allow(dead_code)]
mod front_of_house {
    // mod hosting { // Private module, cannot be used outside of front_of_house
    pub mod hosting {
        // fn add_to_waitlist() {} // Private function, cannot be used outside of hosting
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path, starting from the crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, starting from the current module
    front_of_house::hosting::seat_at_table();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    println!("I'd like {} toast please", meal.toast);

    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.season_fruit = String::from("blueberries"); // This will throw an error because seasonal_fruit is private

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    println!("Orders: {:?}, {:?}", order1, order2);
}
