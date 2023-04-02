// deque.rs

#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,  // capacity
    data: Vec<T>, // container of data
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Deque {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // ending of Vec is head of Deque
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    // head of Vec is tail of Deque
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // remove data from head of dequeue
    pub fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // remove data from tail of dequeue
    pub fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test_deque {
    use super::*;

    #[test]
    fn basic() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);
        if let Err(error) = d.add_front(5) {
            println!("add_front error: {error}");
        }
        if let Some(data) = d.remove_rear() {
            println!("data: {data}");
        } else {
            println!("empty queue");
        }

        println!("size: {}, is_empty: {}", d.size(), d.is_empty());
        println!("content: {:?}", d);
    }
}