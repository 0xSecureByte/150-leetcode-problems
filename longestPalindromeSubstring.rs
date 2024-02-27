impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        if n <= 1 {
            return s.iter().collect();
        }

        let mut start = 0;
        let mut max_len = 1;

        for i in 0..n {
            let mut left = i;
            let mut right = i;

            // Expand around the center for odd-length palindromes
            while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }

            let len = right - left + 1;
            if len > max_len {
                start = left;
                max_len = len;
            }

            // Reset for even-length palindromes
            left = i;
            right = i + 1;

            while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }

            let len = right - left + 1;
            if len > max_len {
                start = left;
                max_len = len;
            }
        }

        s[start..start + max_len].iter().collect()
    }
}
