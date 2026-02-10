impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut indexed_nums: Vec<(i32, usize)> = nums
            .into_iter()
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect();

        let mut counts = vec![0; n];

        Self::merge_sort(&mut indexed_nums, &mut counts);

        counts
    }

    fn merge_sort(arr: &mut [(i32, usize)], counts: &mut [i32]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        Self::merge_sort(&mut arr[0..mid], counts);
        Self::merge_sort(&mut arr[mid..], counts);

        Self::merge(arr, mid, counts);
    }

    fn merge(arr: &mut [(i32, usize)], mid: usize, counts: &mut [i32]) {
        let mut temp = Vec::with_capacity(arr.len());

        let mut i = 0;
        let mut j = mid;
        let mut right_counter = 0;

        while i < mid && j < arr.len() {
            if arr[j].0 < arr[i].0 {
                temp.push(arr[j]);
                right_counter += 1;
                j += 1;
            } else {
                counts[arr[i].1] += right_counter;
                temp.push(arr[i]);
                i += 1;
            }
        }

        while i < mid {
            counts[arr[i].1] += right_counter;
            temp.push(arr[i]);
            i += 1;
        }

        while j < arr.len() {
            temp.push(arr[j]);
            j += 1;
        }

        arr.copy_from_slice(&temp);
    }
}
