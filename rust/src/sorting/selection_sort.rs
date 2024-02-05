// O(n^2) time
// O(1) space
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;

        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(left, smallest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
