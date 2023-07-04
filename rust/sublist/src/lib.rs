#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 { return true; }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) { 
            return true; 
        }
        haystack = &haystack[1..];
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    
    let first_contains_second = is_sub(_first_list, _second_list);
    let second_contains_first = is_sub(_second_list, _first_list);
    
    if first_contains_second && second_contains_first{
        Comparison::Equal
    }else if first_contains_second{
        Comparison::Superlist
    }else if second_contains_first{
        Comparison::Sublist
    }else{
        Comparison::Unequal
    }
}
