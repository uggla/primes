extern crate clap;
extern crate sum_primes;
use clap::{App, Arg};

fn main() {
    let matches = App::new("sum_prime")
        .version("1.0.0")
        .arg(
            Arg::with_name("number")
                .help("Output prime number until number")
                .index(1),
        )
        .get_matches();
    let n = matches
        .value_of("number")
        .expect("number needs to be provided")
        .parse::<u64>()
        .expect("number can not be parsed");

    let prime_numbers = sum_primes::eratosthene(n);
    println!("{:?}", &prime_numbers);
    let sum: u64 = prime_numbers.iter().sum();
    println!("Sum: {}", sum);
}
