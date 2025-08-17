use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut anagrams: HashSet<&str> = HashSet::new();
    let mut chars_palabra: Vec<char> = word.to_lowercase().chars().collect();
    chars_palabra.sort();
    for candidato in possible_anagrams {
        if word.len() == candidato.len() && word.to_lowercase() != *candidato.to_lowercase() {
            let candidato_clone = candidato.to_lowercase();
            let mut chars_candidato: Vec<char> = candidato_clone.chars().collect();
            chars_candidato.sort();
            if chars_palabra == chars_candidato {
                anagrams.insert(candidato);
            }
        }
    }
    anagrams
}
