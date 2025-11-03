use std::{
    collections::LinkedList,
    ops::{Deref, DerefMut},
};

/// 基于链表实现的队列
#[derive(Debug, Default, Clone)]
pub struct QueueWithList<T>(LinkedList<T>);

impl<T> Deref for QueueWithList<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for QueueWithList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> QueueWithList<T> {
    pub fn new() -> Self {
        QueueWithList(LinkedList::new())
    }

    pub fn push(&mut self, elem: T) {
        self.push_back(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.front()
    }

    pub fn tail(&self) -> Option<&T> {
        self.back()
    }
}

impl<T: Clone> QueueWithList<T> {
    pub fn to_vec(&self) -> Vec<T> {
        self.0.iter().cloned().collect()
    }
}

/// 基于数组实现的队列
#[derive(Debug, Clone)]
pub struct QueueWithArray<T, const N: usize> {
    queue: [Option<T>; N],
    front: usize,
    len: usize,
    cap: usize,
}

impl<T: Copy, const N: usize> Default for QueueWithArray<T, N> {
    fn default() -> Self {
        Self {
            queue: [None; N],
            front: 0,
            len: 0,
            cap: N,
        }
    }
}

impl<T, const N: usize> QueueWithArray<T, N> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            panic!("push failed, queue is full");
        }
        let real = (self.front + self.len) % self.cap;

        self.queue[real] = Some(elem);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let front = self.queue[self.front].take();

        self.front = (self.front + 1) % self.cap;
        self.len -= 1;

        front
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.queue[self.front].as_ref()
    }

    pub fn tail(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let real = (self.front + self.len) % self.cap;

        self.queue[real - 1].as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }
}

impl<T: Copy, const N: usize> QueueWithArray<T, N> {
    pub fn new() -> Self {
        QueueWithArray {
            queue: [None; N],
            front: 0,
            len: 0,
            cap: N,
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        (0..self.len)
            .filter_map(|i| self.queue[(self.front + i) % self.cap])
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_with_list_basics() {
        let mut queue = QueueWithList::new();

        assert!(queue.is_empty());
        assert_eq!(queue.pop(), None);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.tail(), Some(&3));

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.tail(), Some(&3));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.to_vec(), vec![2, 3]);

        queue.pop();
        queue.pop();

        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn queue_with_vec_basics() {
        let mut queue = QueueWithArray::<i32, 5>::new();

        assert!(queue.is_empty());
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.tail(), None);
        assert_eq!(queue.pop(), None);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.tail(), Some(&3));
        assert_eq!(queue.to_vec(), vec![1, 2, 3]);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.tail(), Some(&3));
        assert_eq!(queue.to_vec(), vec![2, 3]);

        queue.push(4);
        queue.push(5);
        queue.push(6);

        // queue.push(7); // panic
        // 现在队列已经满了
        assert_eq!(queue.len(), 5);
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.tail(), Some(&6));
        assert_eq!(queue.to_vec(), vec![2, 3, 4, 5, 6]);

        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.len(), 4);
        assert_eq!(queue.peek(), Some(&3));
        assert_eq!(queue.tail(), Some(&6));
        assert_eq!(queue.to_vec(), vec![3, 4, 5, 6]);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), None);
    }
}
