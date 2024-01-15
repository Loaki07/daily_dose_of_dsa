use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Trie {
    is_word_end: bool,
    children: HashMap<char, Trie>,
}

/**
 * `&self` means the method takes an immutable
 * reference. If you need a mutable reference,
 * change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self {
            is_word_end: false,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut trie = self;

        for c in word.chars() {
            trie = trie
                .children
                .entry(c)
                .or_insert(Trie::new());
        }

        trie.is_word_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut trie = self;

        for c in word.chars() {
            if let Some(ref next_trie) =
                trie.children.get(&c)
            {
                trie = next_trie;
            } else {
                return false;
            }
        }

        trie.is_word_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut trie = self;

        for c in prefix.chars() {
            if let Some(ref next_trie) =
                trie.children.get(&c)
            {
                trie = next_trie;
            } else {
                return false;
            }
        }
        true
    }
}

// Your Trie object will be instantiated and
// called as such: let obj = Trie::new();
// obj.insert(word);
// let ret_2: bool = obj.search(word);
// let ret_3: bool = obj.starts_with(prefix);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut trie = Trie::new();
        trie.insert("apple".into());
        assert!(trie.search("apple".into())); // return True
        assert!(!trie.search("app".into())); // return False
        assert!(trie.starts_with("app".into())); // return True
        trie.insert("app".into());
        assert!(trie.search("app".into())); // return True
    }
}
