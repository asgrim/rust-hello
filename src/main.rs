//mod enums;
// mod game;
// mod ownership;
// mod structs;
mod back_of_house;
mod front_of_house;
mod structs;

use crate::front_of_house::hosting as Hello;

fn main() {
    // game::run();
    // ownership::run();
    structs::run();
    // enums::run();
    Hello::add_to_waitlist();
}
