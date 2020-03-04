extern crate sum_primes;

#[test]
fn test_get_prime_0() {
    assert_eq!(sum_primes::get_prime_list(0), vec![]);
}

#[test]
fn test_get_prime_1() {
    assert_eq!(sum_primes::get_prime_list(1), vec![]);
}

#[test]
fn test_get_prime_2() {
    assert_eq!(sum_primes::get_prime_list(2), vec![2]);
}

#[test]
fn test_get_prime_3() {
    assert_eq!(sum_primes::get_prime_list(3), vec![2, 3]);
}

#[test]
fn test_get_prime_100() {
    assert_eq!(
        sum_primes::get_prime_list(100),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}

#[test]
fn test_get_prime_1000() {
    assert_eq!(
        sum_primes::get_prime_list(1000),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997
        ]
    );
}

#[test]
fn test_sum_prime_0() {
    assert_eq!(sum_primes::sum(0), None);
}

#[test]
fn test_sum_prime_1() {
    assert_eq!(sum_primes::sum(1), None);
}

#[test]
fn test_sum_prime_2() {
    assert_eq!(sum_primes::sum(2), Some(2));
}

#[test]
fn test_sum_prime_3() {
    assert_eq!(sum_primes::sum(3), Some(5));
}

#[test]
fn test_sum_prime_100() {
    assert_eq!(sum_primes::sum(100), Some(1060));
}

#[test]
fn test_sum_prime_1000() {
    assert_eq!(sum_primes::sum(1000), Some(76127));
}
