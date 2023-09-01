// it needs to be converted into O(n)
fn findLargestSubarraySum_quadratic(v: &Vec<i32>) -> i32 {
    let mut max_sum: i32 = v[0];
    for i in 0..v.len() {
        let mut cur_sum = 0;
        for j in i..v.len() {
            cur_sum += v[j];
            max_sum = max_sum.max(cur_sum);
        }
    }
    return max_sum;
}

// kardane's algorithm
// skipping subarray sum so far if with current it becomes negative
// because we will consider only the sum which increases the overall sum
// if curr_sum is negative means adding current value has made overall sum -ve
// so we will set curr_sum to 0, and goto next n
fn findLargestSubarraySum_linear(v: &Vec<i32>) -> (i32, usize, usize) {
    let mut max_sum = v[0];
    let mut curr_sum = 0;
    let mut left = 0usize;
    let mut right = 0usize;

    for n in v.iter() {
        // if curr_sum is -ve, set it to 0
        curr_sum = 0.max(curr_sum);
        if curr_sum == 0 {
            left = right
        }
        // sum with prev curr_sum
        curr_sum += n;
        // if curr_sum > max_sum, set it to curr_sum
        max_sum = max_sum.max(curr_sum);
        right += 1;
    }
    //println!("Sub-array indexes: ({}, {})", left, right - 1);
    (max_sum, left, right)
}

#[cfg(test)]
mod kardanes_tests {
    use super::*;

    #[test]
    fn test_kardanes_algo_works() {
        let input_arr: Vec<i32> = vec![4, -1, 2, -7, 3, 4];

        let maxq = findLargestSubarraySum_quadratic(&input_arr);

        assert_eq!(maxq, 7);

        let (maxl, left, right) = findLargestSubarraySum_linear(&input_arr);

        assert_eq!(maxl, maxq);
        assert_eq!(maxl, 7);
        assert_eq!(left, 4);
        assert_eq!(right, 6);
    }
}
