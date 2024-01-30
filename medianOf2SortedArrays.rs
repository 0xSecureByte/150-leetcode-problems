impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = nums1.clone();
        merged.extend(nums2);
        merged.sort();
        let len = merged.len();
        if len % 2 == 0 {
            // If the merged array has an even length, take the average of the middle elements
            let mid1 = merged[len / 2 - 1] as f64;
            let mid2 = merged[len / 2] as f64;
            (mid1 + mid2) / 2.0
        } else {
            // If the merged array has an odd length, take the middle element
            merged[len / 2] as f64
        }
    }
}
