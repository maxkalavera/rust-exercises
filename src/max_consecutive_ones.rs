
/*
    Given a binary array nums, return the maximum number of consecutive 1's in the array.
*/

pub struct  Solution {}

impl Solution {

    fn iterative (
        nums: Vec<i32>
    ) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut sum = nums[0];
        let mut max: i32 = 0;

        for i in 1..nums.len() {
            if nums[i] == 1 {
                sum = sum + 1;
            } else {
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        };

        if sum > max {  sum } else { max }
    }

    fn recursive (
        slice: &[i32]
    ) -> (bool, i32) // (has_ones, max)
    {
        println!("Current: {:?}", slice);
        let len = slice.len();
        if len > 1 {
            let mid = (0 as usize).midpoint(len);
            let (left, right) = slice.split_at(mid);
            let left_res = Solution::recursive(left);
            let right_res = Solution::recursive(right);

            if left_res.0 && right_res.0 {
                (true, left_res.1 + right_res.1)
            } else {
                (false, left_res.1.max(right_res.1))
            }
        } else {
            if slice[0] == 1 {
                (true, 1)
            } else {
                (false, 0)
            }
        }

    }

    pub fn find_max_consecutive_ones (
        nums: Vec<i32>
    ) -> i32 {
        let res = Solution::recursive(&nums);
        println!("Res: {:?}", res);
        0
        // Solution::iterative(nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn max_consecutive_ones_1 () {
        let nums = vec![1,1,0,1,1,1];
        let res = Solution::find_max_consecutive_ones(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn max_consecutive_ones_2 () {
        let nums = vec![1,0,1,1,0,1];
        let res = Solution::find_max_consecutive_ones(nums);
        assert_eq!(res, 2);
    }

}