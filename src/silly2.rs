///! 栈上的链表
/// 新的栈上的链表类型本身就是一个 Node，并且包含一个引用指向另一个 Node
pub struct List<'a, T> {
    pub data: T,
    pub prev: Option<&'a List<'a, T>>,
}

impl <'a, T> List<'a, T> {
    /// 该链表只有一个操作 push，需要注意的是，跟其它链表不同，
    /// 这里的 push 是通过回调的方式来完成新元素推入，并将回调返回的值直接返回给 push 的调用者
    pub fn push<U> (
        prev: Option<&'a List<'a, T>>,
        data: T,
        callback: impl FnOnce(&List<'a, T>) -> U,
    ) -> U {
        let list = List { data: data, prev: prev};
        callback(&list)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter{ next: Some(self)}
    }   
}
// 迭代包装结构体
pub struct Iter<'a, T> { 
    next: Option<&'a List<'a, T>>,
}

// 实现迭代器
impl<'a, T> Iterator for Iter<'a, T> { 
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev;
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn elegance() {
        List::push(None, 3, |list| {
            assert_eq!(list.iter().copied().sum::<i32>(), 3);
            List::push(Some(list), 5, |list| {
                assert_eq!(list.iter().copied().sum::<i32>(), 5 + 3);
                List::push(Some(list), 13, |list| {
                    assert_eq!(list.iter().copied().sum::<i32>(), 13 + 5 + 3);
                })
            })
        })
    }
}