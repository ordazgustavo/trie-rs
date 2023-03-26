#[derive(Default, Debug)]
struct Node {
    children: Vec<Node>,
    key: char,
    value: Option<String>,
    is_terminal: bool,
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            key: value,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn insert(&mut self, string: &str) {
        let mut node = &mut self.root;

        for ch in string.chars() {
            match node.children.binary_search_by(|n| n.key.cmp(&ch)) {
                Ok(i) => {
                    node = &mut node.children[i];
                }
                Err(i) => {
                    node.children.insert(i, Node::new(ch));
                    node = &mut node.children[i];
                }
            };
        }

        node.is_terminal = true;
        node.value.replace(string.to_string());
    }

    pub fn has(&self, string: &str) -> bool {
        let mut is_contained = false;

        let mut node = &self.root;
        for ch in string.chars() {
            match node.children.binary_search_by(|n| n.key.cmp(&ch)) {
                Ok(i) => {
                    is_contained = true;

                    node = &node.children[i];
                }
                Err(_) => return false,
            };
        }

        if !node.is_terminal {
            return false;
        }

        is_contained
    }

    pub fn get(&self, string: &str) -> Option<String> {
        let mut node = &self.root;
        for ch in string.chars() {
            match node.children.binary_search_by(|n| n.key.cmp(&ch)) {
                Ok(i) => {
                    node = &node.children[i];
                }
                Err(_) => return None,
            };
        }

        if !node.is_terminal {
            return None;
        }

        node.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_single_char() {
        let mut trie = Trie::default();
        trie.insert("a");

        assert!(trie.has("a"));
    }

    #[test]
    fn handles_multiple_chars() {
        let mut trie = Trie::default();
        trie.insert("airport");

        assert!(trie.has("airport"));
    }

    #[test]
    fn handles_multiple_words() {
        let mut trie = Trie::default();
        trie.insert("tea");
        trie.insert("test");

        assert!(trie.has("test"));
        assert!(trie.has("tea"));
    }

    #[test]
    fn handles_unicode() {
        let mut trie = Trie::default();
        trie.insert("camera");
        trie.insert("cámara");

        assert!(trie.has("camera"));
        assert!(trie.has("cámara"));
        assert!(!trie.has("cámera"));
    }

    #[test]
    fn handles_partial_search() {
        let mut trie = Trie::default();
        trie.insert("maker");

        assert!(!trie.has("make"));
    }

    #[test]
    fn gets_stored_value() {
        let mut trie = Trie::default();
        trie.insert("handler");

        assert_eq!(Some("handler".to_string()), trie.get("handler"));
    }

    #[test]
    fn gets_terminal_stored_value() {
        let mut trie = Trie::default();
        trie.insert("tea");
        trie.insert("test");

        assert_eq!(Some("tea".to_string()), trie.get("tea"));
        assert_eq!(Some("test".to_string()), trie.get("test"));
    }
}
