/*
how_many_numbers_are_smaller_than_the_current_number

Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it. That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].
Return the answer in an array.

*/

use std::collections::HashMap;

pub struct  Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(
        nums: Vec<i32>
    ) -> Vec<i32> {
        println!("input: {:?}", nums);
        let sorted = {
            let mut v = nums.clone();
            v.sort();
            v
        };
        let mut hist = HashMap::<i32, i32>::with_capacity(nums.len());
        let mut count = 0;
        let mut current = sorted.get(0).unwrap().clone();

        hist.insert(current, 0);
        for num in sorted.iter() {
            if current != *num {
                hist.insert(num.clone(), count);
                current = *num;
            }
            count += 1;
        }

        nums
            .iter()
            .map(|value| 
                hist
                .get(value)
                .unwrap()
                .clone()
            )
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn how_many_numbers_are_smaller_than_the_current_number_1 () {
        let nums = vec![6,5,4,8];
        let res = Solution::smaller_numbers_than_current(nums);
        assert_eq!(res, vec![2,1,0,3]);
    }
    
    #[test]
    // #[ignore]
    fn how_many_numbers_are_smaller_than_the_current_number_2 () {
        let nums = vec![7,7,7,7];
        let res = Solution::smaller_numbers_than_current(nums);
        assert_eq!(res, vec![0,0,0,0]);
    }
}