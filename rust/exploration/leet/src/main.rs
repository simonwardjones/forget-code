// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

fn main() {
    println!("Hello, world!");
    let example_1: Vec<i32> = vec![2, 7, 11, 15];
    let target_1: i32 = 9;
    let example_2: Vec<i32> = vec![3, 2, 4];
    let target_2: i32 = 6;
    let result_1 = Solution::two_sum(example_1, target_1);
    let result_2 = Solution::two_sum(example_2, target_2);
    println!("result_1 = {:?}", result_1);
    println!("result_2 = {:?}", result_2);

    // Just playing with iter iter_mut and into_iter 
    let mut simon: Vec<String> = vec![1.to_string(), 2.to_string(), 3.to_string()];
    println!("{simon:?}");
    for value in simon.iter_mut() {
        value.push_str("!!!!");
        println!("{value}")
    }
    println!("{simon:?}");
}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in numbers.iter().enumerate() {
            for (j, y) in numbers[(i + 1)..].iter().enumerate() {
                // println!("i={i}, j={j} x={x}, y={y}, target={target}");
                if x + y == target {
                    return vec![i.try_into().unwrap(), (i + j + 1).try_into().unwrap()];
                }
            }
        }
        vec![]
    }
}
