pub struct Solution;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Handle(usize);

const NIL_HANDLE: Handle = Handle(usize::MAX);

struct TrieNode {
    terminal: bool,
    children: [Handle; 26],
}

struct Trie {
    nodes: Vec<TrieNode>,
    root: Handle,
}

impl Trie {
    fn new() -> Self {
        let mut trie = Trie {
            nodes: Vec::new(),
            root: NIL_HANDLE,
        };
        trie.root = trie.new_node();
        trie
    }

    fn add_word(&mut self, s: &[u8]) {
        let mut hcurr = self.root;
        for &ch in s {
            let idx = (ch - b'a') as usize;
            let mut hnext =
                self.nodes[hcurr.0].children[idx];
            if hnext == NIL_HANDLE {
                hnext = self.new_node();
                self.nodes[hcurr.0].children[idx] = hnext;
            }
            hcurr = hnext;
        }
        self.nodes[hcurr.0].terminal = true;
    }

    fn find_prefixes(
        &self,
        s: &[u8],
    ) -> Option<Vec<usize>> {
        let mut result = None;
        let mut hcurr = self.root;
        for (i, &ch) in s.iter().enumerate() {
            let idx = (ch - b'a') as usize;
            let hnext = self.nodes[hcurr.0].children[idx];
            if hnext == NIL_HANDLE {
                break;
            }
            if self.nodes[hnext.0].terminal {
                result
                    .get_or_insert_with(|| Vec::new())
                    .push(i);
            }
            hcurr = hnext;
        }
        result
    }

    fn new_node(&mut self) -> Handle {
        let node = TrieNode {
            terminal: false,
            children: [NIL_HANDLE; 26],
        };
        self.nodes.push(node);
        Handle(self.nodes.len() - 1)
    }
}

impl Solution {
    pub fn replace_words(
        dictionary: Vec<String>,
        sentence: String,
    ) -> String {
        let mut trie = Trie::new();
        let mut ans = String::new();

        for word in dictionary {
            trie.add_word(word.as_bytes());
        }

        for word in sentence.split(' ') {
            if let Some(prefixes) =
                trie.find_prefixes(word.as_bytes())
            {
                ans.push_str(&word[0..=prefixes[0]]);
            } else {
                ans.push_str(word);
            }
            ans.push(' ');
        }
        ans.pop();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_replace_words_example1() {
        let dictionary = vec![
            "cat".to_string(),
            "bat".to_string(),
            "rat".to_string(),
        ];
        let sentence =
            "the cattle was rattled by the battery"
                .to_string();
        let expected =
            "the cat was rat by the bat".to_string();
        assert_eq!(
            Solution::replace_words(dictionary, sentence),
            expected
        );
    }

    #[test]
    fn test_replace_words_example2() {
        let dictionary = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        let sentence =
            "aadsfasf absbs bbab cadsfafs".to_string();
        let expected = "a a b c".to_string();
        assert_eq!(
            Solution::replace_words(dictionary, sentence),
            expected
        );
    }
}
