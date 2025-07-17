pub fn num_subarrays_with_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;  
    }
    
   let mut product = 1;
    let mut left = 0;
    let mut count = 0;
    
    for right in 0..nums.len(){
        product *= nums[right];
        
        while product >= k{
            product /= nums[left];
            left += 1;
        }

        if left <= right {
            count += right - left + 1;
        }
    }

    count as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case_1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 8);
        // Subarrays: [10], [5], [2], [6], [10,5], [5,2], [2,6], [5,2,6]
    }

    #[test]
    fn test_basic_case_2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 0);
        // No valid subarrays since k = 0
    }

    #[test]
    fn test_single_element_valid() {
        let nums = vec![5];
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_element_invalid() {
        let nums = vec![10];
        let k = 5;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_k_equals_1() {
        let nums = vec![1, 2, 3];
        let k = 1;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 0);
        // All elements are >= 1, so no valid subarrays
    }

    #[test]
    fn test_all_elements_less_than_k() {
        let nums = vec![1, 1, 1];
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 6);
        // All possible subarrays: [1], [1], [1], [1,1], [1,1], [1,1,1]
    }

    #[test]
    fn test_all_elements_greater_than_k() {
        let nums = vec![100, 200, 300];
        let k = 50;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_empty_array() {
        let nums = vec![];
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_large_k() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 1000;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 15);
        // All possible subarrays are valid
    }

    #[test]
    fn test_ones_in_array() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 5;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 15);
        // All subarrays have product < 5
    }

    #[test]
    fn test_mixed_values() {
        let nums = vec![2, 3, 1, 4];
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 8);
        // Valid subarrays: [2], [3], [1], [4], [2,3], [1,4], [3,1], [2,3,1]
    }

    #[test]
    fn test_edge_case_k_large() {
        let nums = vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3];
        let k = 19;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_consecutive_small_numbers() {
        let nums = vec![1, 2, 1, 2, 1];
        let k = 5;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_product_exactly_k() {
        let nums = vec![2, 5]; // product = 10
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 2); // Only [2] and [5] are valid
    }

    #[test]
    fn test_sliding_window_case() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 9);
        // Valid subarrays with product < 10
    }

    #[test]
    fn test_large_array() {
        let nums = vec![1; 1000]; // 1000 ones
        let k = 2;
        let result = num_subarrays_with_product_less_than_k(nums, k);
        assert_eq!(result, 500500); // n*(n+1)/2 where n=1000
    }
}
