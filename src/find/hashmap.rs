// hashmap.rs

// use slot to store the position, data store the data
#[derive(Debug, Clone, PartialEq)]
struct HashMap<T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}
impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(size: usize) -> Self {
        //' init slot and data
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap { size, slot, data }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.size
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key { panic!("Error: key must greater than 0"); }
        let pos = self.hash(key);
        if 0 == self.slot[pos] {  // there is no data in slot, insert directly
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {  // there is data in slot, find next available slot
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos {  // slot is full, exit
                    println!("Error: slot is full, quit insertion");
                    return;
                }
            }
            // insert data in the found slot
            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key { panic!("Error: key must greater than 0"); }
        
        let pos = self.hash(key);
        if 0 == self.slot[pos] {  // there is no data in the slot, returNone
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = 0;  // update slot and data
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T> = None;
            let mut stop = false; let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {  // find the value, delete 
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    // come back to initial pos of hash, 
                    curr = self.rehash(curr);
                    if curr == pos { stop = true; }
                }
            }
            data
        }

    }

    fn get(&self, key: usize) -> Option<&T> {
        if 0 == key { panic!("Error: key must greater than 0"); }
        // calculate the pos of data
        let pos = self.hash(key);
        let mut data: Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        // find the data recurrently
        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
                data = self.data.get(curr);
            } else {
                // come back to init pos, there is no value
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        data
    }

    fn contains(&self, key: usize) -> bool {
        if 0 == key { panic!("Error: key must greater than 0"); }
        self.slot.contains(&key)
    }

    fn len(&self) -> usize {
        let mut length = 0;
        for d in self.slot.iter() {
            if &0 != d {  // slot data is not 0, there is data, len + 1
                length += 1;
            }
        }

        length
    }
}

#[cfg(test)]
mod test_hashmap {
    use super::*;
    #[test]
    fn basic() {
        let mut hmap = HashMap::new(11);
        hmap.insert(10, "cat");
        hmap.insert(2, "dog");
        hmap.insert(3, "tiger");

        println!("Hashmap size {:?}", hmap.len());
        println!("Hashmap contains key 2? {}", hmap.contains(2));
        println!("Hashmap key 3: {:?}", hmap.get(3));
        println!("Hashmap remove key 3: {:?}", hmap.remove(3));
        println!("Hashmap remove key 3: {:?}", hmap.remove(3));
    }
}