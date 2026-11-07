const ONE_TO_TEN_STR: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (start_bottles - (take_down-1)..=start_bottles)
        .rev()
        .map(|number| {
            let text1 = ONE_TO_TEN_STR[(number as usize) - 1];
            let mut plural = "s";
            let mut text2 = format!("\n");
            let first_letter_capitalized=capitalize_first(text1);
            if number != start_bottles {
                text2 = format!("\n\n");
            }
            let text3;
            if number == 1 {
                plural="";
                text3 = "no green bottles".to_string();
            } else {
                let text4 = ONE_TO_TEN_STR[(number as usize) - 2];
                if number==2 {
                    text3 = format!("{text4} green bottle");
                } else {
                    text3 = format!("{text4} green bottles");
                }
            }
            format!("{text2}{first_letter_capitalized} green bottle{plural} hanging on the wall,\n{first_letter_capitalized} green bottle{plural} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {text3} hanging on the wall."
            )
        })
        .collect()
}
