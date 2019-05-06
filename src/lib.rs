//! A crate that provides a single function to determine if a number is prime

fn first_factor(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    }

    for n in (3..).step_by(2).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        }
    }

    return x;
}

/// Returns true if the number is prime.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    first_factor(n) == n
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMES: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    #[test]
    fn primes_list() {
        for &prime in PRIMES {
            assert!(is_prime(prime), "Expected {} to be prime.", prime);
        }
    }
}
