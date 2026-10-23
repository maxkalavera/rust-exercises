/*
name: evaluate_reverse_polish_notation

description: You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:

The valid operators are '+', '-', '*', and '/'.
Each operand may be an integer or another expression.
The division between two integers always truncates toward zero.
There will not be any division by zero.
The input represents a valid arithmetic expression in a reverse polish notation.
The answer and all the intermediate calculations can be represented in a 32-bit integer.
*/

pub struct  Solution {}

const OPERATORS: [&str; 4] = [
    "+", "-", "*", "/"
];

impl Solution {
    pub fn eval_rpn(
        tokens: Vec<String>
    ) -> i32 {
        let mut stack = Vec::<i32>::with_capacity(tokens.len());

        for token in tokens.iter() {
            match token.as_str() {
                operator if OPERATORS.contains(&operator) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let res = match operator {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        "/" => left / right,
                        _ => panic!("Unrecognized operator")
                    };
                    stack.push(res);
                }
                operand => stack.push(
                    operand.parse::<i32>().unwrap()
                )
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluate_reverse_polish_notation_1 () {
        let res = Solution::eval_rpn(
            vec!["2","1","+","3","*"]
                .into_iter()
                .map(String::from)
                .collect()
        );
        assert_eq!(res, 9);
    }

    #[test]
    #[ignore]
    fn evaluate_reverse_polish_notation_2 () {
        let res = Solution::eval_rpn(
            vec!["4","13","5","/","+"]
                .into_iter()
                .map(String::from)
                .collect()
        );
        assert_eq!(res, 6);
    }

    #[test]
    #[ignore]
    fn evaluate_reverse_polish_notation_3 () {
        let res = Solution::eval_rpn(
            vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
                .into_iter()
                .map(String::from)
                .collect()
        );
        assert_eq!(res, 22);
    }        
}