// 53. 最大子数组和
// sliding-window
pub struct Solution{}

// code start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        nums.iter().fold(0, |acc, x| {
            let res = std::cmp::max(*x, acc+*x);
            max = std::cmp::max(max, res);
            res
        });
        max
    }
}
// code end

#[test]
fn test_max_sub_array() {
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
}