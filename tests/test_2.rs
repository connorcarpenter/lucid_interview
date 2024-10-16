
// /*
//  * Part 2: Given a string of space separated words return a new string
//  * that is composed of the last letter of each word in the input
//  *    Example:
//  *    Given: "sell emu anthropomorphic alveoli acid"
//  *    Result: "lucid"
//  */
// string catLastLetter(string input)
// {
// string result = "";
// // your code here
// return result;
// }


#[test]
fn example_1() {
    let input = "sell emu anthropomorphic alveoli acid";
    let output = Solution::reverse_anagram(input);
    assert_eq!(output, "lucid".to_string());

    // // TestPart2("sell emu anthropomorphic alveoli acid", "lucid");
    // // TestPart2("", "word");
    // // TestPart2("z", "z");
    // // TestPart2("correct horse battery staple", "teye");

    let input = "w o r d";
    let output = Solution::reverse_anagram(input);
    assert_eq!(output, "word".to_string());

    let input = "z";
    let output = Solution::reverse_anagram(input);
    assert_eq!(output, "z".to_string());

    let input = "correct horse battery staple";
    let output = Solution::reverse_anagram(input);
    assert_eq!(output, "teye".to_string());
}

struct Solution;

impl Solution {
    fn reverse_anagram(input: &str) -> String {
        let mut output = String::new();
        let split = input.split(' ');
        for word in split {
            let word = word.chars();
            let last_char = word.last().unwrap();
            output.push(last_char);
        }
        output
    }
}