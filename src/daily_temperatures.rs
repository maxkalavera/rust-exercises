/*
name: daily_temperatures

description: Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
*/

pub struct  Solution {}

impl Solution {
    pub fn daily_temperatures(
        temperatures: Vec<i32>
    ) -> Vec<i32> {
        let mut stack = Vec::<usize>::with_capacity(temperatures.len());
        let mut result = (0..temperatures.len())
            .map(|_| 0 as i32)
            .collect::<Vec<i32>>();

        for (target_index, target) in temperatures.iter().enumerate() {
            let top = stack.last();
            if  top.is_some() 
                // Current temperature is greather that the one in the top of stack
                && *target > temperatures[*top.unwrap()]
            {
                // Exhaust all stack values whose temperature is less than current value's temp
                loop {
                    let top_index = stack.last().cloned();

                    if  top_index.is_some() 
                        && temperatures[top_index.unwrap()] < *target 
                    {
                        let index = top_index.unwrap();
                        result[index] = (target_index - index) as i32;
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }

            stack.push(target_index);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn daily_temperatures_1 () {
        let res = Solution::daily_temperatures(
            vec![73,74,75,71,69,72,76,73]
        );
        assert_eq!(res, vec![1,1,4,2,1,1,0,0]);
    }

    #[test]
    fn daily_temperatures_2 () {
        let res = Solution::daily_temperatures(
            vec![30,40,50,60]
        );
        assert_eq!(res, vec![1,1,1,0]);
    }

    #[test]
    fn daily_temperatures_3 () {
        let res = Solution::daily_temperatures(
            vec![30,60,90]
        );
        assert_eq!(res, vec![1,1,0]);
    }
}