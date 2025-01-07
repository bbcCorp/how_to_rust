#![crate_name = "algorithms"]

#[derive(Debug, Clone)]
pub struct LinkedListAsVec<T> {
    data: Vec<T>,
    len: u32,
}

impl<T: Copy> LinkedListAsVec<T> {
    pub fn new() -> LinkedListAsVec<T> {
        LinkedListAsVec {
            data: Vec::new(),
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.data.push(value);
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        if self.len == 0 {
            self.push_back(value);
            return;
        }

        // Shift the data by 1 position
        for i in (0..self.len).rev() {
            let element_to_move: T = self.data[i as usize];
            if i == self.len - 1 {
                // for the last element allocate space and add the prev node value
                self.data.push(element_to_move);
            } else {
                self.data[(i + 1) as usize] = element_to_move;
            }
        }

        // now make the insertion
        self.data[0] = value;
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            self.data.pop()
        } else {
            None
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        Some(&self.data[index as usize])
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        Some(&self.data[(self.len - 1) as usize])
    }

    // This is the main advantage of a linked list
    pub fn insert_at_index(&mut self, index: u32, value: T) {
        if index > self.len {
            panic!("Index out of bounds");
        }

        if index == self.len {
            self.push_back(value);
            return;
        }

        // Shift all the nodes from index to the end to make space for insertion
        for i in (index..self.len).rev() {
            let element_to_move: T = self.data[i as usize];

            if i == self.len - 1 {
                // for the last element allocate space and add the prev node value
                self.data.push(element_to_move);
            } else {
                self.data[(i + 1) as usize] = element_to_move;
            }
        }

        // now make the insertion
        self.data[index as usize] = value;
        self.len += 1;
    }

    pub fn remove(&mut self, at: usize) -> T {
        panic!("Not implemented");
    }
}
