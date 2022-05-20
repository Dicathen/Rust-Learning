#![allow(non_snake_case)]
struct Example
{
    a_integer: i32
}

impl Example
{
    fn new(tempInt: i32) -> Self
    {
        Example
        {
            a_integer: tempInt
        }
    }
    fn printString(&self)
    {
        println!("{}",self.a_integer)
    }
}

fn main() {
    println!("Hello, world!");
    let exampleData = Example::new(32);
    exampleData.printString();
}
