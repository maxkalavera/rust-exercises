/*
name: exclusive_time_of_functions

description: On a single-threaded CPU, we execute a program containing n functions. Each function has a unique ID between 0 and n - 1.

Function calls are stored in a call stack: when a function call starts, its ID is pushed onto the stack, and when a function call ends, its ID is popped off the stack. The function whose ID is at the top of the stack is the current function being executed. Each time a function starts or ends, we write a log with the ID, whether it started or ended, and the timestamp.

You are given a list logs, where logs[i] represents the ith log message formatted as a string "{function_id}:{"start" | "end"}:{timestamp}". For example, "0:start:3" means a function call with function ID 0 started at the beginning of timestamp 3, and "1:end:2" means a function call with function ID 1 ended at the end of timestamp 2. Note that a function can be called multiple times, possibly recursively.

A function's exclusive time is the sum of execution times for all function calls in the program. For example, if a function is called twice, one call executing for 2 time units and another call executing for 1 time unit, the exclusive time is 2 + 1 = 3.

Return the exclusive time of each function in an array, where the value at the ith index represents the exclusive time for the function with ID i.


*/

pub struct  Solution {}

#[derive(Debug, Clone)]
enum EventType {
    Start,
    End
}

type ProcessEvent = (i32, EventType, i32); // (id, event, timestamp)

impl Solution {
    pub fn exclusive_time(
        n: i32, 
        logs: Vec<String>
    ) -> Vec<i32> {
        let mut exclusive_times = (0..n).map(|_| 0 as i32).collect::<Vec<i32>>();
        let logs  = logs
            .iter()
            .map(Solution::parse_log)
            .collect::<Vec<ProcessEvent>>();
        println!("Logs: {:?}", logs);
        println!("------------");

        let mut stack = Vec::<ProcessEvent>::with_capacity(logs.len());
        for event in logs.iter() {
            match event.1 {
                EventType::Start => {
                    stack.push(event.clone());
                }
                EventType::End => {
                    let finished = stack.pop().unwrap();
                    // println!("Finished: {:?}, Current: {:?}", former, top);
                    // println!("Opener event: {:?}", finished);
                    // println!("Closer event: {:?}", event);
                    // println!("Time elapsed: {:?}", (event.2 + 1) - finished.2);
                    let elapsed = (event.2 + 1) - finished.2;
                    exclusive_times[event.0 as usize] += elapsed;
                    if stack.len() > 0 {
                        let top = stack.last().unwrap();
                        exclusive_times[top.0 as usize] -= elapsed
                    }
                }
            }
            // println!("-----");
        }
        // println!("Exclusive times: {:?}", exclusive_times);

        exclusive_times
    }

    fn parse_log (
        log: &String
    ) -> ProcessEvent
    {
        let parts = log.split(":").collect::<Vec<&str>>();
        (
            parts.get(0).unwrap().parse::<i32>().unwrap(),
            match parts.get(1).unwrap().as_ref() {
                "start" => EventType::Start,
                "end" => EventType::End,
                _ => panic!("")
            },
            parts.get(2).unwrap().parse::<i32>().unwrap()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exclusive_time_of_functions_1 () {
        let res = Solution::exclusive_time(
            2,
            vec!["0:start:0","1:start:2","1:end:5","0:end:6"]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(res, vec![3,4]);
    }

    #[test]
    fn exclusive_time_of_functions_2 () {
        let res = Solution::exclusive_time(
            1,
            vec!["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(res, vec![8]);
    }

    #[test]
    fn exclusive_time_of_functions_3 () {
        let res = Solution::exclusive_time(
            2,
            vec!["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(res, vec![7,1]);
    }

    #[test]
    fn exclusive_time_of_functions_4 () {
        let res = Solution::exclusive_time(
            1,
            vec!["0:start:0","0:start:1","0:start:2","0:end:3","0:end:4","0:end:5"]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(res, vec![6]);
    }
}