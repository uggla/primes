fn is_prime(n: u32) -> bool {
    if n == 1 {
        false
    } else {
        let max = (n as f32).sqrt() as u32;
        !(2..max + 1).any(|x| n % x == 0)
    }
}

pub fn get_prime_list(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
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

pub fn eratosthene(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut table: Vec<bool> = vec![true; (n + 1) as usize];
    let mut index = 2;

    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return Vec::new();
    };

    while index * index <= n {
        if table[index as usize] == true {
            for i in (index * index..n + 1).step_by(index as usize) {
                table[i as usize] = false;
            }
        }
        index += 1;
    }

    for item in 2..=n {
        if table[item as usize] {
            primes.push(item)
        }
    }
    primes
}

pub fn sum(n: u32) -> Option<u32> {
    let prime_list = get_prime_list(n);
    match prime_list.is_empty() {
        true => None,
        false => Some(prime_list.into_iter().sum()),
    }
}

pub fn sum_eratosthene(n: u32) -> Option<u32> {
    let prime_list = eratosthene(n);
    match prime_list.is_empty() {
        true => None,
        false => Some(prime_list.into_iter().sum()),
    }
}
