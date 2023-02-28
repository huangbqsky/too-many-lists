pub mod first;  // 不太优秀的单向链表：栈
pub mod second; // 还可以的单向链表：单所有权 Option + Box
pub mod thrid; // 持久化单向链表：共享所有权 Rc

pub mod fourth; // 一个不咋滴的双向链表 (功能残缺的，只实现了IntoIter， 未实现 Iter 和 IterMut)