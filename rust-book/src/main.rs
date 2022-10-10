use chapters::ch16;
use chapters::ch3;
use chapters::ch4;

use crate::chapters::ch8;

#[allow(dead_code)]
mod chapters {
    pub mod ch3;
    pub mod ch4;
    pub mod ch8;
    pub mod ch16;
}

fn main() {
    ch3::temperature_c_to_f(0.);
    ch3::temperature_f_to_c(0.);
    let fib4 = ch3::fibonacci(4);
    let fib20 = ch3::fibonacci(20);
    println!("The 4th Fibonacci number is {fib4}.");
    println!("The 20th Fibonacci number is {fib20}.");
    ch3::fibonacci(-2);

    let mut s = String::from("hello");
    ch4::change_string(&mut s);

    let median1 = ch8::get_median(&mut vec![5, 3, 5, 7, 9]);
    let median2 = ch8::get_median(&mut vec![5, 3, 7, 9]);
    println!("median1 is {median1}; median2 is {median2}");

    let mode1 = ch8::get_mode(&mut vec![5, 3, 5, 7, 9]);
    let mode2 = ch8::get_mode(&mut vec![3, 5, 7, 9]);
    println!("mode1 is {}; mode2 is {}", mode1, mode2);

    ch16::send_between();
}
