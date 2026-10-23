/*
find_all_numbers_disappeared_in_an_array

Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.

*/

use std::collections::HashSet;

pub struct  Solution {}

impl Solution {
    pub fn find_disappeared_numbers(
        nums: Vec<i32>
    ) -> Vec<i32> {
        let mut set = (1..(nums.len() + 1))
            .map(|i| i as i32)
            .collect::<HashSet<i32>>();

        for num in nums.clone() {
            set.remove(&num);
        }

        println!("input: {:?}", nums);
        println!("set: {:?}", set);

        set.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_all_numbers_disappeared_in_an_array_1 () {
        let nums = vec![4,3,2,7,8,2,3,1];
        let res = Solution::find_disappeared_numbers(nums);
        assert_eq!(res, vec![5,6]);
    }

    #[test]
    fn find_all_numbers_disappeared_in_an_array_2 () {
        let nums = vec![1,1];
        let res = Solution::find_disappeared_numbers(nums);
        assert_eq!(res, vec![2]);
    }
}