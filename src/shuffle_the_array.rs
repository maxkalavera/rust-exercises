/*
    Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
    Return the array in the form [x1,y1,x2,y2,...,xn,yn].
*/

pub struct  Solution {}

impl Solution {
    pub fn shuffle(
        nums: Vec<i32>, 
        n: i32
    ) -> Vec<i32> {
        let n = n as usize;
        (0..nums.len())
            .map(|index| 
                if index % 2 == 0 {
                    nums[index / 2]
                } else {
                    nums[(index / 2) + n]
                }
            )
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shuffle_the_array_1 () {
        let nums = vec![2,5,1,3,4,7];
        let n = 3;
        let res = Solution::shuffle(nums, n);
        assert_eq!(res, vec![2,3,5,4,1,7]);
    }

    #[test]
    fn shuffle_the_array_2 () {
        let nums = vec![1,2,3,4,4,3,2,1];
        let n = 4;
        let res = Solution::shuffle(nums, n);
        assert_eq!(res, vec![1,4,2,3,3,2,4,1]);
    }

    #[test]
    fn shuffle_the_array_3 () {
        let nums = vec![1,1,2,2];
        let n = 2;
        let res = Solution::shuffle(nums, n);
        assert_eq!(res, vec![1,2,1,2]);
    }
}