extern crate sum_primes;

#[test]
fn test_sum_prime_1_million() {
    assert_eq!(sum_primes::sum(1000000), Some(37550402023));
}
