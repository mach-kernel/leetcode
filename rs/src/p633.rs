/*
- [input] c is pos :: 0 <= c <= i32 max
- a^2 + b^2 = c^2
- a != b

---

c = 5
1^2 + 2^2 = 1 + 4 = 5

1 2 3 4 5 ->

*/

pub fn judge_square_sum(c: i32) -> bool {
    let mut low = 0;
    let mut hi = c.isqrt();

    while low <= hi {
        let s = low.saturating_pow(2).saturating_add(
            hi.saturating_pow(2)
        );

        if (s < c) {
            low += 1;
        } else if (s > c) {
            hi -= 1;
        } else {
            return true
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::p633::judge_square_sum;

    #[test]
    fn test_judge_square_sum() {
        assert_eq!(judge_square_sum(5), true);
        assert_eq!(judge_square_sum(3), false);

        // 2nd submit: apparently a can eq b
        assert_eq!(judge_square_sum(2), true);

        // 3rd submit: saturating ops + overflows
        assert_eq!(judge_square_sum(2147483600), true);
    }
}
