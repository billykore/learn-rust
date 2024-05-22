fn bubble_sort(nums: &mut Vec<i32>) {
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            nums.swap(i, i + 1)
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut nums = Vec::from([1, 8, 6, 7, 4]);
    bubble_sort(&mut nums);
    assert_eq!(Vec::from([1, 4, 6, 7, 8]), nums)
}
