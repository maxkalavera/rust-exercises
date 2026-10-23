/*
You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
You are given an integer array nums representing the data status of this set after the error.
Find the number that occurs twice and the number that is missing and return them in the form of an array.
*/

use std::collections::HashSet;

pub struct  Solution {}

impl Solution {
    pub fn find_error_nums(
        nums: Vec<i32>
    ) -> Vec<i32> {
        let mut repeated_set = HashSet::<i32>::new();
        let mut missing_set = (1..=(nums.len()))
            .map(|i| i as i32)
            .collect::<HashSet<i32>>();
        let mut repeated: Option<i32> = None;

        for num in nums.clone().into_iter() {
            if repeated_set.contains(&num) {
                repeated = Some(num.clone());
            } else {
                repeated_set.insert(num);
            }
            missing_set.remove(&num);
        }

        let repeated = repeated.unwrap();
        let missing = missing_set.into_iter().next().unwrap();

        vec![repeated, missing]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_mismatch_1 () {
        let nums = vec![1,2,2,4];
        let res = Solution::find_error_nums(nums);
        assert_eq!(res, vec![2,3]);
    }

    #[test]
    fn set_mismatch_2 () {
        let nums = vec![1,1];
        let res = Solution::find_error_nums(nums);
        assert_eq!(res, vec![1,2]);
    }

    #[test]
    fn set_mismatch_3 () {
        let nums = vec![2,2];
        let res = Solution::find_error_nums(nums);
        assert_eq!(res, vec![2,1]);
    }

    #[test]
    fn set_mismatch_4 () {
        let nums = vec![3,2,2];
        let res = Solution::find_error_nums(nums);
        assert_eq!(res, vec![2,1]);
    }

    #[test]
    fn set_mismatch_5 () {
        let nums = vec![3,2,3,4,6,5];
        let res = Solution::find_error_nums(nums);
        assert_eq!(res, vec![3,1]);
    }    

}