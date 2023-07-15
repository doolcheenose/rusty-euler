/*The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?*/

// let pub input: i64 = 600851475143;

pub fn sol(input: i64) -> i64 {
    return largest_prime_factor(input);
}

pub fn largest_prime_factor(num: i64) -> i64 {
    let prime_factors: Vec<i64> = find_prime_factors(num);
    let first: Option<i64> = prime_factors.last().copied();
    match first {
        Some(first) => first,
        None => 1 as i64,
    }
}

pub fn find_prime_factors(mut num: i64) -> Vec<i64> {
    let sq_num: i64 = (num as f64).sqrt() as i64 + 1;
    let mut q: i64 = 2;
    let mut prime_factors: Vec<i64> = Vec::new();

    loop {
        if q >= sq_num || q > num {
            break;
        }
        if num % q == 0 {
            prime_factors.push(q);
            while num % q == 0 {
                num = num / q;
            }
        }
        q += 1;
    }

    if prime_factors.len() == 0 {
        prime_factors.push(num);
    }

    return prime_factors;
}
