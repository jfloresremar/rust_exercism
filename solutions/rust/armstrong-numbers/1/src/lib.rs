fn num_digits(n: u32) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn split_digits(mut n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}
pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num_digits(num);
    let split_digits = split_digits(num);
    let total = split_digits.iter().map(|n| n.pow(num_digits)).sum();
    num == total
}
