pub mod datastructure;
use datastructure::*;

pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for index in 0..len {
            for i in index + 1..len {
                if nums[i] == nums[index] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::datastructure;
    use super::Solution;
    #[test]
    fn code1512() {
        // 1512. 好数对的数目
        // 给你一个整数数组 nums 。
        // 如果一组数字 (i,j) 满足 nums[i] == nums[j] 且 i < j ，
        // 就可以认为这是一组 好数对 。
        // 返回好数对的数目。

        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}