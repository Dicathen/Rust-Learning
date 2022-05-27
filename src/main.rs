#![allow(non_snake_case)]

use crate::horse::Horse;
mod horse;

fn main() {
    let horse1 = Horse{
        name: String::from("Greg"),
        breed: String::from("Arabian"),
        colorDescription: String::from("Light Brown"),
        otherComments: String::from("A bit dumb"),
        registrationID: String::from("412ACD"),
        spayedNeutered: String::from("Yes"),
        weight: 660.82,
        gender: 'm'
    };
    horse1.PrintInfo();
}
