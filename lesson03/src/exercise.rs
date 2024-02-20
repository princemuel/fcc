fn hi() {
    let deg = 100.0;
    let fahrenheit = c_to_f(deg);
    let celsius = f_to_c(deg);
    let n = 9;

    println!("The value of {deg}C in fahrenheit is {fahrenheit}");
    println!("The value of {deg}F in celsius is {celsius}");
    println!("The {n}th Fibonacci number: {}", fib(n))
}

fn c_to_f(value: f64) -> f64 {
    (value * (9.0 / 5.0)) + 32.0
}

fn f_to_c(value: f64) -> f64 {
    (value - 32.0) * (5.0 / 9.0)
}

// fn fib(n: i32) -> i32 {
//     if n <= 1 {
//         return n;
//     }
//     return fib(n - 1) + fib(n - 2);
// }

fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c: i32;
    let mut index = 2;

    if n == 0 {
        return a;
    }

    while index <= n {
        c = a + b;
        a = b;
        b = c;

        index += 1
    }

    return b;
}
