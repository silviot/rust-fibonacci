fn fibonacci(n: u64) -> u64 {
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

#[test]
fn base_cases() {
    assert_eq!(fibonacci(0), 1);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 2);
    assert_eq!(fibonacci(3), 3);
    assert_eq!(fibonacci(4), 5);
}

#[test]
fn slow_cases() {
    assert_eq!(fibonacci(10), 89);
    assert_eq!(fibonacci(20), 10946);
    assert_eq!(fibonacci(30), 1346269);
    assert_eq!(fibonacci(40), 165580141);
    assert_eq!(fibonacci(50), 12586269025);
}
