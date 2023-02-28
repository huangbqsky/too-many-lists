use std::mem;


pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>), 
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // List链表的关联函数
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // 添加一个值到链表
    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
             elem: val, 
             // mem::replace函数允许我们从一个借用中偷出一个值的同时再放入一个新值。
             // 这里，我们从借用 self 中偷出了它的值 head 并赋予给 next 字段，
             // 同时将一个新值 Link::Empty 放入到 head 中，成功完成偷梁换柱
             next: mem::replace(&mut self.head, Link::Empty),
         });
         self.head = Link::More(new_node);
    }

    // 取出一个数据
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// 为链表 List 实现 Drop trait
impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}