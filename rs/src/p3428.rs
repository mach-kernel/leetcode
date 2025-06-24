/*
- given int[], k, where k = subsequence len
- int[] all positive
- return sum max and min elements of _all_ subsequences with at least k elements

for [1,2,3] k = 2

sum of all subsequences with k = 1 (12) ([1] + [2] + [3])
sum ....                     k = 2 (12) ([1, 2] + [1, 3] + [2, 3])

--
todos:

--
get all subsequences for sz k:
[1, 2, 3]
 ^ take n, weave, then recurse

[1] + [1,2] + [1, 3] + f([2,3])
                       [2] + [2,3] + f([3])
                                       [3]
-- collect -- [3], [2,3], [2], [1,3], [1,2], [1]

----

[1,2,3] + [2,3] + [1,3] + [1,2]

[1] -> [], [1]
*/

pub fn subsequences(nums: Vec<i32>, i: usize) -> Vec<Vec<i32>> {
    if i == nums.len() { return vec![nums]; }
    let mut out: Vec<Vec<i32>> = vec![];

    if let Some(n) = nums.get(i) {
        for j in i+1..nums.len() {
            out.push(vec![*n, nums[j]]);
        }

        out.extend(subsequences(nums.clone(), i+1));
        out.push(vec![*n]);
    }

    out
}

pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
    println!("subs {:?}", subsequences(nums.clone(), 0));

    subsequences(nums.clone(), 0)
        .iter()
        .filter(|s| s.len() <= k as usize)
        .map(|subs| {
            let (min, max) = (subs.iter().min(), subs.iter().max());
            min.unwrap_or(&0) + max.unwrap_or(&0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::p3428::min_max_sums;

    #[test]
    fn test_min_max_sums() {
        assert_eq!(min_max_sums(vec![1,2,3], 2), 24);
        assert_eq!(min_max_sums(vec![1,1,1], 2), 12);
        assert_eq!(min_max_sums(vec![5,0,6], 1), 22);

        // 1st fail -- forgot to include the original sequence
        // (a list is always a subsequence of itself)
        assert_eq!(min_max_sums(vec![0,1,1], 3), 9);

        // 2nd fail via fixing 1st
        assert_eq!(min_max_sums(vec![310], 1), 620);

        // 3rd fail...
        assert_eq!(min_max_sums(vec![0,0,0,2], 3), 16);
    }
}