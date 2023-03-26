#[derive(Default, Debug)]
struct Node<K, V> {
    children: Vec<Node<K, V>>,
    key: K,
    value: Option<V>,
    is_terminal: bool,
}

impl<K, V> Node<K, V> {
    fn new(key: K) -> Self {
        Self {
            children: Default::default(),
            key,
            value: Default::default(),
            is_terminal: false,
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie<K, V> {
    root: Node<K, V>,
}

impl<K, V> Trie<K, V>
where
    K: Ord + Clone,
    V: AsRef<[K]>,
{
    pub fn insert(&mut self, value: V) {
        let mut node = &mut self.root;

        for ch in value.as_ref() {
            match node.children.binary_search_by(|n| n.key.cmp(ch)) {
                Ok(i) => {
                    node = &mut node.children[i];
                }
                Err(i) => {
                    node.children.insert(i, Node::new(ch.clone()));
                    node = &mut node.children[i];
                }
            };
        }

        node.is_terminal = true;
        node.value.replace(value);
    }

    pub fn has(&self, value: V) -> bool {
        let mut node = &self.root;

        for ch in value.as_ref() {
            match node.children.binary_search_by(|n| n.key.cmp(ch)) {
                Ok(i) => {
                    node = &node.children[i];
                }
                Err(_) => return false,
            };
        }

        return node.is_terminal;
    }

    pub fn has_prefix(&self, value: V) -> bool {
        let mut node = &self.root;

        for ch in value.as_ref() {
            match node.children.binary_search_by(|n| n.key.cmp(ch)) {
                Ok(i) => {
                    node = &node.children[i];
                }
                Err(_) => return false,
            };
        }

        return true;
    }

    pub fn get(&self, value: V) -> Option<&V> {
        let mut node = &self.root;

        for ch in value.as_ref() {
            match node.children.binary_search_by(|n| n.key.cmp(ch)) {
                Ok(i) => {
                    node = &node.children[i];
                }
                Err(_) => return None,
            };
        }

        if !node.is_terminal {
            return None;
        }

        node.value.as_ref()
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
        trie.insert("🫡");

        assert!(trie.has("camera"));
        assert!(trie.has("cámara"));
        assert!(!trie.has("cámera"));
        assert!(trie.has("🫡"));
    }

    #[test]
    fn handles_duplicated() {
        let mut trie = Trie::default();
        trie.insert("camera");
        trie.insert("camera");

        assert!(trie.has("camera"));
    }

    #[test]
    fn handles_partial_search() {
        let mut trie = Trie::default();
        trie.insert("maker");

        assert!(!trie.has("ma"));
        assert!(trie.has_prefix("ma"));
    }

    #[test]
    fn handles_large_data_set() {
        let words = include_str!("../benches/words.txt");
        let mut trie = Trie::default();

        for x in words.lines() {
            trie.insert(x);
        }

        assert!(trie.has("matter"));
    }

    #[test]
    fn gets_stored_value() {
        let mut trie = Trie::default();
        trie.insert("handler");

        assert_eq!(Some(&"handler"), trie.get("handler"));
    }

    #[test]
    fn gets_terminal_stored_value() {
        let mut trie = Trie::default();
        trie.insert("tea");
        trie.insert("test");

        assert_eq!(Some(&"tea"), trie.get("tea"));
        assert_eq!(Some(&"test"), trie.get("test"));
    }
}
