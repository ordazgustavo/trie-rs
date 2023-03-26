use criterion::{criterion_group, criterion_main, Criterion};

const WORDS: &str = include_str!("./words.txt");

pub fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert");

    group.bench_function("Trie::insert()", |b| {
        b.iter(|| {
            let mut trie = trie_rs::Trie::default();
            for x in WORDS.lines() {
                trie.insert(x);
            }
            trie
        })
    });

    group.bench_function("HashMap::insert()", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }
            map
        })
    });

    group.bench_function("BTreeMap::insert()", |b| {
        b.iter(|| {
            let mut map = std::collections::BTreeMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }
            map
        })
    });

    group.finish();
}

criterion_group!(benches, insert);
criterion_main!(benches);
