impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_helper(s.as_bytes(), p.as_bytes())
    }

    fn is_match_helper(s: &[u8], p: &[u8]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        
        let first_match = !s.is_empty() && (p[0] == s[0] || p[0] == b'.');
        
        if p.len() >= 2 && p[1] == b'*' {
            return Self::is_match_helper(s, &p[2..]) || (first_match && Self::is_match_helper(&s[1..], p));
        }
        
        first_match && Self::is_match_helper(&s[1..], &p[1..])
    }
}