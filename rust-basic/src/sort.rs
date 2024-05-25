fn bubble_sort(arr: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len()-1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut nums = Vec::from([1, 8, 6, 7, 4]);
    bubble_sort(&mut nums);
    assert_eq!(Vec::from([1, 4, 6, 7, 8]), nums)
}

#[test]
fn test_sort() {
    let mut nums = [1, 8, 6, 7, 4];
    nums.sort();
    print!("{:?}", nums);
}
