mod front_of_house;

mod back_of_house{
    pub struct Breakfast{
        toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("Peach"),
            }
        }
    }

    pub enum Appetizer{
        Soup,
        Salad,
    }

}

use crate::front_of_house::hosting;


pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("ddfhd");
    //meal.toast=String::from("nfbd"); //error
    meal.seasonal_fruit=String::from("nfbd");

    let order1= back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
    front_of_house::serving::take_order();

}

use std::io::{self, Write};
