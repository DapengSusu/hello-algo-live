use std::{cell::RefCell, cmp::Ordering, collections::VecDeque, rc::Rc};

use crate::bt;

pub type NodeRc<T> = Rc<RefCell<TreeNode<T>>>;
pub type OptionNodeRc<T> = Option<NodeRc<T>>;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: OptionNodeRc<T>,
    pub right: OptionNodeRc<T>,
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

impl<T: Clone> BinaryTree<T> {
    /// 转换成 Vec，中序遍历
    pub fn to_vec(&self) -> Vec<T> {
        bt::in_order(&self.root)
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

// 二叉搜索树
pub struct BinarySearchTree<T> {
    root: OptionNodeRc<T>,
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
        bt::in_order(&self.root)
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
        let mut tree = BinaryTree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert!(bt::contains_bfs(&tree.root, &2));
        assert!(bt::contains_bfs(&tree.root, &3));
        assert!(!bt::contains_bfs(&tree.root, &5));

        assert!(bt::contains_dfs(&tree.root, &2));
        assert!(bt::contains_dfs(&tree.root, &3));
        assert!(!bt::contains_dfs(&tree.root, &5));

        assert!(bt::contains(&tree.root, &2));
        assert!(bt::contains(&tree.root, &3));
        assert!(!bt::contains(&tree.root, &5));
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

        assert_eq!(bt::pre_order(&tree.root), vec![1, 2, 4, 5, 3, 6]);
    }

    #[test]
    fn tree_in_order_should_work() {
        let tree = new_binary_tree();

        assert_eq!(bt::in_order(&tree.root), vec![4, 2, 5, 1, 6, 3]);
    }

    #[test]
    fn tree_post_order_should_work() {
        let tree = new_binary_tree();

        assert_eq!(bt::post_order(&tree.root), vec![4, 5, 2, 6, 3, 1]);
    }

    #[test]
    fn search_tree_basics_should_work() {
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
