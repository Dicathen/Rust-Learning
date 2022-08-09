#![allow(non_snake_case)]
use crate::Entities::Print;
mod Entities;

fn main() {
    let driver1 = Entities::Driver::new(1, String::from("John"), String::from("4"), false, 4.5, true, true, String::from("None"), Entities::Type::Compact);
    driver1.PrintInfo();

}
