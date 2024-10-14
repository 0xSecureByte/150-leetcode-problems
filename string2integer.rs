impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut num = 0i32;
        let mut i = 0;
        let mut sign = 1;
        let chars = s.as_bytes(); // Using byte slice for more efficient indexing

        // Skipping leading whitespaces
        while i < chars.len() && chars[i] == b' ' {
            i += 1;
        }

        // Checking for sign
        if i < chars.len() {
            if chars[i] == b'-' {
                sign = -1;
                i += 1;
            } else if chars[i] == b'+' {
                i += 1;
            }
        }

        // Converting characters to integer
        while i < chars.len() && (chars[i] as char).is_digit(10) {
            let digit = (chars[i] - b'0') as i32;

            // Checking for overflows
            if num > (i32::MAX - digit) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            num = num * 10 + digit;
            i += 1;
        }

        sign * num
    }
}