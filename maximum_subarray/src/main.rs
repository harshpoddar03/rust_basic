pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_current = nums[0];
    let mut max_global = nums[0];

    nums.iter().skip(1).for_each(|&item| {
        max_current = std::cmp::max(item, max_current + item);
        if max_current > max_global {
            max_global = max_current;
        }
    });
    max_global
}
fn main() {
    print!("{}", max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}
