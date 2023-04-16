/**
 * Conjecture: Take any number, reverse its digits and add it to the original
 * number. Repeat this process and you will eventually produce a palindrome.
 * This is an open problem. 196 is the smallest number for which a palindrome
 * hasn't been found through this iterative process.
 *
 * https://twitter.com/fermatslibrary/status/1647586644151750656?s=20
 */


use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use num::pow::pow;

fn main() {
    let mut num = BigUint::from(196u32);
    let mut result: BigUint;
    while &num != &reverse_number(&num) {
        result = &num + &reverse_number(&num);
        num = result;
    }
    println!("{num}");
    // println!("{}", reverse_number(BigUint::from(1234u32)));
}

/// Reverses the digits of a given number and returns the result.
fn reverse_number(num: &BigUint) -> BigUint {
    let mut num = num.clone();
    let mut digits: Vec<u32> = vec![];
    let mut result: BigUint = Zero::zero();

    // Extract the digits of the number and store them in a vector
    while num > Zero::zero() {
        let digit: u32 = (&num % BigUint::from(10u32)).to_u32().unwrap();
        num /= BigUint::from(10u32);
        digits.push(digit);
    }

    // Reconstruct the number with the digits in reverse order
    let mut power: usize = One::one();
    for digit in &digits {
        let true_power: usize = &digits.len() - power;
        result += digit * pow(10, true_power);
        power += 1;
    }

    result
}
