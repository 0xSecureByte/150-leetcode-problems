struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut comm_prefix = String::new();
        let mut j = 0;

        loop {
            if j >= strs[0].len() {
                break;
            }

            let current_char = strs[0].chars().nth(j).unwrap();
            for i in 0..strs.len() {
                if j >= strs[i].len() || strs[i].chars().nth(j).unwrap() != current_char {
                    return comm_prefix;
                }
            }
            
            comm_prefix.push(current_char);
            j += 1;
        }
        comm_prefix
    }
}

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("{}", Solution::longest_common_prefix(strs));

    let strs2 = vec!["ab".to_string(), "a".to_string()];
    println!("{}", Solution::longest_common_prefix(strs2));
}