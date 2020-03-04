# Python program to print all primes smaller than or equal to
# n using Sieve of Eratosthenes

import argparse


def SieveOfEratosthenes(n):
    result = []
    # Create a boolean array "prime[0..n]" and initialize
    # all entries it as true. A value in prime[i] will
    # finally be false if i is Not a prime, else true.
    prime = [True for i in range(n + 1)]
    p = 2
    while p * p <= n:

        # If prime[p] is not changed, then it is a prime
        if prime[p] is True:

            # Update all multiples of p
            for i in range(p * p, n + 1, p):
                prime[i] = False
        p += 1

    # Create a list with all prime numbers
    for p in range(2, n):
        if prime[p]:
            result.append(p)

    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("number")
    args = parser.parse_args()
    n = int(args.number)
    prime_numbers = SieveOfEratosthenes(n)
    print(prime_numbers)
    print("Sum: {}".format(sum(prime_numbers)))


if __name__ == "__main__":
    main()
