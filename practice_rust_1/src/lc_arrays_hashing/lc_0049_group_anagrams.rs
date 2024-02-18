// soln has O(m*n*26) complexity
pub fn group_anagrams(
    strs: Vec<String>,
) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut hm: HashMap<[u8; 26], Vec<String>> =
        HashMap::new();

    for s in strs {
        let mut key = [0_u8; 26];

        for ch in s.chars() {
            key[ch as usize - 'a' as usize] += 1;
        }

        if let Some(val) = hm.get_mut(&key) {
            val.push(s);
        } else {
            hm.insert(key, vec![s]);
        }
    }

    hm.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        // Prepare the input
        let input =
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>();

        // Call the group_anagrams function
        let mut result = group_anagrams(input);

        // Sort the inner vectors and the outer vector to
        // make testing for equality possible
        for group in result.iter_mut() {
            group.sort_unstable();
        }
        result.sort_unstable();

        // Prepare the expected output
        let mut expected_output = vec![
            vec!["bat"],
            vec!["nat", "tan"],
            vec!["ate", "eat", "tea"],
        ];

        // Sort the inner vectors and the outer vector of
        // the expected output
        for group in expected_output.iter_mut() {
            group.sort_unstable();
        }
        expected_output.sort_unstable();

        // Assert that the result equals the expected
        // output
        assert_eq!(result, expected_output);
    }
}
