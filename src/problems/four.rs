/* A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers. */

pub fn sol() -> i64 {
    pal_product()
}

pub fn pal_product() -> i64 {
    let mut largest: i64 = 0;
    for d1 in 100..999 {
        for d2 in d1..999 {
            let prod = d1 * d2;
            if is_pal(&prod) && prod > largest {
                largest = prod;
            }
        }
    }
    return largest;
}

pub fn is_pal(num: &i64) -> bool {
    let str_num = num.to_string();
    let half = str_num.len() / 2;

    str_num.bytes().take(half).eq(str_num.bytes().rev().take(half))
}
