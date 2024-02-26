// ----------------------------------------------------------------------------
// ----------------------------------------------------------------- PRIMES ---

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Prime {
    pub value: u32,
    pub exponent: u32,
}

/// List all prime factors of number.
pub fn prime_factors(number: u32) -> Vec<Prime> {
    let mut primes: Vec<Prime> = Vec::new();

    get_primes(number, &mut primes);

    if primes.is_empty() {
        primes.push(Prime {
            value: number,
            exponent: 1,
        });
    }

    primes
}

/// Find the first divisor recursively and save those which are primes.
fn get_primes(number: u32, primes: &mut Vec<Prime>) {
    for i in 2..(number) {
        if number % i == 0 {
            if is_prime(i) {
                add_prime(primes, i);
            } else {
                get_primes(i, primes);
            }

            if is_prime(number / i) {
                add_prime(primes, number / i);
            } else {
                get_primes(number / i, primes);
            }

            break;
        }
    }
}

/// Up the prime exponent if exist in the vec or add the prime.
fn add_prime(primes: &mut Vec<Prime>, new_prime: u32) {
    for p in primes.iter_mut() {
        if p.value == new_prime {
            p.exponent += 1;
            return;
        }
    }

    primes.push(Prime {
        value: new_prime,
        exponent: 1,
    });
}

/// Is number a prime ?
fn is_prime(number: u32) -> bool {
    if number > 1 {
        for i in 2..=((number as f32).sqrt() as u32) {
            if number % i == 0 {
                return false;
            }
        }
        return true;
    }
    false
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ TESTS ---
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_prime_ok() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(8011), true);
        assert_eq!(is_prime(999999929), true);
    }

    #[test]
    fn is_prime_nok() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(289), false);
        assert_eq!(is_prime(8012), false);
        assert_eq!(is_prime(999999999), false);
    }

    #[test]
    fn list_prime_factors() {
        assert_eq!(
            prime_factors(3),
            vec![Prime {
                value: 3,
                exponent: 1
            }]
        );
        assert_eq!(
            prime_factors(8),
            vec![Prime {
                value: 2,
                exponent: 3
            }]
        );
        assert_eq!(
            prime_factors(20),
            vec![
                Prime {
                    value: 2,
                    exponent: 2
                },
                Prime {
                    value: 5,
                    exponent: 1
                },
            ]
        );
        assert_eq!(
            prime_factors(24568),
            vec![
                Prime {
                    value: 2,
                    exponent: 3
                },
                Prime {
                    value: 37,
                    exponent: 1
                },
                Prime {
                    value: 83,
                    exponent: 1
                },
            ]
        );
        assert_eq!(
            prime_factors(68549888),
            vec![
                Prime {
                    value: 2,
                    exponent: 8
                },
                Prime {
                    value: 11,
                    exponent: 2
                },
                Prime {
                    value: 2213,
                    exponent: 1
                },
            ]
        );
        assert_eq!(
            prime_factors(115455),
            vec![
                Prime {
                    value: 3,
                    exponent: 1
                },
                Prime {
                    value: 5,
                    exponent: 1
                },
                Prime {
                    value: 43,
                    exponent: 1
                },
                Prime {
                    value: 179,
                    exponent: 1
                },
            ]
        );
        // assert_eq!(
        //     prime_factors(999999929),
        //     vec![Prime {
        //         value: 999999929,
        //         exponent: 1
        //     },]
        // );
    }
}
