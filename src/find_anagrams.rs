use std::collections::HashMap;
use std::collections::HashSet;



fn factorial(number: usize) -> usize {
    if number == 0 {
        return 1;
    }
    return number * factorial(number - 1);
}

fn compute_possible_anagrams(search_string: String) -> HashSet<String>{
    let possible_anagrams_size = factorial(search_string.len());
    let mut possibilities = Vec::with_capacity(possible_anagrams_size);

    fn permute(chars: &mut Vec<char>, l: usize, r: usize, possibilities: &mut Vec<String>) {
        if l == r {
            possibilities.push(chars.iter().collect());
        } else {
            for i in l..=r {
                chars.swap(l, i);
                permute(chars, l + 1, r, possibilities);
                chars.swap(l, i);
            }
        }
    }

    let mut chars: Vec<char> = search_string.chars().collect();
    permute(&mut chars, 0, search_string.len() - 1, &mut possibilities);
    let final_possibilities: HashSet<String> = possibilities.into_iter().collect();
    println!("Possible anagrams: {:?}", final_possibilities);
    return final_possibilities;
}

fn find_anagrams(input_string: String, search: String) -> HashMap<i32, String> {
    let possible_anagrams = compute_possible_anagrams(search.clone());
    if input_string.len() < search.len() {
        return HashMap::new();
    }
    let mut found_indexes :HashMap<i32, String> = HashMap::new();
    for (index, c) in input_string.chars().enumerate() {
        if index + search.len() <= input_string.len() {
            let mut sub_string = input_string[index..index + search.len()].to_string();
            let mut sub_string_chars: Vec<char> = sub_string.chars().collect();
            sub_string_chars.sort();
            let sorted_sub_string: String = sub_string_chars.iter().collect();
            if possible_anagrams.contains(&sorted_sub_string) {
                found_indexes.insert(index as i32, sub_string);
            }
        } else {
            break;
        }
    }
    return found_indexes;

}

fn main() {
    let input_string: String = "abab".to_string();
    let anagram_search: String = "ab".to_string();
    let anagram_indices = find_anagrams(input_string, anagram_search);
    println!("Anagram indices: {:?}", anagram_indices);


}