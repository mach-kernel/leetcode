/*
requirements:
- strings are similar if at most swap 2 letters
- strings are in group if similar to at least one word in group

For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".


['arts', 'rats', 'star', 'tars'] -> {tars, rats, arts}, {star}

a -> (0, 1, 2, 1) -> rats, tars
r -> (1, 0, 3, 2) -> ??
t -> (2, 2, 1, 0) -> arts, rats
s -> (3, 3, 0, 3) -> tars


*/
use std::collections::{HashMap, HashSet};

pub fn frequencies(nums: Vec<usize>) -> HashMap<usize, usize> {
    let mut fs = HashMap::new();

    for n in nums {
        if let Some(f) = fs.get(&n) {
            fs.insert(n, f+1);
        } else {
            fs.insert(n, 1);
        }
    }

    fs
}

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    // no string[i] access in rust :(
    let charstrs: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

    // take one string (whole charspace), sort chars
    let mut chars: Vec<char> = strs[0].chars().collect();
    chars.sort();

    // i -> vec[pos of char in each word]
    let mut positions: Vec<Vec<usize>> = vec![vec![0; strs[0].len()]; strs.len()];
    for (ci, c) in chars.iter().enumerate() {
        for i in (0..charstrs.len()) {
            for j in (0..charstrs[0].len()) {
                if charstrs[i][j] == *c {
                    positions[ci][i] = j;
                }
            }
        }
    }

    let mut groups: Vec<HashSet<String>> = positions.iter().map(|pv| {
        let mut fm = frequencies(pv.clone());
        fm.retain(|_, c| *c > 1);

        HashSet::from_iter(
            pv.iter().enumerate().filter_map(|(wi, p)| {
                if (fm.contains_key(p)) {
                    Some(strs[wi].clone())
                } else {
                    None
                }
            })
        )
    }).collect();

    groups.sort_by_key(|g| g.len());
    groups.reverse();

    let mut prune: Vec<usize> = vec![];

    for i in (0..groups.len()) {
        for j in (0..groups.len()) {
            if (i == j) {
                continue;
            }
            if (HashSet::from_iter(groups[i].intersection(&groups[j]).cloned()) == groups[j]) {
                prune.push(j);
            }
        }
    }
    
    prune.dedup();

    println!("strs {:?} cs {:?} -- posi {:?}", strs, chars, positions);
    println!("gs {:?} prune {:?}", groups, prune);

    0
}

#[cfg(test)]
mod test {
    use crate::p839::num_similar_groups;

    #[test]
    fn test_num_similar_groups() {
        assert_eq!(num_similar_groups(vec!["tars".to_string(),"rats".to_string(),"arts".to_string(),"star".to_string()]), 2);
        assert_eq!(num_similar_groups(vec!["omv".to_string(),"ovm".to_string()]), 1);
        assert_eq!(num_similar_groups(vec!["nmiwx".to_string(),"mniwx".to_string(),"wminx".to_string(),"mnixw".to_string(),"xnmwi".to_string()]), 2);
    }
}
