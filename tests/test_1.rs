
// /*
//  * Part 1: Given a list of numbers, return the 3 largest numbers that are divisible by 7
//  * sorted from largest to smallest, for example, given the list
//  * [14,3,21,17,7,70]  this function should return [70,21,14]
//  * Note:  you can assume for purposes of this problem that all input will have at least
//  * three integers that are divisible by 7
//  */
// vector<int> findLargestThreeIntsDivisibleBySeven(vector<int> input)
// {
// vector<int> result;
// // your code here
// return result;
// }


#[test]
fn example_1() {
    let input = vec![14,3,21,17,7,70];
    let output = Solution::find_3_largest(input);
    assert_eq!(output, [70,21,14]);
}

struct Solution;

impl Solution {
    fn find_3_largest(input: Vec<i32>) -> [i32; 3] {
        // largest -> smallest
        let mut output = [i32::MIN, i32::MIN, i32::MIN];

        for value in input {
            if value % 7 == 0 {

                if value > output[0] {
                    output[2] = output[1];
                    output[1] = output[0];
                    output[0] = value;
                } else if value > output[1] {
                    output[2] = output[1];
                    output[1] = value;
                } else if value > output[2] {
                    output[2] = value;
                }
            }
        }

        output
    }
}