use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub type OptionNodeRc<T> = Option<Rc<RefCell<TreeNode<T>>>>;
pub type NodeRc<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub left: OptionNodeRc<T>,
    pub right: OptionNodeRc<T>,
    pub value: T,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            left: None,
            right: None,
            value: val,
        }
    }

    pub fn new_node_rc(val: T) -> NodeRc<T> {
        Rc::new(RefCell::new(Self::new(val)))
    }
}

impl<T: Default> Default for TreeNode<T> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            value: T::default(),
        }
    }
}

/// 简单二叉树
pub struct BinaryTree<T> {
    pub root: OptionNodeRc<T>,
}

impl<T> BinaryTree<T> {
    /// 创建空二叉树
    pub fn new() -> Self {
        Self { root: None }
    }

    /// 无序插入，使用层序遍历找到第一个空位置
    pub fn insert(&mut self, val: T) {
        let new_node = TreeNode::new_node_rc(val);
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut queue = VecDeque::new();

        // Safety: 这里使用 unwrap() 是安全的，root 一定不是 None
        queue.push_back(self.root.as_ref().unwrap().clone());

        while let Some(current) = queue.pop_front() {
            let mut current_borrowed = current.borrow_mut();
            if current_borrowed.left.is_none() {
                current_borrowed.left = Some(new_node);
                return;
            } else if current_borrowed.right.is_none() {
                current_borrowed.right = Some(new_node);
                return;
            } else {
                // Safety: 同上，这里使用 unwrap() 是安全的
                queue.push_back(current_borrowed.left.as_ref().unwrap().clone());
                queue.push_back(current_borrowed.right.as_ref().unwrap().clone());
            }
        }
    }
}

impl<T: PartialEq> BinaryTree<T> {
    /// 广度优先搜索（BFS），其思想与队列一致
    pub fn contains_bfs(&self, val: &T) -> bool {
        let mut queue = VecDeque::new();
        if let Some(root) = self.root.as_ref() {
            queue.push_back(root.clone());
        }

        while let Some(node) = queue.pop_front() {
            if &node.borrow().value == val {
                return true;
            }

            if let Some(left) = node.borrow().left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = node.borrow().right.as_ref() {
                queue.push_back(right.clone());
            }
        }

        false
    }

    /// 深度优先搜索（DFS）
    pub fn contains_dfs(&self, val: &T) -> bool {
        let mut stack = Vec::new();
        if let Some(root) = self.root.as_ref() {
            stack.push(root.clone());
        }

        while let Some(node) = stack.pop() {
            if &node.borrow().value == val {
                return true;
            }

            if let Some(left) = node.borrow().left.as_ref() {
                stack.push(left.clone());
            }

            if let Some(right) = node.borrow().right.as_ref() {
                stack.push(right.clone());
            }
        }

        false
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> From<[T; N]> for BinaryTree<T> {
    fn from(v: [T; N]) -> Self {
        let mut tree = BinaryTree::new();

        v.into_iter().for_each(|val| {
            tree.insert(val);
        });

        tree
    }
}

pub mod dfs {
    use crate::binary_tree::OptionNodeRc;

    /// 前序遍历
    pub fn pre_order<T: Clone>(root: &OptionNodeRc<T>) -> Vec<T> {
        let mut ordered = Vec::new();

        pre_order_recursive(root, &mut ordered);

        ordered
    }

    /// 中序遍历
    pub fn in_order<T: Clone>(root: &OptionNodeRc<T>) -> Vec<T> {
        let mut ordered = Vec::new();

        in_order_recursive(root, &mut ordered);

        ordered
    }

    /// 后序遍历
    pub fn post_order<T: Clone>(root: &OptionNodeRc<T>) -> Vec<T> {
        let mut ordered = Vec::new();

        post_order_recursive(root, &mut ordered);

        ordered
    }

    fn pre_order_recursive<T: Clone>(root: &OptionNodeRc<T>, ordered: &mut Vec<T>) {
        if let Some(node) = root {
            ordered.push(node.borrow().value.clone());
            pre_order_recursive(&node.borrow().left, ordered);
            pre_order_recursive(&node.borrow().right, ordered);
        }
    }

    fn in_order_recursive<T: Clone>(root: &OptionNodeRc<T>, ordered: &mut Vec<T>) {
        if let Some(node) = root {
            in_order_recursive(&node.borrow().left, ordered);
            ordered.push(node.borrow().value.clone());
            in_order_recursive(&node.borrow().right, ordered);
        }
    }

    fn post_order_recursive<T: Clone>(root: &OptionNodeRc<T>, ordered: &mut Vec<T>) {
        if let Some(node) = root {
            post_order_recursive(&node.borrow().left, ordered);
            post_order_recursive(&node.borrow().right, ordered);
            ordered.push(node.borrow().value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_basics_should_work() {
        let mut tree = BinaryTree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert!(tree.contains_bfs(&2));
        assert!(tree.contains_bfs(&3));
        assert!(!tree.contains_bfs(&5));

        assert!(tree.contains_dfs(&2));
        assert!(tree.contains_dfs(&3));
        assert!(!tree.contains_dfs(&5));
    }

    fn new_binary_tree() -> BinaryTree<i32> {
        // *******1*******
        // ****2*****3****
        // **4***5*6******
        BinaryTree::from([1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn tree_pre_order_should_work() {
        let tree = new_binary_tree();

        assert_eq!(dfs::pre_order(&tree.root), vec![1, 2, 4, 5, 3, 6]);
    }

    #[test]
    fn tree_in_order_should_work() {
        let tree = new_binary_tree();

        assert_eq!(dfs::in_order(&tree.root), vec![4, 2, 5, 1, 6, 3]);
    }

    #[test]
    fn tree_post_order_should_work() {
        let tree = new_binary_tree();

        assert_eq!(dfs::post_order(&tree.root), vec![4, 5, 2, 6, 3, 1]);
    }
}
