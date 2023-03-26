use criterion::{criterion_group, criterion_main, Criterion};

const WORDS: &str = include_str!("./words.txt");

pub fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get");

    group.bench_function("Trie::get()", |b| {
        b.iter(|| {
            let mut trie = trie_rs::Trie::default();
            for x in WORDS.lines() {
                trie.insert(x);
            }

            assert_eq!(Some(&"matter"), trie.get("matter"));
        })
    });

    group.bench_function("HashMap::get()", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            assert_eq!(Some(&"matter"), map.get("matter"));
        })
    });

    group.bench_function("BTreeMap::get()", |b| {
        b.iter(|| {
            let mut map = std::collections::BTreeMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            assert_eq!(Some(&"matter"), map.get("matter"));
        })
    });

    group.finish();
}

criterion_group!(benches, get);
criterion_main!(benches);
