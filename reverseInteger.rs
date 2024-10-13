impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let mut x = x.abs();
        let mut rev = 0;
        while x != 0 {
            let digits = x % 10;
            rev = rev * 10 + digits;
            x /= 10;
        }
        sign * rev
    }
}