fn linear_search(arr: Vec<i32>, target: i32) -> Option<usize> {
    for (i, v) in arr.iter().enumerate() {
        if target.eq(&v) {
            return Some(i);
        }
    }
    return None;
}

#[test]
fn test_linear_search() {
    let arr = Vec::from([6, 7, 8, 9, 10, 5, 1, 2, 3, 4]);
    match linear_search(arr, 5) {
        None => println!("Not found"),
        Some(pos) => {
            assert_eq!(pos, 5);
            println!("Found at index {}", pos)
        }
    }
}

fn binary_search(arr: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = arr[0] as usize;
    let mut right = arr[arr.len() - 1] as usize;

    while left <= right {
        let mid = left + ((right - left) / 2);

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
    let arr = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    match binary_search(arr, 8) {
        None => println!("Not found"),
        Some(pos) => {
            assert_eq!(pos, 7);
            println!("Found at index {}", pos)
        }
    };
}