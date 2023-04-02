/// hot_potato.rs
/// Use queue to solve this problem
use crate::data_structures::Queue;

fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // init the queue, enqueue the names
    let mut q = Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }

    while q.size() > 1 {
        // enqueue and dequeue, we want the last remains
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // enqueue/dequeue times up to num, delete
        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}

#[cfg(test)]
mod test_hot_potato {
    use super::*;
    #[test]
    fn basic() {
        let name = vec!["Shieber", "Ryan", "Pavlo", "Lamport", "Andy", "aucker"];
        let rem = hot_potato(name, 8);
        // assert_eq!()
        println!("The left person is {rem}");
    }
}

