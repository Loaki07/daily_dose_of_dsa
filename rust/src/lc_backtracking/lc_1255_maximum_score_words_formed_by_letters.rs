pub struct Solution;

impl Solution {
    // O(2^n  + W + L) time | O(n + W) space
    pub fn max_score_words(
        words: Vec<String>,
        letters: Vec<char>,
        score: Vec<i32>,
    ) -> i32 {
        let mut count = [0; 26];
        for &ch in letters.iter() {
            count[(ch as u8 - b'a') as usize] += 1;
        }
        let mut freqs = vec![[0; 27]; words.len()];
        for (i, s) in words.iter().enumerate() {
            for b in s.bytes() {
                let j = (b - b'a') as usize;
                freqs[i][j] += 1;
                freqs[i][26] += score[j];
            }
        }
        Self::backtrack(0, &mut count, &freqs)
    }

    fn backtrack(
        i: usize,
        count: &mut [i32; 26],
        freqs: &Vec<[i32; 27]>,
    ) -> i32 {
        if i == freqs.len() {
            return 0;
        }

        let mut res = Self::backtrack(i + 1, count, freqs);
        if (0..26).all(|j| count[j] >= freqs[i][j]) {
            let mut acc = 0;
            for j in 0..26 {
                count[j] -= freqs[i][j];
            }

            let res_2 =
                Self::backtrack(i + 1, count, freqs);
            res = res.max(freqs[i][26] + res_2);
            for j in 0..26 {
                count[j] += freqs[i][j];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let words = vec![
            "dog".to_string(),
            "cat".to_string(),
            "dad".to_string(),
            "good".to_string(),
        ];
        let letters = vec![
            'a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o',
        ];
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = Solution::max_score_words(
            words, letters, score,
        );
        assert_eq!(result, 23);
    }

    #[test]
    fn example2() {
        let words = vec![
            "xxxz".to_string(),
            "ax".to_string(),
            "bx".to_string(),
            "cx".to_string(),
        ];
        let letters =
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ];
        let result = Solution::max_score_words(
            words, letters, score,
        );
        assert_eq!(result, 27);
    }

    #[test]
    fn example3() {
        let words = vec!["leetcode".to_string()];
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ];
        let result = Solution::max_score_words(
            words, letters, score,
        );
        assert_eq!(result, 0);
    }
}
