//! Determine if 999983 is prime

use primes_testing_workshop as primes;

#[test]
fn is_prime_999983() {
    assert!(primes::is_prime(999983));
}

#[test]
fn is_prime_1000999983() {
    assert!(!primes::is_prime(1000999983));
}
