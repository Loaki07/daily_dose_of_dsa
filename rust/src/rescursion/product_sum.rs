#[derive(Debug)]
enum NestedList {
    Int(i32),
    List(Vec<NestedList>),
}

fn product_sum_helper(
    nested_list: &NestedList,
    multiplier: i32,
) -> i32 {
    // dbg!(multiplier);

    match nested_list {
        NestedList::Int(value) => {
            // dbg!(value, multiplier);

            value * multiplier
        }
        NestedList::List(list) => {
            let sub_sum = list
                .iter()
                .map(|x| {
                    product_sum_helper(x, multiplier + 1)
                })
                .sum::<i32>();

            // dbg!(sub_sum, multiplier);

            sub_sum * multiplier
        }
    }
}

// [5, 2, [7, -1], 3, [6, [-13, 8], 4]]
// 12
// calculated as: 5 + 2 + 2 * (7 - 1) + 3 +
// 2 * (6 + 3 * (13 + 8) + 4) O(n) time | O(d)
// space where d is the depth
pub fn product_sum(nested_list: &Vec<NestedList>) -> i32 {
    let mut sum = 0;
    for element in nested_list {
        sum += product_sum_helper(element, 1)
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let nested_list = vec![
            NestedList::Int(5),
            NestedList::Int(2),
            NestedList::List(vec![
                NestedList::Int(7),
                NestedList::Int(-1),
            ]),
            NestedList::Int(3),
            NestedList::List(vec![
                NestedList::Int(6),
                NestedList::List(vec![
                    NestedList::Int(-13),
                    NestedList::Int(8),
                ]),
                NestedList::Int(4),
            ]),
        ];

        assert_eq!(product_sum(&nested_list), 12);
    }

    #[test]
    fn ex2() {
        let nested_list = vec![
            NestedList::Int(1),
            NestedList::Int(2),
            NestedList::List(vec![NestedList::Int(3)]),
            NestedList::Int(4),
            NestedList::Int(5),
        ];

        assert_eq!(product_sum(&nested_list), 18);
    }
}
