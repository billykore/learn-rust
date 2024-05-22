fn binary_search(arr: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = arr[0] as usize;
    let mut right = arr[arr.len() - 1] as usize;
    let mid = (left + (right - left) / 2);

    while left <= right {
        if arr[mid] == target {
            return Some(mid);
        }

        if arr[mid] < target {
            left = mid + 1;
        }

        if arr[mid] > target {
            right = mid - 1;
        }
    }

    return None;
}

#[test]
fn test_binary_search() {
    let arr = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    let pos = binary_search(arr, 8);
    match pos {
        None => println!("Not found"),
        Some(idx) => assert_eq!(idx, 7)
    }
}