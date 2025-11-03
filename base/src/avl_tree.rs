//! AVL 树是一种平衡搜索二叉树，它能够在不影响二叉树的中序遍历序列的前提下，
//! 通过旋转操作，使失衡节点重新恢复平衡。

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::binary_tree;

type NodeRc<T> = Rc<RefCell<AvlTreeNode<T>>>;
type OptionNodeRc<T> = Option<NodeRc<T>>;

#[derive(Debug)]
pub struct AvlTreeNode<T> {
    value: T,
    height: i32,
    left: OptionNodeRc<T>,
    right: OptionNodeRc<T>,
}

impl<T> AvlTreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            left: None,
            right: None,
            height: 0,
            value: val,
        }
    }

    fn new_node_rc(val: T) -> NodeRc<T> {
        Rc::new(RefCell::new(Self::new(val)))
    }
}

impl<T: Default> Default for AvlTreeNode<T> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            height: 0,
            value: T::default(),
        }
    }
}

impl<T: Clone> Clone for AvlTreeNode<T> {
    fn clone(&self) -> Self {
        Self {
            left: self.left.as_ref().map(Rc::clone),
            right: self.right.as_ref().map(Rc::clone),
            height: self.height,
            value: self.value.clone(),
        }
    }
}

pub struct AvlTree<T> {
    root: OptionNodeRc<T>,
}

impl<T> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for AvlTree<T> {
    fn from(v: [T; N]) -> Self {
        let mut tree = AvlTree::new();

        v.into_iter().for_each(|val| {
            tree.insert(val);
        });

        tree
    }
}

impl<T> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<T: Clone> AvlTree<T> {
    /// 转换为普通二叉树，忽略 height
    pub fn to_tree(&self) -> binary_tree::BinaryTree<T> {
        binary_tree::BinaryTree {
            root: clone_tree(&self.root),
        }
    }
}

fn clone_tree<T: Clone>(node: &OptionNodeRc<T>) -> binary_tree::OptionNodeRc<T> {
    match node {
        None => None,
        Some(node) => {
            let node_ref = node.borrow();

            Some(Rc::new(RefCell::new(binary_tree::TreeNode {
                left: clone_tree(&node_ref.left),
                value: node_ref.value.clone(),
                right: clone_tree(&node_ref.right),
            })))
        }
    }
}

impl<T: Ord> AvlTree<T> {
    /// 插入节点
    pub fn insert(&mut self, val: T) {
        self.root = insert_recursive(self.root.clone(), val);
    }
}

impl<T: Clone + Ord> AvlTree<T> {
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
}

fn insert_recursive<T: Ord>(node: OptionNodeRc<T>, val: T) -> OptionNodeRc<T> {
    match node {
        None => Some(AvlTreeNode::new_node_rc(val)),
        Some(node) => {
            // 查找插入位置并插入节点
            let ordering = { node.borrow().value.cmp(&val) };
            match ordering {
                Ordering::Equal => return Some(node), // 重复节点不插入
                Ordering::Greater => {
                    let left = node.borrow().left.clone();
                    {
                        let left = insert_recursive(left, val);
                        node.borrow_mut().left = left;
                    }
                }
                Ordering::Less => {
                    let right = node.borrow().right.clone();
                    {
                        let right = insert_recursive(right, val);
                        node.borrow_mut().right = right;
                    }
                }
            }
            // 更新节点高度
            update_height(&node);
            // 执行旋转操作，使该节点重新恢复平衡
            rotate(Some(node))
        }
    }
}

// 获取节点高度
fn height<T>(node: &OptionNodeRc<T>) -> i32 {
    match node {
        // 叶节点高度为0，空节点高度为-1
        Some(node) => node.borrow().height,
        None => -1,
    }
}

// 更新节点高度
fn update_height<T>(node: &NodeRc<T>) {
    let left_height = height(&node.borrow().left);
    let right_height = height(&node.borrow().right);

    // 节点高度等于最大子树高度+1
    node.borrow_mut().height = left_height.max(right_height) + 1;
}

