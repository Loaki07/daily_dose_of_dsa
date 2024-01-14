use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    // The code first creates a HashMap (hm) to store
    // the frequency of each character in magazine.

    // It iterates over each character (ch) in
    // magazine. For each character, it uses the
    // entry API of HashMap to fetch or insert a
    // default value of 0. It then increments the
    // count of this character
    // (*hm.entry(ch).or_insert(0) += 1;).
    // After this loop, hm contains the count of every
    // character available in the magazine.
    //     The code then iterates over each character
    // in the ransom_note:

    // For each character in ransom_note, it checks
    // the frequency of that character in hm.
    // If the character is not found in the magazine
    // (i.e., its count in hm is 0), the function
    // returns false, indicating that the ransom note
    // cannot be constructed. If the character is
    // found, it decrements the count in hm and
    // continues the process.
    pub fn can_construct(
        ransom_note: String,
        magazine: String,
    ) -> bool {
        let mut hm = HashMap::new();

        let mut count = 0;
        for ch in ransom_note.chars() {
            *hm.entry(ch).or_insert(0) += 1;
            count += 1;
        }

        for ch in magazine.chars() {
            if let Entry::Occupied(mut e) = hm.entry(ch) {
                if *e.get() > 0 {
                    *e.get_mut() -= 1;
                    count -= 1;
                    if count == 0 {
                        return true;
                    }
                }
            }
        }

        false

        /* Alternate soln
        for ch in magazine.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            let e = hm.entry(ch).or_insert(0);
            if *e == 0 {
                return false;
            }
            *e -= 1;
        }

        true
        */
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(!Solution::can_construct(
            "a".to_string(),
            "b".to_string()
        ));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::can_construct(
            "aa".to_string(),
            "ab".to_string()
        ));
    }

    #[test]
    fn ex3() {
        assert!(Solution::can_construct(
            "aa".to_string(),
            "aab".to_string()
        ));
    }
}
