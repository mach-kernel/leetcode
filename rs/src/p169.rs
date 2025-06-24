/*
nums = [2,4,5,5,5,5,5,6,6], target = 5
 */
pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    let threshold = nums.len() / 2;
    let mut count: usize = 0;

    for num in nums {
        if (num == target) {
            count += 1;
        }
    }

    count > threshold
}

#[cfg(test)]
mod test {
    use crate::p169::is_majority_element;

    #[test]
    fn test_is_majority_element() {
        assert_eq!(is_majority_element(vec![2,4,5,5,5,5,5,6,6], 5), true);
        assert_eq!(is_majority_element(vec![10,100,101,101], 101), false);

    }
}