// 获取节点平衡因子，节点平衡因子定义为节点左子树的高度减去右子树的高度。
// 特别的，空节点的平衡因子为0。设平衡因子为 f，则一颗 AVL 树的任意节点的
// 平衡因子皆满足 -1 <= f <= 1
fn balance_factor<T>(node: &OptionNodeRc<T>) -> i32 {
    match node {
        Some(node) => {
            let left = &node.borrow().left;
            let right = &node.borrow().right;

            height(left) - height(right)
        }
        None => 0,
    }
}

// 右旋操作
fn right_rotate<T>(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
    match node {
        Some(node) => {
            // Panics: 这里 node 的左子树节点不能为空
            let child = node.borrow().left.clone().unwrap();
            let grand_child = child.borrow().right.clone();
            // 以 child 为原点将 node 向右旋转
            node.borrow_mut().left = grand_child;
            update_height(&node);
            child.borrow_mut().right = Some(node);
            update_height(&child);
            // 返回旋转后子树的根节点
            Some(child)
        }
        None => None,
    }
}

// 左旋操作
fn left_rotate<T>(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
    match node {
        Some(node) => {
            // Panics: 这里 node 的右子树节点不能为空
            let child = node.borrow().right.clone().unwrap();
            let grand_child = child.borrow().left.clone();
            // 以 child 为原点将 node 向左旋转
            node.borrow_mut().right = grand_child;
            update_height(&node);
            child.borrow_mut().left = Some(node);
            update_height(&child);
            // 返回旋转后子树的根节点
            Some(child)
        }
        None => None,
    }
}

// 节点平衡因子 = 左子树高度 - 右子树高度
// ____________________________________________________________
// | 失衡节点的平衡因子 | 子节点的平衡因子 | 应采用的旋转方法 |
// |   >  1（左偏树）   |       >= 0       |      右旋        |
// |   >  1（左偏树）   |        < 0       |   先左旋再右旋   |
// |   < -1（右偏树）   |       <= 0       |      左旋        |
// |   < -1（右偏树）   |        > 0       |   先右旋再左旋   |
// ------------------------------------------------------------
// 执行旋转操作，使该节点重新恢复平衡
fn rotate<T>(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
    // 获取节点 node 的平衡因子
    let factor = balance_factor(&node);
    if factor > 1 {
        // 左偏树
        // Safety: 如果 node 是 None，则 balance_factor 应该为0，所以
        // 这里 node 一定不是 None，使用 unwrap() 是安全的
        let node = node.unwrap();
        if balance_factor(&node.borrow().left) >= 0 {
            // 右旋
            right_rotate(Some(node))
        } else {
            // 先左旋再右旋
            {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = left_rotate(left);
            }
            right_rotate(Some(node))
        }
    } else if factor < -1 {
        // 右偏树
        // Safety: 同理，这里使用 unwrap() 是安全的
        let node = node.clone().unwrap();
        if balance_factor(&node.borrow().right) <= 0 {
            // 左旋
            left_rotate(Some(node))
        } else {
            // 先右旋再左旋
            {
                let right = node.borrow().right.clone();
                node.borrow_mut().right = right_rotate(right);
            }
            left_rotate(Some(node))
        }
    } else {
        // 平衡树，无需旋转，直接返回
        node
    }
}

#[cfg(test)]
mod tests {
    use crate::bt;

    use super::*;

    #[test]
    fn avl_basics_should_work() {
        let mut avl_tree = AvlTree::new();

        avl_tree.insert(1);
        assert_eq!(avl_tree.search(&1).unwrap().borrow().value, 1);

        avl_tree.insert(2);
        assert_eq!(avl_tree.search(&2).unwrap().borrow().value, 2);
    }

    #[test]
    fn avl_convert_should_work() {
        let avl_tree = AvlTree::from([1, 3, 4, 5, 7]);
        println!("{:#?}", avl_tree.root);
        let tree = avl_tree.to_tree();

        assert_eq!(tree.to_vec(), vec![1, 3, 4, 5, 7]);
        assert!(bt::contains(&tree.root, &5));
    }
}
