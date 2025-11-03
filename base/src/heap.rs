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
    /// 获取堆顶元素（根节点）
    fn peek(&self) -> Option<&T>;

    /// 元素入堆
    fn push(&mut self, val: T);

    /// 元素出堆
    fn pop(&mut self) -> Option<T>;
}

/// 大顶堆，使用 Vec 实现
#[derive(Debug, Default)]
pub struct MaxHeap<T>(Vec<T>);

impl<T: PartialOrd> MaxHeap<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: PartialOrd> Heap<T> for MaxHeap<T> {
    fn peek(&self) -> Option<&T> {
        self.0.first()
    }

    fn push(&mut self, val: T) {
        // 添加节点
        self.0.push(val);
        // 从底至顶堆化（heapity）
        let len = self.len();
        sift_up(&mut self.0, len - 1, |a, b| a <= b);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 交换删除堆顶元素和堆底元素（首元素和尾元素）
        let val = self.0.swap_remove(0);

        // 从顶至底堆化
        sift_down(&mut self.0, 0, |a, b| a > b);

        Some(val)
    }
}

impl<T: PartialOrd, const N: usize> From<[T; N]> for MaxHeap<T> {
    fn from(v: [T; N]) -> Self {
        let mut heap = MaxHeap::new();

        v.into_iter().for_each(|val| {
            heap.push(val);
        });

        heap
    }
}

// 从节点 i 开始，从底至顶堆化
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

// 从节点 i 开始，从顶至底堆化
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

/// 小顶堆
#[derive(Debug, Default)]
pub struct MinHeap<T: Ord>(Vec<T>);
// 也可以使用 Reverse<T> 来简化实现
// pub struct MinHeap<T: Ord>(Vec<Reverse<T>>);

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord> Heap<T> for MinHeap<T> {
    fn peek(&self) -> Option<&T> {
        self.0.first()
    }

    fn push(&mut self, val: T) {
        // 添加节点
        self.0.push(val);
        // 从底至顶堆化（heapity）
        let len = self.len();
        sift_up(&mut self.0, len - 1, |a, b| a >= b);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 交换删除堆顶元素和堆底元素（首元素和尾元素）
        let val = self.0.swap_remove(0);

        // 从顶至底堆化
        sift_down(&mut self.0, 0, |a, b| a < b);

        Some(val)
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
    fn max_heap_from_vec_should_work() {
        let mut heap = MaxHeap::from([1, 2, 5, 3, 4]);

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
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
}
