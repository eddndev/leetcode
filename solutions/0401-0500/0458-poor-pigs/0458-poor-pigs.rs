impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        let states = rounds + 1;
        let mut pigs = 0;
        let mut capacity = 1;

        while capacity < buckets {
            capacity *= states;
            pigs += 1;
        }

        pigs
    }
}
