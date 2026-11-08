fn is_prime(n: u32) -> bool {
    // primer intento toooo sloooow
    // (2..n).filter(|divisor| n % divisor == 0).count() == 0

    // segundo intento thanks to Greenwqtch's solution
    let limit = (n as f64).sqrt() as u32;
    (2..=limit).all(|x| n % x != 0)
}
pub fn nth(n: u32) -> u32 {
    // primer intento
    // let mut nth = 0;
    // let mut i = 2;
    // while nth < n {
    //     i += 1;
    //     if is_prime(i) {
    //         nth += 1;
    //     }
    // }
    // i

    // segundo intento
    (2..).filter(|num| is_prime(*num)).nth(n as usize).unwrap()
}
