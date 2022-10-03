pub fn temperature_f_to_c(fahrenheit: f32) -> f32 {
    let result = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{fahrenheit}°F = {result}°C");
    return result;
}

pub fn temperature_c_to_f(celsius: f32) -> f32 {
    let result = (celsius * (5.0 / 9.0)) + 32.0;
    println!("{celsius}°C = {result}°F");
    return result;
}

pub fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        println!("Number must be bigger than 0.");
        return 0;
    }
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}