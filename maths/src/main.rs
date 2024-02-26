use crate::prime::prime_factors;
use crate::prime::Prime;

mod prime;

fn main() {
    println!("Hello, world!");
    prime_factors(3);

    let tab = vec![8, 12, 20];
    println!("lcm of {:?} : {} ", &tab, lcm(&tab));
    println!("gcf of {:?} : {:?} ", &tab, gcf(&tab));
}

// ----------------------------------------------------------------------------
// -------------------------------------------------------------------- GCF ---

/// Return the Greatest Common Factor value with the prime factorization method.
fn gcf(numbers: &Vec<u32>) -> Result<u32, u32> {
    if numbers.is_empty() || numbers.iter().any(|&nb| nb == 0) {
        Err(0)
    } else {
        // Primes factors ?
        let factors: Vec<Vec<Prime>> = numbers.iter().map(|&nb| prime_factors(nb)).collect();

        // Find those which are common
        let mut conserves: Vec<Prime> = Vec::new();
        let mut current_prime: Prime;

        for factor_group in factors.iter() {
            // Loop in all lists
            'hop: for prime in factor_group.iter() {
                // Check if it has already been done
                if !conserves.iter().any(|p| p.value == prime.value) {
                    current_prime = *prime;

                    // Check if it exists in all tabs
                    for fg in factors.iter() {
                        if let Some(aa) = fg.iter().find(|p| p.value == prime.value) {
                            if aa.exponent < current_prime.exponent {
                                current_prime.exponent = aa.exponent;
                            }
                        } else {
                            continue 'hop;
                        }
                    }

                    // If so, keep the smallest exponent
                    conserves.push(current_prime);
                }
            }
        }

        // Multiply kept values together
        Ok(multiply_value(&conserves))
    }
}

// ----------------------------------------------------------------------------
// -------------------------------------------------------------------- LCM ---

/// Return the Least Common Multiple value with the prime factorization method.
fn lcm(numbers: &[u32]) -> u32 {
    // Primes factors ?
    let factors: Vec<Vec<Prime>> = numbers.iter().map(|&nb| prime_factors(nb)).collect();

    // Keep the largest time a prime appears
    let mut conserves: Vec<Prime> = Vec::new();

    for factor_group in factors.iter() {
        'here: for prime in factor_group.iter() {
            // lcm_update_choice(&mut conserves, prime);

            for p in conserves.iter_mut() {
                if p.value == prime.value {
                    if p.exponent < prime.exponent {
                        p.exponent = prime.exponent;
                    }
                    continue 'here;
                }
            }

            conserves.push(*prime);
        }
    }

    // Multiply kept values together
    multiply_value(&conserves)
}

fn multiply_value(tab: &[Prime]) -> u32 {
    let mut total: u32 = 1;
    for p in tab
        .iter()
        .map(|p| p.value.pow(p.exponent))
        .collect::<Vec<u32>>()
        .iter()
    {
        total *= p;
    }

    total
}

// ----------------------------------------------------------------------------
// ------------------------------------------------------------------ TESTS ---
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lcm_ok() {
        assert_eq!(lcm(&vec![0, 5]), 0);
        assert_eq!(lcm(&vec![5]), 5);
        assert_eq!(lcm(&vec![20, 75]), 300);
        assert_eq!(lcm(&vec![12, 15, 75]), 300);
        assert_eq!(lcm(&vec![1265, 1587, 7565]), 132_062_205);
        assert_eq!(lcm(&vec![10, 5, 67, 1548, 3, 568, 47]), 3_461_002_920);
    }

    #[test]
    fn gcm_ok() {
        assert_eq!(gcf(&vec![]), Err(0));
        assert_eq!(gcf(&vec![0, 5]), Err(0));
        assert_eq!(gcf(&vec![65, 5, 0]), Err(0));
        assert_eq!(gcf(&vec![5]), Ok(5));
        assert_eq!(gcf(&vec![20, 75]), Ok(5));
        assert_eq!(gcf(&vec![12, 15, 75]), Ok(3));
        assert_eq!(gcf(&vec![330, 75, 450, 225]), Ok(15));
        assert_eq!(gcf(&vec![10, 5, 67, 1548, 3, 568, 47]), Ok(1));
    }
}
