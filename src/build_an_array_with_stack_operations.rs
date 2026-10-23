/*
build_an_array_with_stack_operations

You are given an integer array target and an integer n.

You have an empty stack with the two following operations:

"Push": pushes an integer to the top of the stack.
"Pop": removes the integer on the top of the stack.
You also have a stream of the integers in the range [1, n].

Use the two stack operations to make the numbers in the stack (from the bottom to the top) equal to target. You should follow the following rules:

If the stream of the integers is not empty, pick the next integer from the stream and push it to the top of the stack.
If the stack is not empty, pop the integer at the top of the stack.
If, at any moment, the elements in the stack (from the bottom to the top) are equal to target, do not read new integers from the stream and do not do more operations on the stack.
Return the stack operations needed to build target following the mentioned rules. If there are multiple valid answers, return any of them.
*/

pub struct  Solution {}

impl Solution {
    pub fn build_array(
        target: Vec<i32>, 
        n: i32
    ) -> Vec<String> {
        let len = target.len();
        let mut operations = Vec::<String>::with_capacity(2 * (n as usize));
        let mut index = 0;
        
        for stream_current in 1..=n {
            if index >= len {
                break;
            }

            if target[index] == stream_current {
                index += 1;
                operations.push(String::from("Push"));
            } else {
                operations = [
                    operations.clone(),
                    vec![
                        String::from("Push"),
                        String::from("Pop")
                    ]
                ].concat();
            }
        }

        println!("input: {:?}", target);
        println!("operations: {:?}", operations);

        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_an_array_with_stack_operations_1 () {
        let nums = vec![1,3];
        let n = 3;
        let res = Solution::build_array(nums, n);
        assert_eq!(res, vec!["Push","Push","Pop","Push"]);
    }

    #[test]
    fn build_an_array_with_stack_operations_2 () {
        let nums = vec![1,2,3];
        let n = 3;
        let res = Solution::build_array(nums, n);
        assert_eq!(res, vec!["Push","Push","Push"]);
    }

    #[test]
    fn build_an_array_with_stack_operations_3 () {
        let nums = vec![1,2];
        let n = 4;
        let res = Solution::build_array(nums, n);
        assert_eq!(res, vec!["Push","Push"]);
    }    
}