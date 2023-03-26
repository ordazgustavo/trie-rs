# trie-rs

```rust
let mut trie = Trie::default();
trie.insert("caramel");
trie.insert("camera");
trie.insert("cámara");
trie.insert("🫡");

assert!(trie.has("caramel"));
assert!(trie.has_prefix("cám"));
```
