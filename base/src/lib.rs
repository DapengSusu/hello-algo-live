mod avl_tree;
pub use avl_tree::AvlTree;

mod binary_tree;
pub use binary_tree::{BinarySearchTree, BinaryTree};

mod heap;
pub use heap::{MaxHeap, MinHeap};

mod linked_list;
pub use linked_list::LinkedList;

mod queue;
pub use queue::{QueueWithArray, QueueWithList};

mod stack;
pub use stack::{StackWithList, StackWithVec};

mod top_k;
pub use top_k::top_k_heap;

pub mod bt {
    use std::collections::VecDeque;

    use crate::binary_tree::OptionNodeRc;

    /// 广度优先搜索（BFS），其思想与队列一致
    pub fn contains_bfs<T: PartialEq>(root: &OptionNodeRc<T>, val: &T) -> bool {
        let mut queue = VecDeque::new();
        if let Some(root) = root.as_ref() {
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
    pub fn contains_dfs<T: PartialEq>(root: &OptionNodeRc<T>, val: &T) -> bool {
        let mut stack = Vec::new();
        if let Some(root) = root.as_ref() {
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

    /// 深度优先搜索（DFS）- 递归实现
    pub fn contains<T: PartialEq>(root: &OptionNodeRc<T>, val: &T) -> bool {
        match root {
            None => false, // 空树或到达叶子节点
            Some(node) => {
                let node_ref = node.borrow();
                if &node_ref.value == val {
                    return true;
                }

                contains(&node_ref.left, val) || contains(&node_ref.right, val)
            }
        }
    }

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
mod tests {}
