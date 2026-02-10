impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        let (mut s_idx, mut p_idx) = (0, 0);
        let (mut star_idx, mut s_tmp_idx) = (None, 0);

        while s_idx < s_bytes.len() {
            if p_idx < p_bytes.len() && (p_bytes[p_idx] == b'?' || p_bytes[p_idx] == s_bytes[s_idx])
            {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
                star_idx = Some(p_idx);
                s_tmp_idx = s_idx;
                p_idx += 1;
            } else if let Some(star_p) = star_idx {
                p_idx = star_p + 1;
                s_tmp_idx += 1;
                s_idx = s_tmp_idx;
            } else {
                return false;
            }
        }

        while p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
            p_idx += 1;
        }

        p_idx == p_bytes.len()
    }
}
