use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_cased_word = word.to_lowercase();
    let target_count_map = create_count_map(&lower_cased_word);

    let is_anagram = |candidate: &str| {
        let lower_cased_candidate = candidate.to_lowercase();
        candidate.len() == word.len()
            && lower_cased_word != lower_cased_candidate
            && target_count_map == create_count_map(&lower_cased_candidate)
    };

    possible_anagrams
        .iter()
        .filter(|word| is_anagram(word))
        .map(|&x| x)
        .collect()
}

fn create_count_map(word: &str) -> HashMap<char, i32> {
    let mut count_map: HashMap<char, i32> = HashMap::new();
    for ch in word.chars() {
        let count = count_map.entry(ch).or_insert(0);
        *count += 1;
    }
    count_map
}
