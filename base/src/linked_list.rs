use std::{
    fmt::{self, Display},
    marker::PhantomData,
    mem,
    ptr::NonNull,
};

#[derive(Debug)]
struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    elem: T,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            prev: None,
            next: None,
            elem: T::default(),
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.elem, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.elem),
        }
    }
}

/// 简易链表实现
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    /// 创建空链表
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// 向链表头部插入一个元素
    pub fn push_front(&mut self, t: T) {
        self.push(t, true);
    }

    /// 向链表尾部插入一个元素
    pub fn push_back(&mut self, t: T) {
        self.push(t, false);
    }

    /// 移除链表头部元素并将其返回
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop(true)
    }

    /// 移除链表尾部元素并将其返回
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop(false)
    }

    /// 返回链表头部元素的不可变借用
    pub fn front(&self) -> Option<&T> {
        self.head
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    /// 返回链表尾部元素的不可变借用
    pub fn back(&self) -> Option<&T> {
        self.tail
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    /// 返回链表头部元素的可变借用
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    /// 返回链表尾部元素的可变借用
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    /// 返回不可变迭代器
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }

    /// 返回可变的代器
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.head,
            _marker: PhantomData,
        }
    }

    /// 检查链表中是否包含指定元素，包含返回 true，否则返回 false
    pub fn contains(&self, t: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head;
        while let Some(node_ptr) = current {
            unsafe {
                if &(*node_ptr.as_ptr()).elem == t {
                    return true;
                }
                current = (*node_ptr.as_ptr()).next;
            }
        }

        false
    }

    /// 将链表反转
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base::LinkedList;
    ///
    /// let mut list = LinkedList::from([2, 4, 6, 8, 0]);
    ///
    /// list.reverse();
    /// assert_eq!(list, LinkedList::from([0, 8, 6, 4, 2]));
    /// ```
    pub fn reverse(&mut self) {
        if self.len <= 1 {
            return;
        }

        let mut current = self.head;
        // 遍历所有结点，交换 next 和 prev
        while let Some(node_ptr) = current {
            let node_ptr_raw = node_ptr.as_ptr();

            unsafe {
                // 暂时保存当前结点的 next 结点
                let node_next = (*node_ptr_raw).next;
                // 交换 next 和 prev 指针
                mem::swap(&mut (*node_ptr_raw).next, &mut (*node_ptr_raw).prev);
                // 推进到下一结点
                current = node_next;
            }
        }

        // 交换链表的 head 和 tail 指针
        mem::swap(&mut self.head, &mut self.tail);
    }

    /// 将 other 中的全部元素移动到链表尾部，完成后 other 为空
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail_ptr) => {
                if let Some(mut head_other) = other.head.take() {
                    // 这里使用 `as_mut` 是可行的，因为我们拥有
                    // 对两个链表全部内容的独占的访问权限
                    unsafe {
                        tail_ptr.as_mut().next = Some(head_other);
                        head_other.as_mut().prev = Some(tail_ptr);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    /// 将链表从指定位置一分为二，返回 at 及 at 之后所有元素组成的新链表。
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base::LinkedList;
    ///
    /// let mut list = LinkedList::from([2, 4, 6, 8, 0]);
    /// let mut split = list.split_off(2);
    ///
    /// assert_eq!(split, LinkedList::from([6, 8, 0]));
    /// ```
    pub fn split_off(&mut self, at: usize) -> LinkedList<T> {
        let len = self.len();

        assert!(at <= len, "Cannot split off at a nonexistent index");
        if at == 0 {
            return mem::take(self);
        } else if at == len {
            return Self::new();
        }

        let split_node = {
            let mut iter = self.iter_mut();
            // 手动跳过前 at-1 个结点，不要使用 `.skip()`，避免依赖 Skip
            for _ in 0..at - 1 {
                iter.next();
            }
            iter.current
        };

        // split_node 是第一部分的新 tail 结点，它也包含第二部分的 head 结点。
        if let Some(mut split_node) = split_node {
            let second_part_head;
            let second_part_tail;

            unsafe {
                second_part_head = split_node.as_mut().next.take();
            }
            if let Some(mut head) = second_part_head {
                unsafe {
                    head.as_mut().prev = None;
                }
                second_part_tail = self.tail;
            } else {
                second_part_tail = None;
            }

            let second_part = LinkedList {
                head: second_part_head,
                tail: second_part_tail,
                len: self.len - at,
            };

            // 更新第一部分的 tail 指针
            self.tail = Some(split_node);
            self.len = at;

            second_part
        } else {
            std::mem::take(self)
        }
    }

    /// 将链表转化为一个 Vec
    pub fn into_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }

    /// 清空链表内的全部元素
    pub fn clear(&mut self) {
        // 从链表头部开始，依次将裸指针转回 Box，触发 Box 的析构函数，从而安全释放内存。

        // 取出链表头指针。take() 将 self.head 置为 None，确保链表结构被清空。
        let mut current = self.head.take();
        // 遍历所有结点
        while let Some(node_ptr) = current {
            let node_ptr_raw = node_ptr.as_ptr();
            // Safety:
            // 1. 我们正在 drop 链表，保证了对该内存的所有权是唯一的；
            // 2. Box::from_raw(node_ptr_raw) 恢复了 Rust 对该堆内存的所有权。
            let mut node = unsafe { Box::from_raw(node_ptr_raw) };

            // 移动到下一个结点
            // 必须使用 take() 将其从当前结点中移出，这样 current 才能指向下一个结点。
            current = node.next.take();

            // 当 node（Box） 离开作用域时会被 Rust 自动 drop
        }

        self.tail.take();
        self.len = 0;
    }

    /// 从前往后获取指定位置元素的不可变借用，如果 index 无效返回 None
    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_node(index)
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    /// 从前往后获取指定位置元素的可变借用，如果 index 无效返回 None
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_node(index)
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    /// 返回链表中元素数量
    pub fn len(&self) -> usize {
        self.len
    }

    /// 判断链表是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> LinkedList<T> {
    #[inline]
    fn new_node(&self, t: T, is_front: bool) -> Option<NonNull<Node<T>>> {
        let node = Box::new(Node {
            prev: if is_front { None } else { self.tail },
            next: if is_front { self.head } else { None },
            elem: t,
        });

        Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) })
    }

    #[inline]
    fn push(&mut self, t: T, is_front: bool) {
        let node_ptr = self.new_node(t, is_front);

        if is_front {
            match self.head {
                None => self.tail = node_ptr,
                Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
            }
            self.head = node_ptr;
        } else {
            match self.tail {
                None => self.head = node_ptr,
                Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
            }
            self.tail = node_ptr;
        }
        self.len += 1;
    }

    #[inline]
    fn pop(&mut self, is_front: bool) -> Option<T> {
        if is_front {
            self.head.map(|head_ptr| {
                let node_ptr_raw = head_ptr.as_ptr();
                // Safety:
                // 使用 Box 接管这个结点的内存，取走值后销毁这个结点
                let node = unsafe { Box::from_raw(node_ptr_raw) };

                match node.next {
                    Some(next_ptr) => {
                        unsafe {
                            (*next_ptr.as_ptr()).prev = None;
                        }
                        self.head = Some(next_ptr);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                self.len -= 1;
                node.elem
            })
        } else {
            self.tail.map(|tail_ptr| {
                let node_ptr_raw = tail_ptr.as_ptr();
                // Safety:
                // 使用 Box 接管这个结点的内存，取走值后销毁这个结点
                let node = unsafe { Box::from_raw(node_ptr_raw) };

                match node.prev {
                    Some(prev_ptr) => {
                        unsafe {
                            (*prev_ptr.as_ptr()).next = None;
                        }
                        self.tail = Some(prev_ptr);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                self.len -= 1;
                node.elem
            })
        }
    }

    #[inline]
    fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        if index >= self.len {
            return None;
        }

        let mut current = self.head;
        for _ in 0..index {
            // Safety: 这里的 index 一定在有效范围内
            let node_ptr = current.unwrap();

            current = unsafe { (*node_ptr.as_ptr()).next }
        }

        current
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned())
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(v: [T; N]) -> Self {
        Self::from_iter(v)
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_iter(v)
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();

        list.extend(iter);
        list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<'a, T: 'a + Copy> Extend<&'a T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

/// 消耗所有权的迭代器
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct Iter<'a, T: 'a> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node_ptr| {
            let node = unsafe { &(*node_ptr.as_ptr()) };

            self.current = node.next;
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node_ptr| {
            let node = unsafe { &mut (*node_ptr.as_ptr()) };

            self.current = node.next;
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_basics_should_work() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.is_empty());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);

        assert_eq!(list.front(), Some(&1));
        if let Some(val) = list.front_mut() {
            *val += 5;
        }
        assert_eq!(list.front(), Some(&6));

        assert_eq!(list.back(), Some(&3));
        if let Some(val) = list.back_mut() {
            *val += 5;
        }
        assert_eq!(list.back(), Some(&8));

        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_back(), Some(8));

        list.push_front(4);
        list.push_front(8);
        list.push_front(10);

        assert_eq!(list.len(), 4);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&2));

        assert_eq!(list.get(0), list.front());
        assert_eq!(list.get(1), Some(&8));
        assert_eq!(list.get_mut(4), None);
        *(list.get_mut(2).unwrap()) = 5;
        assert_eq!(list.get(2), Some(&5));

        assert!(list.contains(&8));
        assert!(!list.contains(&9));

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        list.push_front(5);
        assert_eq!(list.front(), Some(&5));
        assert_eq!(list.back(), Some(&5));
    }

    #[test]
    fn list_iterator_should_work() {
        let mut list = LinkedList::new();

        list.push_back(1);
        list.push_back(3);
        list.push_back(5);
        list.push_back(7);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), None);

        let mut iter_mut = list.iter_mut();

        *(iter_mut.next().unwrap()) += 1;
        assert_eq!(list.pop_front(), Some(2));

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(7));
        assert_eq!(iter.next(), Some(5));
    }

    #[test]
    fn list_reverse_should_work() {
        let mut list = LinkedList::from([2, 4, 6, 8, 0]);

        list.reverse();

        assert_eq!(list, LinkedList::from([0, 8, 6, 4, 2]));
    }

    #[test]
    fn list_append_should_work() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut list_src = LinkedList::from([5, 6, 7]);

        list.append(&mut list_src);

        assert_eq!(list.into_vec(), vec![1, 2, 3, 5, 6, 7]);
        assert!(list_src.is_empty());
    }

    #[test]
    fn list_split_off_should_work() {
        let mut list = LinkedList::from([1, 3, 5, 7, 9]);
        let mut split = list.split_off(3);

        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&5));

        assert_eq!(split.pop_front(), Some(7));
        assert_eq!(split.pop_front(), Some(9));
        assert_eq!(split.pop_front(), None);
    }
}
