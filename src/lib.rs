extern crate rayon;
use rayon::prelude::*;

fn is_prime(n: u64) -> bool {
    if n == 1 {
        false
    } else {
        !(2..n).into_par_iter().any(|x| n % x == 0)
    }
}

pub fn get_prime_list(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return Vec::new();
    };
    for item in 2..=n {
        if is_prime(item) {
            primes.push(item)
        }
    }
    //println!("{:#?}", primes);
    primes
}

pub fn sum(n: u64) -> Option<u64> {
    let prime_list = get_prime_list(n);
    match prime_list.is_empty() {
        true => None,
        false => Some(prime_list.into_iter().sum()),
    }
}
