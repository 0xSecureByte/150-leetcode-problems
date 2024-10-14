use std::{cmp::Ordering, collections::HashSet};


struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicates for i
            }
            
            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    result.insert(vec![nums[i], nums[j], nums[k]]);
                    
                    // Skip duplicates for j and k
                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k - 1] {
                        k -= 1;
                    }
                    
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        result.into_iter().collect()
    }
}
fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    println!("{:?}", Solution::three_sum(nums));
}