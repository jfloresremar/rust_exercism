pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let mut number = n;
    let mut primes = (2_u64..).filter(|num| is_prime(*num));
    let mut saved_factors = Vec::new();

    while let Some(p) = primes.next() {
        while number % p == 0 {
            saved_factors.push(p);
            number /= p;
        }
        if number == 1 || p > number / p {
            if number > 1 {
                saved_factors.push(number);
            }
            break;
        }
    }

    saved_factors
}

fn is_prime(n: u64) -> bool {
    let limit = (n as f64).sqrt() as u64;
    (2..=limit).all(|x| n % x != 0)
}
