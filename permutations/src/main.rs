

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current_permutation = Vec::new(); 
    let mut used = vec![false; nums.len()]; 

    backtrack(&nums, &mut current_permutation, &mut used, &mut result); 

    result 
}

fn backtrack(
    nums: &Vec<i32>,
    current_permutation: &mut Vec<i32>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<i32>>,
) {
    if current_permutation.len() == nums.len() {

        result.push(current_permutation.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        current_permutation.push(nums[i]);
        used[i] = true;

        backtrack(nums, current_permutation, used, result);

        current_permutation.pop();
        used[i] = false;
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let result = permute(nums);
    println!("{:?}", result);
}