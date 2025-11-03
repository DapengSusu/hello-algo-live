use std::{
    collections::LinkedList,
    ops::{Deref, DerefMut},
};

/// 基于链表实现的栈
#[derive(Debug, Default, Clone)]
pub struct StackWithList<T>(LinkedList<T>);

impl<T> Deref for StackWithList<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StackWithList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> StackWithList<T> {
    pub fn new() -> Self {
        StackWithList(LinkedList::new())
    }

    pub fn push(&mut self, elem: T) {
        self.push_back(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }

    pub fn peek(&self) -> Option<&T> {
        self.back()
    }
}

impl<T: Clone> StackWithList<T> {
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

/// 基于动态数组实现的栈
#[derive(Debug, Default, Clone)]
pub struct StackWithVec<T>(Vec<T>);

impl<T> Deref for StackWithVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StackWithVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> StackWithVec<T> {
    pub fn new() -> Self {
        StackWithVec(Vec::new())
    }

    pub fn peek(&self) -> Option<&T> {
        self.last()
    }
}

impl<T: Clone> StackWithVec<T> {
    pub fn to_vec(&self) -> Vec<T> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_with_list_basics() {
        let mut stack = StackWithList::new();

        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3)); // pop 3
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.to_vec(), vec![1, 2]);

        stack.pop(); // pop 2
        stack.pop(); // pop 1

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_with_vec_basics() {
        let mut stack = StackWithVec::new();

        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3)); // pop 3
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.to_vec(), vec![1, 2]);

        stack.pop(); // pop 2
        stack.pop(); // pop 1

        assert_eq!(stack.pop(), None);
    }
}
