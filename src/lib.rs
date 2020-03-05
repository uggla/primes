fn is_prime(n: u64) -> bool {
    if n == 1 {
        false
    } else {
        let max = (n as f32).sqrt() as u64;
        !(2..max + 1).any(|x| n % x == 0)
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

pub fn eratosthene(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    /*  Create a boolean array "prime[0..n+1]" and initialize
        all entries it as true. A value in prime[i] will
        finally be false if i is Not a prime, else true.
    */
    let mut table: Vec<bool> = vec![true; (n + 1) as usize];
    let mut prime = 2;

    if n < 2 {
        return Vec::new();
    }

    while prime * prime <= n {
        // If table[prime] is not changed, then it is a prime
        if table[prime as usize] == true {
            // Update all multiples of prime
            for item in (prime * prime..=n).step_by(prime as usize) {
                table[item as usize] = false;
            }
        }
        prime += 1;
    }

    // Create a list with all prime numbers
    for item in 2..=n {
        if table[item as usize] {
            primes.push(item)
        }
    }
    primes
}

pub fn sum(n: u64) -> Option<u64> {
    let prime_list = get_prime_list(n);
    match prime_list.is_empty() {
        true => None,
        false => Some(prime_list.into_iter().sum()),
    }
}

pub fn sum_eratosthene(n: u64) -> Option<u64> {
    let prime_list = eratosthene(n);
    match prime_list.is_empty() {
        true => None,
        false => Some(prime_list.into_iter().sum()),
    }
}
