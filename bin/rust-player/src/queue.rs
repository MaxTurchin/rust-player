use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time;


pub struct PlayQueue {
    current_idx: i32,
    queue: Vec<String>
}

impl PlayQueue {
    pub fn new() -> Self {
        Self {
            current_idx: -1,
            queue: Vec::<String>::new()
        }
    }

    
    pub fn add_to_queue(&mut self, item: String) {
        self.queue.push(item)
    }

    pub fn remove(&mut self, key: String) {
        let index = match self.queue.binary_search(&key) {
            Ok(i) => i,
            Err(_) => panic!("Oh no! Attempting to remove item that doesn't exit in queue")
        };
        self.queue.remove(index);
    }

    pub fn play_next(&mut self, key: String) {
        self.queue.insert(0, key);
    }


    pub fn get_next(&mut self) -> Option<String> {
        if (self.current_idx + 1) as usize == self.len() {
            return None;
        }
        self.current_idx += 1;

        return Some(self.queue[self.current_idx as usize].clone());
    }

    pub fn len(&self) -> usize{
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}