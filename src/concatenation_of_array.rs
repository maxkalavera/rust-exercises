
/*
Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
Specifically, ans is the concatenation of two nums arrays.
Return the array ans. 
*/

pub struct  Solution {}

impl Solution {
    pub fn get_concatenation(
        nums: Vec<i32>
    ) -> Vec<i32> {
        let frame = nums.len();
        (0..(frame * 2))
            .enumerate()
            .map(|(index, _)| nums[index % frame])
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn concatenation_of_array_1 () {
        let nums = vec![1,2,1];
        let res = Solution::get_concatenation(nums);
        assert_eq!(res, vec![1,2,1,1,2,1]);
    }

    #[test]
    fn concatenation_of_array_2 () {
        let nums = vec![1,3,2,1];
        let res = Solution::get_concatenation(nums);
        assert_eq!(res, vec![1,3,2,1,1,3,2,1]);
    }
}