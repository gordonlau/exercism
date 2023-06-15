use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercased_target = word.to_lowercase();
    let target_map = create_chars_map(&lowercased_target);
    let mut anagrams = HashSet::new();

    for word in possible_anagrams{
        let lowercased_word = &word.to_lowercase();
        let char_map = create_chars_map(lowercased_word);
        
        if char_map == target_map && lowercased_word != &lowercased_target{
            anagrams.insert(*word);
        }

    }
    anagrams
}


fn create_chars_map(word: &str) -> HashMap<char, i32>{
    let mut chars_map = HashMap::new();
    let chars: Vec<char> = word.chars().collect();

    for char in chars{
        *chars_map.entry(char).or_insert(0) += 1;
    }
    chars_map
}