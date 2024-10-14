impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_dict = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        
        let mut num = 0;
        let mut i = 0;
        let bytes = s.as_bytes();

        while i < bytes.len() {
            let mut found = false;
            for &(symbol, value) in &roman_dict {
                let symbol_bytes = symbol.as_bytes();
                if i + symbol_bytes.len() <= bytes.len() && &bytes[i..i + symbol_bytes.len()] == symbol_bytes {
                    num += value;
                    i += symbol_bytes.len();
                    found = true;
                    break;
                }
            }
            if !found {
                i += 1;
            }
        }
        num
    }
}