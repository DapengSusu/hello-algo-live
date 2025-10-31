use std::{fmt::Display, marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    elem: T,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.elem, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.elem),
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, t: T) {
        self.push(t, true);
    }

    pub fn push_back(&mut self, t: T) {
        self.push(t, false);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop(true)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop(false)
    }

    pub fn front(&self) -> Option<&T> {
        self.head
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.head,
            _marker: PhantomData,
        }
    }

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

    pub fn into_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }

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

    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_node(index)
            .map(|node_ptr| unsafe { &(*node_ptr.as_ptr()).elem })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_node(index)
            .map(|node_ptr| unsafe { &mut (*node_ptr.as_ptr()).elem })
    }

    pub fn len(&self) -> usize {
        self.len
    }

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
                Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
                None => self.tail = node_ptr,
            }
            self.head = node_ptr;
        } else {
            match self.tail {
                Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
                None => self.head = node_ptr,
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

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
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
}
