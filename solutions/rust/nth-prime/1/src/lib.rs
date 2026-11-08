fn is_prime(n: u32) -> bool {
    (2..n).filter(|divisor| n % divisor == 0).count() == 0
}
pub fn nth(n: u32) -> u32 {
    let mut nth = 0;
    let mut i = 2;
    while nth < n {
        i += 1;
        if is_prime(i) {
            nth += 1;
        }
    }
    i
}
