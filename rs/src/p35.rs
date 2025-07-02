/*
[1,3,5,6]
t=2

l = 0, h = 3, m = 2 n = 5
l = 0, h = 2, m = 1

-- t=5

l=0, h=3, m=2, n=5

 */

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    let mut mid = 0;

    while (lo < hi) {
        mid = ((hi - lo) / 2) + lo;
        let n = nums[mid];

        if (n == target) {
            return mid as i32;
        }
        else if (n > target) {
            hi = mid.saturating_sub(1)
        }
        else {
            lo = mid + 1;
        }
    }

    let n = nums[lo];

    // if something is appended to the end, it's end + 1,
    // but assume everything to the right shifts over for insert front
    let res = if (n < target) {
        lo + 1
    } else {
        lo
    };

    res as i32
}

#[cfg(test)]
mod tests {
    use crate::p35::search_insert;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1,3], 0), 0);
        assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
    }
}