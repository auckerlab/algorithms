// stack.rs

#[derive(Debug)]
pub struct Stack<T> {
    top: usize, // top of the stack
    data: Vec<T>, // data container of stack
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    // push to stack
    pub fn push(&mut self, val: T) {
        self.data.push(val); // data store at the end of stack
        self.top += 1;
    }

    // pop out of stack
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    // get the top of stack
    pub fn peek(&mut self) -> Option<&T> {
        if self.top == 0 { 
            return None; 
        }
        self.data.get(self.top - 1)
    }

    // check if the stack is empty
    pub fn is_empty(&self) -> bool {
        0 == self.top
    }

    // size of stack
    pub fn size(&self) -> usize {
        self.top  // top of stack is the num of elements
    }
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn check_stack() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(5);
        assert_eq!(Some(&5), s.peek());
        assert_eq!(3, s.size());
        assert_eq!(5, s.pop().unwrap());
        assert_eq!(2, s.size());
    }
}
