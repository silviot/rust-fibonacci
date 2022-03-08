fn fibonacci_memoized(n: u64, previous_results: &mut Vec<u64>) -> u64 {
    // Base of the recursion: if we're asked an index lower than
    // the size of previous_results it means we're able to return
    // a value from the vector instead of calculating it again.
    if previous_results.len() >= (n + 1).try_into().unwrap() {
        return previous_results[n as usize];
    }
    // In case we don't have the value yet we calculate it
    let result = fibonacci_memoized(n - 1, previous_results) + fibonacci_memoized(n - 2, previous_results);
    // and if it's the right one to go next put it into the vector
    if previous_results.len() == n as usize {
        previous_results.push(result);
    }
    // and return it
    result
}


fn fibonacci(n: u64) -> u64 {
    // The array previous_results contains all fibonacci numbers
    // calculated so far. fibonacci(0) is at index 0, and so on.
    // We know the first two fibonacci numbers are 0 and 1.
    let mut previous_results = vec![0u64, 1];
    return fibonacci_memoized(n, &mut previous_results);
}

fn main() {
    // assign 20 to the variable num
    let num = 40;
    println!("Fibonacci number {} is {}", num, fibonacci(num));
}

#[test]
fn base_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
}

#[test]
fn slow_cases() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
    assert_eq!(fibonacci(30), 832040);
    assert_eq!(fibonacci(40), 102334155);
    assert_eq!(fibonacci(50), 12586269025);
    assert_eq!(fibonacci(60), 1548008755920);
    assert_eq!(fibonacci(70), 190392490709135);
    assert_eq!(fibonacci(80), 23416728348467685);
    assert_eq!(fibonacci(90), 2880067194370816120);
    assert_eq!(fibonacci(93), 12200160415121876738);
    // This one is too big for u64
    // assert_eq!(fibonacci(94), ????);
}
