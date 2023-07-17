// it needs to be converted into O(n)
fn findLargestSubarraySum_O_of_n_2(v: Vec<i32>) -> i32 {
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
fn findLargestSubarraySum_O_of_n(v: Vec<i32>) -> i32 {
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
    println!("Sub-array indexes: ({}, {})", left, right - 1);
    return max_sum;
}

fn main() {
    println!("Demonstration of Kardane's algorithm.");
    println!("enter numbers separated by a space: ");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("a string expected!!");
    line = line.trim_end().to_string();
    //line.push('\0');
    println!("provided: {:?}", line);
    let input_arr: Vec<i32> = line
        .split(" ")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap_or_default())
        .collect();
    println!("using array: {:?}", input_arr);
    let k_sum = findLargestSubarraySum_O_of_n(input_arr);
    println!("max sub array sum: {}", k_sum);
}
