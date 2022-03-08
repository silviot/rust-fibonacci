fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    // assign 20 to the variable num
    let num = 40;
    println!("Fibonacci number {} is {}", num, fibonacci(num));
}
