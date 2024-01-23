use core::num;

pub fn remove_element(
    nums: &mut Vec<i32>,
    val: i32,
) -> i32 {
    while let Some(index) =
        nums.iter().position(|v| *v == val)
    {
        nums.swap_remove(index);
    }

    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let res = remove_element(&mut nums, 2);
        assert_eq!(res, 5);
    }
}
