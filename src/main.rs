fn fibonacci_memoized(n: u64, previous_results: &mut Vec<u64>) -> u64 {
    if previous_results.len() >= (n + 1).try_into().unwrap() {
        return previous_results[n as usize];
    }
    if n == 0 || n == 1 {
        return 1;
    }
    let result = fibonacci_memoized(n - 1, previous_results) + fibonacci_memoized(n - 2, previous_results);
    if previous_results.len() == n as usize {
        previous_results.push(result);
    }
    result
}


fn fibonacci(n: u64) -> u64 {
    let mut previous_results = vec![1u64, 1];
    return fibonacci_memoized(n, &mut previous_results);
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
    assert_eq!(fibonacci(49), 12586269025);
    assert_eq!(fibonacci(60), 2504730781961);
    assert_eq!(fibonacci(70), 308061521170129);
    assert_eq!(fibonacci(80), 37889062373143906);
    assert_eq!(fibonacci(90), 4660046610375530309);
    assert_eq!(fibonacci(92), 12200160415121876738);
    // This one is too big for u64
    // assert_eq!(fibonacci(93), ????);
}
