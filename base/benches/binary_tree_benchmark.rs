use std::hint::black_box;

use base::{BinaryTree, dfs};

use criterion::{Criterion, criterion_group, criterion_main};

fn new_tree() -> BinaryTree<i32> {
    black_box(BinaryTree::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
}

// fn pre_order_benchmark(c: &mut Criterion) {
//     let tree = new_tree();

//     c.bench_function("pre-order", |b| b.iter(|| dfs::pre_order(&tree.root)));
// }

// fn in_order_benchmark(c: &mut Criterion) {
//     let tree = new_tree();

//     c.bench_function("in-order", |b| b.iter(|| dfs::in_order(&tree.root)));
// }

fn post_order_benchmark(c: &mut Criterion) {
    let tree = new_tree();

    c.bench_function("post-order", |b| b.iter(|| dfs::post_order(&tree.root)));
}

criterion_group!(
    benches,
    // pre_order_benchmark,
    // in_order_benchmark,
    post_order_benchmark
);
criterion_main!(benches);
