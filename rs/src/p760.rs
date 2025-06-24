use std::collections::HashMap;

pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let n2i: HashMap<i32, usize> = HashMap::from_iter(nums2.iter().enumerate().map(|(i, n)| (*n, i)));
    nums1.iter().map(|n| n2i[n] as i32).collect()
}

#[cfg(test)]
mod tests {
    use crate::p760::anagram_mappings;

    #[test]
    fn test_anagram_mappings() {
        assert_eq!(anagram_mappings(vec![12,28,46,32,50], vec![50,12,32,46,28]), vec![1,4,3,2,0]);
        assert_eq!(anagram_mappings(vec![84,46], vec![84,46]), vec![0,1]);
    }
}
