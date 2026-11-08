pub fn square(s: u32) -> u64 {
    //todo!("grains of rice on square {s}");
    if s < 1 || s > 64 {
        panic!()
    }
    (1..=s).fold(1, |acc, x| if x == 1 { 1 } else { acc * 2 })
}

pub fn total() -> u64 {
    (1..=64).map(|number| square(number)).sum()
}
