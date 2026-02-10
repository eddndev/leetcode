impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut tails: Vec<i32> = Vec::new();

        for env in envelopes {
            let height = env[1];

            match tails.binary_search(&height) {
                Ok(_) => {}
                Err(idx) => {
                    if idx == tails.len() {
                        tails.push(height);
                    } else {
                        tails[idx] = height;
                    }
                }
            }
        }

        tails.len() as i32
    }
}
