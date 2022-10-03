extern crate core;

fn main() {
    ch3_temperature_c_to_f(0.);
    ch3_temperature_f_to_c(0.);
    let fib4 = ch3_fibonacci(4);
    let fib20 = ch3_fibonacci(20);
    println!("The 4th Fibonacci number is {fib4}.");
    println!("The 20th Fibonacci number is {fib20}.");
    ch3_fibonacci(-2);
}

fn ch3_temperature_f_to_c(fahrenheit: f32) -> f32 {
    let result = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{fahrenheit}°F = {result}°C");
    return result;
}

fn ch3_temperature_c_to_f(celsius: f32) -> f32 {
    let result = (celsius * (5.0 / 9.0)) + 32.0;
    println!("{celsius}°C = {result}°F");
    return result;
}

fn ch3_fibonacci(n: i32) -> i32 {
    if n <= 0 {
        println!("Number must be bigger than 0.");
        return 0;
    }
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        ch3_fibonacci(n - 1) + ch3_fibonacci(n - 2)
    }
}
