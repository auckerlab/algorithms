/// list_stack.rs
/// Use linked list to impl the stack
// link list node
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        // init, no next link
        Node { data: data, next: None }
    }
}

// linked list stack
#[derive(Debug, Clone)]
pub struct Stack<T> {
    size: usize,
    top: Link<T>,  // top of stack 
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Stack { size: 0, top: None }
    }

    // take get node in top, remains the empty,
    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // as_ref turn the top node into reference obj
    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.data
        })
    }
    
    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }
}

#[cfg(test)]
mod test_list_stack {
    use super::*;
    #[test]
    fn basic() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(4);
        println!("TOP: {:?}, size: {}", s.peek().unwrap(), s.size());
        println!("pop: {:?}, size: {}", s.pop().unwrap(), s.size());
        println!("is_empty: {}, stack: {:?}", s.is_empty(), s);
    }
}