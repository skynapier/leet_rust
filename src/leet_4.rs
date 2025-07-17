pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    let (mut prev, mut curr) = (0,0);
    
    let (mut i, mut j) = (0, 0);
    
    for _ in 0..(m+n) / 2 + 1 {
        prev = curr;
        
        if i < m && (j >= n || nums1[i] <= nums2[j]){
            curr = nums1[i];
            i +=1;
        }else{
            curr = nums2[j];
            j +=1;
        }
        
    }
    
    if (n+m) % 2 == 1{
        curr as f64
    }else{
        (prev + curr) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_basic_case_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_empty_first_array() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_empty_second_array() {
        let nums1 = vec![2];
        let nums2 = vec![];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_single_element_each() {
        let nums1 = vec![1];
        let nums2 = vec![2];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_odd_total_length() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4, 5];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_even_total_length() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_negative_numbers() {
        let nums1 = vec![-2, -1];
        let nums2 = vec![0, 1];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, -0.5);
    }

    #[test]
    fn test_duplicate_numbers() {
        let nums1 = vec![1, 1];
        let nums2 = vec![1, 2];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_large_arrays() {
        let nums1 = vec![1, 3, 5, 7, 9];
        let nums2 = vec![2, 4, 6, 8, 10];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 5.5);
    }

    #[test]
    fn test_one_array_much_larger() {
        let nums1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let nums2 = vec![9];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_all_elements_in_first_array_smaller() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_all_elements_in_second_array_smaller() {
        let nums1 = vec![4, 5, 6];
        let nums2 = vec![1, 2, 3];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_floating_point_precision() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2, 4];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert!((result - 2.5).abs() < 1e-9);
    }

    #[test]
    fn test_zero_values() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let nums1 = vec![-5, -3, -1];
        let nums2 = vec![1, 3, 5];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 0.0);
    }
}

