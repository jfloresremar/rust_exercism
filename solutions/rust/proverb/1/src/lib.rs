pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => format!("And all for the want of a {}.", list.first().unwrap()),
        _ => {
            let body = list
                .windows(2)
                .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "{}\nAnd all for the want of a {}.",
                body,
                list.first().unwrap()
            )
        }
    }
}
