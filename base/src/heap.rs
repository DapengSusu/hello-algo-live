//! 堆（heap）是满足特定条件的完全二叉树，主要分为大顶堆和小顶堆：
//! * 大顶堆：任意节点的值大于等于其子节点的值
//! * 小顶堆：任意节点的值小于等于其子节点的值
//!
//! 具有如下特点：
//! * 最底层节点靠左填充，其它层的节点都被填满；
//! * 二叉树的根节点称为“堆顶”，最底层靠右的节点称为“堆底”
//! * 对于大顶堆（小顶堆），堆顶元素（根节点）的值最大（最小）
//!
//! 通常使用堆来实现优先队列（priority queue），这是一种具有优先级排序的队列。
//! 大顶堆相当于元素按从大到小的顺序出队的优先队列。
//!
//! Rust 中 std::collections 中提供了 BinaryHeap，这是一个大顶堆的实现，
//! 可以通过使用 std::cmp::Reverse 实现小顶堆。

#![allow(dead_code)]

trait Heap<T> {
    /// 关联大顶堆或小顶堆
    type HeapTp;

    /// 从列表中构建堆（建堆操作）
    fn from_vec<I>(v: I) -> Self::HeapTp
    where
        I: Into<Vec<T>>;

    /// 获取堆顶元素（根节点）
    fn peek(&self) -> Option<&T>;

    /// 元素入堆
    fn push(&mut self, val: T);

    /// 元素出堆
    fn pop(&mut self) -> Option<T>;

    /// 堆中元素数量
    fn len(&self) -> usize;

    /// 判断堆是否为空
    fn is_empty(&self) -> bool;
}

/// 大顶堆，使用 Vec 实现
#[derive(Debug, Default)]
pub struct MaxHeap<T>(Vec<T>);

