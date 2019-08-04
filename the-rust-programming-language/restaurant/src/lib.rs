#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }

        fn seat_at_table() { }
    }

    pub mod serving {
        fn take_order() { }

        pub fn serve_order() {
            //here we go up two levels
            super::super::back_of_house::fix_incorrect_order();
         }

        fn take_payment() { }
        
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        //super means go up one level
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() { }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //if we make an enum public, all of its variants are then public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//front_of_house and eat_at_restaurant are siblings in this module, and thus can refer to each other
pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    //Change out mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}