extern crate sum_primes;

#[test]
fn test_sum_prime_eratosthene_100000() {
    assert_eq!(sum_primes::sum_eratosthene(100000), Some(454396537));
}
