use std::collections::{HashMap, HashSet};

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();

    if len < 3 {
        return vec![];
    }

    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut results = HashSet::new();

    for (i, &num) in nums.iter().enumerate() {
        map.insert(num, i);
    }

    for i in 0..len-1 {
        for j in i+1..len {
            let mut key_set = HashSet::new();
            let complement = 0 - (nums[i] + nums[j]);
            key_set.insert(i);
            key_set.insert(j);
            if let Some(&k) = map.get(&complement) {
                key_set.insert(k);
            }
            if key_set.len() == 3 {
                let mut result = Vec::new();
                for key in key_set {
                    result.push(nums[key]);
                }
                result.sort();
                results.insert(result);
            }
        }
    }

    results.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = three_sum(nums);
        result.sort();
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_array() {
        let nums = vec![];
        let result = three_sum(nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_small_array() {
        let nums = vec![0, 1];
        let result = three_sum(nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_no_solution() {
        let nums = vec![1, 2, 3];
        let result = three_sum(nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0];
        let result = three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_multiple_zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-2, 0, 1, 1, 2];
        let mut result = three_sum(nums);
        result.sort();
        let mut expected = vec![vec![-2, 0, 2], vec![-2, 1, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicates() {
        let nums = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
        let result = three_sum(nums);
        assert!(!result.is_empty());

        let mut sorted_results = result.clone();
        for triplet in &mut sorted_results {
            triplet.sort();
        }
        sorted_results.sort();
        sorted_results.dedup();
        assert_eq!(result.len(), sorted_results.len());
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![-100000, -99999, -99998, 0, 99998, 99999, 100000];
        let result = three_sum(nums);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_single_solution() {
        let nums = vec![-1, 0, 1];
        let result = three_sum(nums);
        assert_eq!(result, vec![vec![-1, 0, 1]]);
    }

    #[test]
    fn test_boundary_case() {
        let nums = vec![0, 0, 0, 0, 0];
        let result = three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}


