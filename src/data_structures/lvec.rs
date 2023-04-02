// lvec.rs
use std::fmt::Debug;

// node
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node { elem: elem, next: None }
    }
}

// linked list
#[derive(Debug)]
pub struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    fn new() -> Self {
        LVec { size: 0, head: None }
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn push(&mut self, elem: T) {
        let node = Node::new(elem);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();

            // find the last node
            for _i in 0..self.size-1 {
                curr = curr.next.as_mut().unwrap();
            }

            // insert new data at the last node
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    // append new data at the stack
    fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear();
    }

    fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size {
            index = self.size;
        }

        // there are 3 situations:
        let mut node = Node::new(elem);
        if self.is_empty() {  // LVec is empty, insert
            self.head = Some(Box::new(node));
        } else if index == 0 {  // insert into the head of linked list
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {  // insert into the middle part of linked list
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {  // find the insert part
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.remove(self.size - 1)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index > self.size { return None; }

        // there are 2 situations to delete the node
        let mut node;
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else { // if not the head node, find the node to be deleted, and handle the link
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index-1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;
        Some(node.elem)
    }

    // print the Lvec, besides, use `ToString` trait and use `println` to print
    fn print_lvec(&self) {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            println!("lvec val: {:#?}", node.elem);
            curr = node.next.as_ref();
        }
    }
}

#[cfg(test)]
mod test_lvec {
    use super::*;
    #[test]
    fn basic() {
        let mut lvec: LVec<i32> = LVec::new();
        lvec.push(10); lvec.push(11);
        lvec.push(12); lvec.push(13);
        lvec.insert(0, 9);

        let mut lvec2: LVec<i32> = LVec::new();
        lvec2.insert(0, 8);
        lvec2.append(&mut lvec);
        println!("lvec2 len: {}", lvec2.len());
        lvec2.print_lvec();

        let res1 = lvec2.pop();
        let res2 = lvec2.remove(0);
        println!("pop {:#?}", res1.unwrap());
        println!("remove {:#?}", res2.unwrap());
        lvec2.print_lvec();
    }
}
