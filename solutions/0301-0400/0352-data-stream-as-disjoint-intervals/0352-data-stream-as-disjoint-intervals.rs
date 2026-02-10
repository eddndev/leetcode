use std::collections::BTreeMap;

struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let prev = self
            .intervals
            .range(..=value)
            .next_back()
            .map(|(&start, &end)| (start, end));

        if let Some((_, end)) = prev {
            if end >= value {
                return;
            }
        }

        let merge_left = prev.map_or(false, |(_, end)| end == value - 1);

        let next = self
            .intervals
            .range((value + 1)..)
            .next()
            .map(|(&start, &end)| (start, end));
        let merge_right = next.map_or(false, |(start, _)| start == value + 1);

        match (merge_left, merge_right) {
            (true, true) => {
                let (prev_start, _) = prev.unwrap();
                let (next_start, next_end) = next.unwrap();

                self.intervals.remove(&next_start);
                self.intervals.insert(prev_start, next_end);
            }
            (true, false) => {
                let (prev_start, _) = prev.unwrap();
                self.intervals.insert(prev_start, value);
            }
            (false, true) => {
                let (next_start, next_end) = next.unwrap();
                self.intervals.remove(&next_start);
                self.intervals.insert(value, next_end);
            }
            (false, false) => {
                self.intervals.insert(value, value);
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals
            .iter()
            .map(|(&start, &end)| vec![start, end])
            .collect()
    }
}
