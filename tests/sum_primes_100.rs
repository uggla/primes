extern crate sum_primes;

#[test]
fn test_sum_prime_100() {
    assert_eq!(sum_primes::sum(100), Some(1060));
}
