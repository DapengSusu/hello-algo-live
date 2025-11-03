use std::hint::black_box;

use base::{BinaryTree, bt};

use criterion::{Criterion, criterion_group, criterion_main};

fn new_tree() -> BinaryTree<i32> {
    black_box(BinaryTree::from([2, 1, 4, 3, 6, 5, 8, 7, 0, 9]))
}

fn bfs_benchmark(c: &mut Criterion) {
    let tree = new_tree();

    c.bench_function("BFS 广度优先搜索", |b| {
        b.iter(|| bt::contains_bfs(&tree.root, &6))
    });
}

fn dfs_benchmark(c: &mut Criterion) {
    let tree = new_tree();

    c.bench_function("DFS 深度优先搜索", |b| {
        b.iter(|| bt::contains_dfs(&tree.root, &6))
    });
}

fn dfs_rec_benchmark(c: &mut Criterion) {
    let tree = new_tree();

    c.bench_function("DFS 深度优先搜索（递归）", |b| {
        b.iter(|| bt::contains(&tree.root, &6))
    });
}

criterion_group!(benches, bfs_benchmark, dfs_benchmark, dfs_rec_benchmark);
criterion_main!(benches);
