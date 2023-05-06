#![allow(non_snake_case)]
#![allow(dead_code)]
/*use crate::Entities::Print;
mod Entities;

fn main() {
    let driver1 = Entities::Driver::new(1, String::from("John"), String::from("4"), false, 4.5, true, true, String::from("None"), Entities::Type::Compact);
    driver1.PrintInfo();
}*/
use f256::f256;
fn factorial(n: i128) -> i128 {
    return (1..=n).fold(1, |a, b| a * b);
}

fn iteration(power: i128, base: i128) -> f256 {
    let returnNum: f256 = f256::from(base.pow(power as u32)) / f256::from(factorial(power));
    returnNum
}

fn ExpEstimation(power: i128, errorBound: f256) -> f256 {
    let mut taylorRemainder: f256 = f256::from(0.0);
    let mut accum: f256 = f256::from(1.0) + f256::from(power);
    let mut iterationNum: i128 = 2;
    loop {
        accum = accum + iteration(iterationNum, power);
        taylorRemainder = iteration(iterationNum + 1, power);
        println!("Iteration {}", iterationNum);
        println!("Taylor Remainder is {}", taylorRemainder);
        println!("Accum is {}", accum);
        println!("Factorial is {}", factorial(iterationNum));
        iterationNum = iterationNum + 1;
        if taylorRemainder < errorBound {
            break;
        }
    }
    accum
}

fn main() {
    let errorBound: f256 = f256::from(0.000000000000000000000000001);
    let power: i128 = 1;
    let estimation: f256 = ExpEstimation(power.into(), errorBound);
    println!("Estimation is {}", estimation);
}