impl<T: PartialOrd> MaxHeap<T> {
    /// 创建一个空的 MaxHeap
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: PartialOrd> Heap<T> for MaxHeap<T> {
    type HeapTp = MaxHeap<T>;

    // 时间复杂度：O(n)
    fn from_vec<I>(v: I) -> Self::HeapTp
    where
        I: Into<Vec<T>>,
    {
        // 将列表元素直接放进堆中
        let mut heap = MaxHeap(v.into());
        // 堆化除叶节点外的其它节点
        for i in (0..=parent(heap.len() - 1)).rev() {
            sift_down_max(&mut heap.0, i);
        }

        heap
    }

    fn peek(&self) -> Option<&T> {
        self.0.first()
    }

    // 时间复杂度：O(logn)
    fn push(&mut self, val: T) {
        // 添加节点
        self.0.push(val);
        // 从底至顶堆化（heapity）
        let len = self.len();
        sift_up_max(&mut self.0, len - 1);
    }

    // 时间复杂度：O(logn)
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 交换删除堆顶元素和堆底元素（首元素和尾元素）
        let val = self.0.swap_remove(0);

        // 从顶至底堆化
        sift_down_max(&mut self.0, 0);

        Some(val)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: PartialOrd> From<Vec<T>> for MaxHeap<T> {
    fn from(v: Vec<T>) -> Self {
        MaxHeap::from_vec(v)
    }
}

impl<T: PartialOrd, const N: usize> From<[T; N]> for MaxHeap<T> {
    fn from(v: [T; N]) -> Self {
        MaxHeap::from_vec::<[T; N]>(v)
    }
}

/// 大顶堆的从底至顶堆化
fn sift_up_max<T: PartialOrd>(v: &mut [T], i: usize) {
    sift_up(v, i, |a, b| a <= b);
}

/// 大顶堆的从顶至底堆化
fn sift_down_max<T: PartialOrd>(v: &mut [T], i: usize) {
    sift_down(v, i, |a, b| a > b);
}

/// 小顶堆
#[derive(Debug, Default)]
pub struct MinHeap<T: PartialOrd>(Vec<T>);
// 也可以使用 Reverse<T> 来简化实现
// pub struct MinHeap<T: Ord>(Vec<Reverse<T>>);

impl<T: PartialOrd> MinHeap<T> {
    /// 创建一个空的 MinHeap
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: PartialOrd> Heap<T> for MinHeap<T> {
    type HeapTp = MinHeap<T>;

    fn from_vec<I>(v: I) -> Self::HeapTp
    where
        I: Into<Vec<T>>,
    {
        let mut heap = MinHeap(v.into());

        for i in (0..=heap.len()).rev() {
            sift_down_min(&mut heap.0, i);
        }

        heap
    }

    fn peek(&self) -> Option<&T> {
        self.0.first()
    }

    fn push(&mut self, val: T) {
        // 添加节点
        self.0.push(val);
        // 从底至顶堆化（heapity）
        let len = self.len();
        sift_up_min(&mut self.0, len - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 交换删除堆顶元素和堆底元素（首元素和尾元素）
        let val = self.0.swap_remove(0);

        // 从顶至底堆化
        sift_down_min(&mut self.0, 0);

        Some(val)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: PartialOrd> From<Vec<T>> for MinHeap<T> {
    fn from(v: Vec<T>) -> Self {
        MinHeap::from_vec(v)
    }
}

impl<T: PartialOrd, const N: usize> From<[T; N]> for MinHeap<T> {
    fn from(v: [T; N]) -> Self {
        MinHeap::from_vec::<[T; N]>(v)
    }
}

/// 小顶堆的从底至顶堆化
fn sift_up_min<T: PartialOrd>(v: &mut [T], i: usize) {
    sift_up(v, i, |a, b| a >= b);
}

/// 小顶堆的从顶至底堆化
fn sift_down_min<T: PartialOrd>(v: &mut [T], i: usize) {
    sift_down(v, i, |a, b| a < b);
}

/// 从节点 i 开始，从底至顶堆化
fn sift_up<T, F>(v: &mut [T], mut i: usize, cmp: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    loop {
        if i == 0 {
            // 节点 i 已经是堆顶节点了，结束堆化
            break;
        }
        // 获取节点 i 的父节点索引 p
        let p = parent(i);
        if cmp(&v[i], &v[p]) {
            // 该节点满足要求，结束堆化
            break;
        }
        // 交换两节点
        v.swap(i, p);
        // 循环向上堆化
        i = p;
    }
}

/// 从节点 i 开始，从顶至底堆化
fn sift_down<T, F>(v: &mut [T], mut i: usize, cmp: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    loop {
        // 判断节点 i，l，r 中值最大（小）的节点，记为 ext
        let (l, r, mut ext) = (left(i), right(i), i);
        if l < v.len() && cmp(&v[l], &v[ext]) {
            ext = l;
        }
        if r < v.len() && cmp(&v[r], &v[ext]) {
            ext = r;
        }
        // 若节点 i 最大（小）或 l，r 越界，则无需继续堆化，退出
        if ext == i {
            break;
        }
        // 交换两节点
        v.swap(i, ext);
        // 循环向下堆化
        i = ext;
    }
}

/// 获取左子节点的索引
fn left(i: usize) -> usize {
    2 * i + 1
}

/// 获取右子节点的索引
fn right(i: usize) -> usize {
    2 * i + 2
}

// 获取父节点的索引
fn parent(i: usize) -> usize {
    // 向下整除
    (i - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_index_should_work() {
        assert_eq!(parent(3), 1);
        assert_eq!(parent(4), 1);
        assert_eq!(parent(5), 2);
        assert_eq!(parent(6), 2);
    }

    #[test]
    fn max_heap_basics_should_work() {
        let mut heep = MaxHeap::new();

        heep.push(1);
        assert_eq!(heep.peek(), Some(&1));
        heep.push(3);
        assert_eq!(heep.peek(), Some(&3));
        heep.push(5);
        assert_eq!(heep.peek(), Some(&5));
        heep.push(4);
        assert_eq!(heep.peek(), Some(&5));
        heep.push(2);
        assert_eq!(heep.peek(), Some(&5));
        heep.push(6);
        assert_eq!(heep.peek(), Some(&6));

        assert_eq!(heep.pop(), Some(6));
        assert_eq!(heep.peek(), Some(&5));
        assert_eq!(heep.pop(), Some(5));
        assert_eq!(heep.pop(), Some(4));
        assert_eq!(heep.pop(), Some(3));
        assert_eq!(heep.pop(), Some(2));
        assert_eq!(heep.pop(), Some(1));
        assert_eq!(heep.peek(), None);
        assert_eq!(heep.pop(), None);
    }

    #[test]
    fn min_heap_basics_should_work() {
        let mut heep = MinHeap::new();

        heep.push(1);
        assert_eq!(heep.peek(), Some(&1));
        heep.push(3);
        assert_eq!(heep.peek(), Some(&1));
        heep.push(5);
        assert_eq!(heep.peek(), Some(&1));
        heep.push(4);
        assert_eq!(heep.peek(), Some(&1));
        heep.push(0);
        assert_eq!(heep.peek(), Some(&0));
        heep.push(2);
        assert_eq!(heep.peek(), Some(&0));

        assert_eq!(heep.pop(), Some(0));
        assert_eq!(heep.pop(), Some(1));
        assert_eq!(heep.pop(), Some(2));
        assert_eq!(heep.pop(), Some(3));
        assert_eq!(heep.pop(), Some(4));
        assert_eq!(heep.pop(), Some(5));
        assert_eq!(heep.pop(), None);
    }

    #[test]
    fn heap_from_vec_should_work() {
        let mut min_heap = MinHeap::from_vec([1, 2, 5, 3, 4, 2]);
        // MaxHeap 实现了 From trait
        let mut max_heap = MaxHeap::from([1, 2, 5, 3, 4, 2]);

        assert_eq!(min_heap.peek(), Some(&1));
        assert_eq!(max_heap.peek(), Some(&5));

        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(max_heap.pop(), Some(5));

        assert_eq!(min_heap.peek(), Some(&2));
        assert_eq!(max_heap.peek(), Some(&4));
    }
}
