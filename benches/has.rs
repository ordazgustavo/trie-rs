use criterion::{criterion_group, criterion_main, Criterion};

const WORDS: &str = include_str!("./words.txt");

pub fn has(c: &mut Criterion) {
    let mut group = c.benchmark_group("Has");

    group.bench_function("Trie::has()", |b| {
        b.iter(|| {
            let mut trie = trie_rs::Trie::default();
            for x in WORDS.lines() {
                trie.insert(x);
            }

            assert!(trie.has("matter"));
        })
    });

    group.bench_function("HashMap::contains_key()", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            assert!(map.contains_key("matter"));
        })
    });

    group.bench_function("BTreeMap::contains_key()", |b| {
        b.iter(|| {
            let mut map = std::collections::BTreeMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            assert!(map.contains_key("matter"));
        })
    });

    group.finish();
}

criterion_group!(benches, has);
criterion_main!(benches);
