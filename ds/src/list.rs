use std::cell::Cell;
use std::rc::Rc;

struct List<T> where T: 'static + Clone {
    size: usize,
    head: Option<Rc<Box<Node<T>>>>,
    tail: Option<Rc<Box<Node<T>>>>,
}

struct Node<T> where T: 'static + Clone {
    value: T,
    // Cell实现内部可变性，Option实现存在性，Rc实现多引用，Box实现固定大小
    prev: Cell<Option<Rc<Box<Node<T>>>>>,
    next: Cell<Option<Rc<Box<Node<T>>>>>,
}

impl<T> List<T> where T: 'static + Clone {
    fn new() -> Self {
        List {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_empty() {
            let node = Node {
                value,
                prev: Cell::new(None),
                next: Cell::new(None),
            };
            let node_rc = Rc::new(Box::new(node));
            self.head = Some(Rc::clone(&node_rc));
            self.tail = Some(Rc::clone(&node_rc));
        } else {
            let node_prev;
            if let Some(ref tail_val) = self.tail {
                // 读取旧尾节点并增加其Rc
                node_prev = Rc::clone(tail_val);
            } else {
                panic!()
            }
            // 构建节点
            let node = Node {
                value,
                // 设置前驱节点
                prev: Cell::new(Some(node_prev)),
                next: Cell::new(None),
            };
            let node_rc = Rc::new(Box::new(node));
            if let Some(ref tail_val) = self.tail {
                // 设置旧尾节点的后缀节点
                tail_val.next.set(Some(Rc::clone(&node_rc)));
            } else {
                panic!()
            }
            // 更新尾节点为新的节点
            self.tail = Some(node_rc);
        }
        self.size += 1;
    }

    pub fn push_front(&mut self, value: T) {
        if self.is_empty() {
            let node = Node {
                value,
                prev: Cell::new(None),
                next: Cell::new(None),
            };
            let node_rc = Rc::new(Box::new(node));
            self.head = Some(Rc::clone(&node_rc));
            self.tail = Some(Rc::clone(&node_rc));
        } else {
            let node_prev;
            if let Some(ref head_val) = self.head {
                // 读取旧头节点并增加其Rc
                node_prev = Rc::clone(head_val);
            } else {
                panic!()
            }
            // 构建节点
            let node = Node {
                value,
                prev: Cell::new(None),
                // 设置后缀节点
                next: Cell::new(Some(node_prev)),
            };
            let node_rc = Rc::new(Box::new(node));
            if let Some(ref head_val) = self.head {
                // 设置旧头节点的前驱节点
                head_val.prev.set(Some(Rc::clone(&node_rc)));
            } else {
                panic!()
            }
            // 更新头节点为新的节点
            self.head = Some(node_rc);
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let ans;
            if let Some(ref tail_val) = self.tail {
                ans = Some(tail_val.value.clone());
                // 先取出来，临时保存
                let tail_prev = tail_val.prev.take();
                // 中断旧尾节点的前驱节点
                tail_val.prev.set(None);
                if let Some(ref tail_prev_val) = tail_prev {
                    // 如果前面还有节点，更新它的next
                    tail_prev_val.next.set(None);
                    self.tail = Some(Rc::clone(tail_prev_val));
                } else {
                    self.tail = None;
                }
                ans
            } else {
                panic!()
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let ans;
            if let Some(ref head_val) = self.head {
                ans = Some(head_val.value.clone());
                let head_next = head_val.next.take();
                // 中断旧头节点的后缀节点
                head_val.next.set(None);
                if let Some(ref head_next_val) = head_next {
                    // 如果后面还有节点，prev
                    head_next_val.prev.set(None);
                    self.head = Some(Rc::clone(head_next_val));
                } else {
                    self.head = None;
                }
                ans
            } else {
                panic!()
            }
        }
    }

    pub fn peek_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.tail.as_ref().unwrap().value.clone())
        }
    }

    pub fn peek_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.head.as_ref().unwrap().value.clone())
        }
    }

    pub fn peek_back_ref(&mut self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.tail.as_ref().unwrap().value)
        }
    }

    pub fn peek_front_ref(&mut self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.head.as_ref().unwrap().value)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        if self.size == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;

    #[test]
    fn test() {
        let mut list = List::new();
        list.push_front(1.to_string());
        list.push_back(2.to_string());
        list.push_back(3.to_string());
        println!("{}", list.pop_front().unwrap());
        println!("{}", list.pop_front().unwrap());
        list.push_front(4.to_string());
        list.push_front(5.to_string());
        list.push_back(6.to_string());
        println!("{}", list.pop_front().unwrap());
        list.push_front(7.to_string());
        list.push_front(8.to_string());
        println!("{}", list.pop_back().unwrap());
        println!("{}", list.pop_front().unwrap());
        list.push_front(9.to_string());
        list.push_front(10.to_string());
        println!("{}", list.peek_front_ref().unwrap());
        while !list.is_empty() {
            println!("{}", list.pop_front().unwrap());
        }
    }
}