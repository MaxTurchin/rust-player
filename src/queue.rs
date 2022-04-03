use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time;


pub struct PlayQueue {
    queue: VecDeque<String>
}

impl PlayQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::<String>::new()
        }
    }

    pub fn push(&mut self, fpath: String) {
        self.queue.push_back(fpath);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop_front()
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


    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}