/*
    [-4,-1,0,3,10] -> [16,1,0,9,100] -> [0,1,9,16,100]
 */

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let (mut l, mut r): (i32, i32) = (0, nums.len() as i32 - 1);

    while (l <= r) {
        let (nl, nr) = (nums[l as usize].pow(2), nums[r as usize].pow(2));

        if (nl > nr) {
            out.push(nl);
            l += 1;
        } else {
            out.push(nr);
            r -= 1;
        }

    }

    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use crate::p977::sorted_squares;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
        assert_eq!(sorted_squares(vec![-1]), vec![1]);

    }
}