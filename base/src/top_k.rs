//! TOP-K 问题
//! 给定一个长度为 n 的无序数组 nums，请返回数组中最大的 k 个元素。
//!
//! 堆解法：
//! 1. 初始化一个小顶堆，其堆顶元素最小
//! 2. 先将数组的前 k 个元素依次入堆
//! 3. 从第 k+1 个元素开始，若当前元素大于堆顶元素，则将堆顶元素出堆，并将当前元素入堆
//! 4. 遍历完成后，堆中保存的就是最大的 k 个元素
//!
//! 时间复杂度：O(nlogk)
//! 当 k 较小时，时间复杂度趋向于 O(n)，当 k 较大时，时间复杂度不会超过 O(nlogn)

use std::{cmp::Reverse, collections::BinaryHeap};

/// 基于堆查找数组中最大的 k 个元素
pub fn top_k_heap<T, I>(nums: I, k: usize) -> BinaryHeap<Reverse<T>>
where
    T: Ord,
    I: IntoIterator<Item = T>,
{
    // BinaryHeap 是大顶堆，使用 Reverse 将元素取反，从而实现小顶堆
    let mut heap = BinaryHeap::<Reverse<T>>::new();

    for num in nums.into_iter() {
        // 数组的前 k 个元素入堆
        if k > heap.len() {
            heap.push(Reverse(num));
        } else if num > heap.peek().unwrap().0 {
            heap.pop();
            heap.push(Reverse(num));
        }
    }

    heap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_heap_should_work() {
        let mut max_result = top_k_heap([2, 5, 3, 7, 3, 6, 1], 4);

        assert_eq!(max_result.pop().unwrap().0, 3);
        assert_eq!(max_result.pop().unwrap().0, 5);
        assert_eq!(max_result.pop().unwrap().0, 6);
        assert_eq!(max_result.pop().unwrap().0, 7);
        assert_eq!(max_result.pop(), None);
    }
}
