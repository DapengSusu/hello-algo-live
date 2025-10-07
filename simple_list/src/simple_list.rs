use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

/// 简易的不可变单向链表实现
pub struct SimpleList<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for SimpleList<T> {
    fn default() -> Self {
        SimpleList { head: None }
    }
}

impl<T> SimpleList<T> {
    /// 创建一个空的链表
    pub fn new() -> Self {
        SimpleList::default()
    }

    /// 向链表头部添加一个元素，返回一个新的链表
    pub fn prepend(&self, elem: T) -> SimpleList<T> {
        SimpleList {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    /// 删除头部元素，将剩下元素作为一个新的链表返回
    pub fn tail(&self) -> SimpleList<T> {
        SimpleList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// 获取头部元素
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// 不可变迭代器
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for SimpleList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::SimpleList;

    #[test]
    fn basics() {
        let list = SimpleList::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = SimpleList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
