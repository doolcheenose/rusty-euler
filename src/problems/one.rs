/*If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.*/

pub fn euler_1(ceiling: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut num: i32 = 3;
    loop {
        if num >= ceiling {
            break;
        }
        if num % 3 == 0 {
            sum += num;
        } else if num % 5 == 0 {
            sum += num;
        }
        num += 1;
    }
    return sum;
}
