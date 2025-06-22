use std::cmp::max;

/*
[4,1,5,1,2,5,1,5,5,4]
[1,1,1,2,4,4,5,5,5,5]
 ^                 ^ <-- find the min sum for each largest number
   ^             ^   <-- slide inward assuming other pairs are 'taken'
                         (i.e, no n^2 scan)
 */

pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.clone();
    sorted.sort();

    let mut max_sum = i32::MIN;
    let (mut l, mut r) = (0, sorted.len() - 1);

    while (l < r) {
        max_sum = max(max_sum, sorted[l] + sorted[r]);
        l += 1;
        r -= 1;
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use crate::p1877::min_pair_sum;

    #[test]
    fn test_min_pair_sum() {
        assert_eq!(min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(min_pair_sum(vec![3,5,4,2,4,6]), 8);
        assert_eq!(min_pair_sum(vec![4,1,5,1,2,5,1,5,5,4]), 8);
    }
}
