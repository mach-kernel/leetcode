use std::cmp::max;
use std::collections::HashSet;
// i32 = enum of fruit type
// 2 baskets to hold u/l fruit types

pub fn pick_fruit(fruits: &Vec<i32>, start: usize) -> i32 {
    let basket_a = fruits[start];
    let mut basket_b = i32::MIN;
    let mut picked = 0;

    for i in start..fruits.len() {
        let fruit = fruits[i];
        if fruit == basket_a || fruit == basket_b {
            picked += 1;
        }
        else if basket_b == i32::MIN {
            basket_b = fruit;
            picked += 1;
        }
        else {
            break;
        }
    }

    picked
}

/*
2 pointer style with hashset to enforce window constraint

[1,0,1,4,1,4,1,2,3]
 a b
   b
     b -> !
   a b
 */
pub fn pick_fruit_slide(fruits: Vec<i32>) -> i32 {
    if fruits.len() < 2 { return fruits.len() as i32; }

    let mut max_picked = i32::MIN;
    let (mut a, mut b) = (0, 0);

    for i in 1..fruits.len() {
        if fruits[i] != fruits[a] {
            b = i;
            break;
        }
    }

    let mut picked = (b - a) as i32;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(fruits[a]);
    seen.insert(fruits[b]);

    let mut i = b;

    while i < fruits.len() {
        if (seen.contains(&fruits[i])) {
            picked += 1;
            i += 1;
        } else {
            a = b;
            seen.clear();

            for j in a..fruits.len() {
                if fruits[a] != fruits[j] {
                    b = j;
                    break;
                }
            }

            seen.insert(fruits[a]);
            seen.insert(fruits[b]);
            max_picked = max(max_picked, picked);
            picked = (b - a) as i32;
            i = b;
        }
    }

    max(max_picked, picked)
}

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut picked = 0;

    for i in 0..fruits.len() {
        let f = fruits[i];
        if (i >= 1 && f == fruits[i-1]) {
            continue;
        }

        picked = max(picked, pick_fruit(&fruits, i))
    }

    picked
}

#[cfg(test)]
mod tests {
    use crate::p904::{pick_fruit_slide, total_fruit};

    #[test]
    fn test_total_fruit() {
        assert_eq!(total_fruit(vec![1,2,3,2,2]), 4);
        assert_eq!(pick_fruit_slide(vec![1,0,1,4,1,4,1,2,3]), 5);
        assert_eq!(pick_fruit_slide(vec![0,1,2,2]), 3);
        assert_eq!(pick_fruit_slide(vec![1,2,3,2,2]), 4);
        assert_eq!(pick_fruit_slide(vec![0,0,1,1]), 4);
        assert_eq!(pick_fruit_slide(vec![0,1,6,6,4,4,6]), 5);
        assert_eq!(pick_fruit_slide(vec![3,3,3,1,2,1,1,2,3,3,4]), 5);
    }
}