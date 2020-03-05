extern crate sum_primes;

#[test]
fn test_sum_prime_eratosthene_1_million() {
    assert_eq!(sum_primes::sum_eratosthene(1000000), Some(37550402023));
}
