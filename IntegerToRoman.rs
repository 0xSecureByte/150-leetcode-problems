impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_dict = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut roman_num = String::new();
        let mut num = num;
        for &(value, symbol) in &roman_dict {
            let count = num / value;
            if count > 0 {
                roman_num.push_str(&symbol.repeat(count as usize));
                num -= value * count;
            }
        }
        roman_num
    }
}