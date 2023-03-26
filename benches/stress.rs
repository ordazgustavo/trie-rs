use criterion::{criterion_group, criterion_main, Criterion};

const WORDS: &str = include_str!("./furigana.txt");

pub fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stress");

    group.bench_function("Trie", |b| {
        b.iter(|| {
            let mut trie = trie_rs::Trie::default();
            for x in WORDS.lines() {
                trie.insert(x);
            }

            if trie.has("なれっじ・まねじめんと") {
                trie.get("なれっじ・まねじめんと");
            }

            trie
        })
    });

    group.bench_function("HashMap", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            if map.contains_key("なれっじ・まねじめんと") {
                map.get("なれっじ・まねじめんと");
            }

            map
        })
    });

    group.bench_function("BTreeMap", |b| {
        b.iter(|| {
            let mut map = std::collections::BTreeMap::new();
            for x in WORDS.lines() {
                map.insert(x, x);
            }

            if map.contains_key("なれっじ・まねじめんと") {
                map.get("なれっじ・まねじめんと");
            }

            map
        })
    });

    group.finish();
}

criterion_group!(benches, insert);
criterion_main!(benches);
