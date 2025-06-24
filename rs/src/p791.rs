/*
Input:  order = "cba", s = "abcd"
Output:  "cbad"

Explanation: "a", "b", "c" appear in order, so the order of "a", "b", "c" should be "c", "b", and "a".
Since "d" does not appear in order, it can be at any position in the returned string. "dcba", "cdba", "cbda" are also valid outputs.
*/
use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut lut: HashMap<char, i32> = HashMap::new();
    for (i, c) in order.chars().into_iter().enumerate() {
        lut.insert(c, i as i32);
    }

    let mut sorted: Vec<char> = s.chars().into_iter().collect();
    sorted.sort_by_key(|k| lut.get(k).unwrap_or(&-1));
    let cs: Vec<String> = sorted.iter().map(|k| k.to_string()).collect();

    cs.join("")
}

#[cfg(test)]
mod test {
    use crate::p791::custom_sort_string;

    #[test]
    fn test_custom_sort_string() {
        assert_eq!(custom_sort_string("bcafg".to_string(), "abcd".to_string()), "dbca".to_string());
    }
}