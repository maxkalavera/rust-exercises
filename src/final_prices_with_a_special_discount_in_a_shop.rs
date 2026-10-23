/*
name: final_prices_with_a_special_discount_in_a_shop

description: You are given an integer array prices where prices[i] is the price of the ith item in a shop.

There is a special discount for items in the shop. If you buy the ith item, then you will receive a discount equivalent to prices[j] where j is the minimum index such that j > i and prices[j] <= prices[i]. Otherwise, you will not receive any discount at all.

Return an integer array answer where answer[i] is the final price you will pay for the ith item of the shop, considering the special discount.
*/

pub struct  Solution {}

impl Solution {
    pub fn final_prices(
        prices: Vec<i32>
    ) -> Vec<i32> {

        prices
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let found = prices
                .iter()
                .skip(index + 1)
                .find(|target| **target <= *value);
            
            if found.is_some() {
                value.clone() - found.cloned().unwrap()
            } else {
                value.clone()
            }

        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn final_prices_with_a_special_discount_in_a_shop_1 () {
        let res = Solution::final_prices(
            vec![8,4,6,2,3]
        );
        assert_eq!(res, vec![4,2,4,2,3]);
    }

    #[test]
    fn final_prices_with_a_special_discount_in_a_shop_2 () {
        let res = Solution::final_prices(
            vec![1,2,3,4,5]
        );
        assert_eq!(res, vec![1,2,3,4,5]);
    }

    #[test]
    fn final_prices_with_a_special_discount_in_a_shop_3 () {
        let res = Solution::final_prices(
            vec![10,1,1,6]
        );
        assert_eq!(res, vec![9,0,1,6]);
    }        
}