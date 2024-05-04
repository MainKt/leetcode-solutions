use solutions_rs::Solution;

fn main() {
    let text1 = "abcde";
    let text2 = "ace";

    println!(
        "Longest common subsequence length: {}",
        Solution::longest_common_subsequence(text1.into(), text2.into())
    );
}
