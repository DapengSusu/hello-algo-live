use std::{cmp::Ordering, rc::Rc};

use crate::{
    binary_tree::{OptionNodeRc, TreeNode},
    dfs,
};

// 二叉搜索树
pub struct BinarySearchTree<T> {
    pub root: OptionNodeRc<T>,
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<T: Clone + Ord> BinarySearchTree<T> {
    pub fn search(&self, target: &T) -> OptionNodeRc<T> {
        let mut current = self.root.clone();

        while let Some(node) = current.clone() {
            match target.cmp(&node.borrow().value) {
                Ordering::Equal => break,
                Ordering::Less => current = node.borrow().left.clone(),
                Ordering::Greater => current = node.borrow().right.clone(),
            }
        }

        current
    }

    pub fn insert(&mut self, val: T) {
        // 若树为空，则初始化根节点
        if self.root.is_none() {
            self.root = Some(TreeNode::new_node_rc(val));
            return;
        }

        let mut current = self.root.clone();
        let mut previous = None;

        while let Some(node) = current.clone() {
            match val.cmp(&node.borrow().value) {
                // 找到重复节点直接返回
                Ordering::Equal => return,
                Ordering::Less => {
                    previous = current;
                    current = node.borrow().left.clone();
                }
                Ordering::Greater => {
                    previous = current;
                    current = node.borrow().right.clone();
                }
            }
        }

        // Safety: 这里使用 unwrap() 是安全的，previous 一定不是 None
        let previous = previous.unwrap();
        if val > previous.borrow().value {
            previous.borrow_mut().right = Some(TreeNode::new_node_rc(val));
        } else {
            previous.borrow_mut().left = Some(TreeNode::new_node_rc(val));
        }
    }

    pub fn remove(&mut self, val: &T) {
        // 若树为空，则直接返回
        if self.root.is_none() {
            return;
        }

        let mut current = self.root.clone();
        let mut previous = None;

        while let Some(node) = current.clone() {
            match val.cmp(&node.borrow().value) {
                // 找到待删除节点
                Ordering::Equal => break,
                Ordering::Less => {
                    previous = current;
                    current = node.borrow().left.clone();
                }
                Ordering::Greater => {
                    previous = current;
                    current = node.borrow().right.clone();
                }
            }
        }

        // 若无待删除节点，则直接返回
        if current.is_none() {
            return;
        }
        // Safety: 这里使用 unwrap() 是安全的
        let current = current.unwrap();
        let (left_child, right_child) = (
            current.borrow().left.clone(),
            current.borrow().right.clone(),
        );
        match (left_child.clone(), right_child.clone()) {
            // 待删除节点的子节点数量为0或1
            (None, None) | (Some(_), None) | (None, Some(_)) => {
                let child = left_child.or(right_child);
                if Rc::ptr_eq(&current, self.root.as_ref().unwrap()) {
                    // 删除的节点为根节点
                    self.root = child;
                } else {
                    let prev = previous.unwrap();
                    let left = prev.borrow().left.clone();
                    if left.is_some() && Rc::ptr_eq(left.as_ref().unwrap(), &current) {
                        prev.borrow_mut().left = child;
                    } else {
                        prev.borrow_mut().right = child;
                    }
                }
            }
            // 待删除节点的子节点数量为2
            (Some(_), Some(_)) => {
                // 获取中序遍历中 current 的下一个节点
                let mut next = current.borrow().right.clone();
                while let Some(node) = next.clone() {
                    if node.borrow().left.is_some() {
                        next = node.borrow().left.clone();
                    } else {
                        break;
                    }
                }
                let next_val = next.unwrap().borrow().value.clone();
                // 递归删除节点 next
                self.remove(&next_val);
                // 用 next 覆盖 current
                current.borrow_mut().value = next_val;
            }
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        dfs::in_order(&self.root)
    }
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone, const N: usize> From<[T; N]> for BinarySearchTree<T> {
    fn from(v: [T; N]) -> Self {
        let mut tree = BinarySearchTree::new();

        v.into_iter().for_each(|val| {
            tree.insert(val);
        });

        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_basics_should_work() {
        let mut tree = BinarySearchTree::from([4, 2, 6, 1, 3, 5, 7]);
        assert!(tree.search(&8).is_none());

        let node = tree.search(&4);
        assert!(node.is_some());
        assert_eq!(node.unwrap().borrow().value, 4);

        tree.remove(&4);
        assert!(tree.search(&4).is_none());

        assert_eq!(tree.to_vec(), vec![1, 2, 3, 5, 6, 7]);
    }
}
