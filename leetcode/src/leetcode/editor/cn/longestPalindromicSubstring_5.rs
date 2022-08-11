
/**
给你一个字符串 s，找到 s 中最长的回文子串。 

 

 示例 1： 

 
输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
 

 示例 2： 

 
输入：s = "cbbd"
输出："bb"
 

 

 提示： 

 
 1 <= s.length <= 1000 
 s 仅由数字和英文字母组成 
 

 Related Topics 字符串 动态规划 👍 5561 👎 0

*/
//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut nums = HashMap::new();
        let mut vec = vec![];
        let mut sum = 0;

        for item in s.chars(){
            let count = nums.entry(item).or_insert(0);
            *count += 1;
        }

        for (_key,value) in nums{
            vec.push(value);
        }

        vec.sort();  // 想要了解排序的掘友可以查看文章末尾的第二个资料
        vec.reverse();

        for item in &vec{
            if *item > 1 {
                if item % 2 == 0{
                    sum += item;
                }else{
                    sum += item - 1;
                }
            }
        }

        if sum < s.len() && sum % 2 == 0 {
            sum += 1;
        }

        sum as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

fn main() {

}