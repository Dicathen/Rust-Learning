#![allow(non_snake_case)]

fn factorial(n: i128) -> i128
{
	if n == 0 || n == 1 {1} else {n * factorial(n-1)}
}

fn iteration (power: i128, base: i128) -> f64
{
	let returnNum: f64 = base.pow(power as u32) as f64 /factorial(power) as f64;
	returnNum
}

fn ExpEstimation(power: i128, errorBound : f64) -> f64
{
	let mut taylorRemainder: f64 = 0.0;
	let mut accum: f64 = 1.0 + power as f64;
	let mut iterationNum: i128 = 2;
	loop {
		accum = accum + iteration(iterationNum,power);
		taylorRemainder = iteration(iterationNum + 1,power);
		println!("Iteration {}", iterationNum);
		println!("Taylor Remainder is {}",taylorRemainder);
		println!("Accum is {}", accum);
		println!("Factorial is {}",factorial(iterationNum));
		iterationNum = iterationNum + 1;
	   if taylorRemainder < errorBound {
	   	break;
	   } 
	}
	accum
}

fn main()
{
	let errorBound: f64 = 0.0000000000000000000000001;
	let power: i128 = 1;
	let estimation: f64 = ExpEstimation(power.into(),errorBound);
	println!("Estimation is {}", estimation);
}
