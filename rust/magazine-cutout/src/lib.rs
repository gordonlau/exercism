// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in magazine{
        *map.entry(word).or_insert(0) += 1;
    }
    for word in note{
        let frequency = map.entry(word).or_insert(0);
        if *frequency == 0{
            return false;
        }else{
            *frequency -= 1;
        }
    }
    true
}
