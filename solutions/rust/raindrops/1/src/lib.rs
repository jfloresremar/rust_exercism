pub fn raindrops(n: u32) -> String {
    let output: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter_map(|(divisor, sound)| (n % divisor == 0).then_some(sound))
        .collect();
    if output.is_empty() {
        n.to_string()
    } else {
        output
    }
}
