use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time;

use crate::track::{self, Track};

pub struct PlayQueue {
    current_idx: i32,
    queue: Vec<Track>
}

impl PlayQueue {
    pub fn new() -> Self {
        Self {
            current_idx: -1,
            queue: Vec::<Track>::new()
        }
    }

    pub fn add_to_queue(&mut self, fpath: String) {
        let track = Track::new(fpath);
        self.queue.push(track);
    }

    pub fn remove(&mut self, fpath: String) {
        let idx = match self.queue.iter().position(|item| item.fpath == fpath) {
            Some(i) => i,
            None => panic!("Oh no! Attempting to remove item that doesn't exit in queue")
        };
        self.queue.remove(idx);
    }

    pub fn get_next(&mut self) -> Option<Track> {
        if (self.current_idx + 1) as usize == self.len() {
            return None;
        }
        self.current_idx += 1;
        println!("idx: {}", self.current_idx);

        return Some(self.queue[self.current_idx as usize].clone());
    }

    pub fn set_next_to_previous(&mut self) {
        //Move current_idx so that previous track is next in playback
        if self.current_idx - 2 < -1 {
            self.current_idx -= 1 //Play current song again
        } else {
            self.current_idx -= 2; //Play previous song
        }
    }

    pub fn len(&self) -> usize{
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}