/* The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million. */

pub fn sol() -> u64 {
    let mut sum: u64 = 10;
    for num in (7..2000000).step_by(2) {
        if is_prime(num) {
            sum += num;
        }
    }
    return sum;
}

pub fn is_prime(num: u64) -> bool {
    if num % 2 == 0 {
        return true;
    }
    let mut q = 3;
    let num_sqrt = (num as f64).sqrt() as u64 + 1;

    loop {
        if q >= num_sqrt || q > num {
            break true;
        }
        if num % q == 0 {
            break false;
        }
        q += 2;
    }
}
