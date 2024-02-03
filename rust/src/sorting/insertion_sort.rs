/// Sorts a mutable slice using in-place insertion
/// sort algorith.

pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let curr = arr[i];

        while j > 0 && curr < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
