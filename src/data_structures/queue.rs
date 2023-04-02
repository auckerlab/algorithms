// queue.rs

#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,   // capacity of the queue
    data: Vec<T>, // container of queue
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // enqueue, check if there is spare space
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // dequeue the data
    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // check if the queue is empty
    pub fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    // size of queue
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test_queue {
    use super::*;

    #[test]
    fn basic() {
        let mut q = Queue::new(5);
        assert_eq!(q.size(), 0);
        assert_eq!(q.is_empty(), true);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(3);
        let _r3 = q.enqueue(4);
        let _r4 = q.enqueue(5);
        let _r5 = q.enqueue(6);
        assert_eq!(q.size(), 5);
        println!("content: {:?}", q);
    }
}
