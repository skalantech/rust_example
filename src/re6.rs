#![allow(unused)]
pub fn example() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{my_string}");
    println!("{my_str}");

    let num = Number::from(30);
    println!("The number is {:?}", num);
    let int = 5;
    let num1: Number = int.into();
    println!("The number is {:?}", num1);
}

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}
