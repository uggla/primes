extern crate sum_primes;

#[test]
fn test_sum_prime_100000() {
    assert_eq!(sum_primes::sum(100000), Some(454396537));
}
