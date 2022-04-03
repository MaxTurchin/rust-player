use cpal::PlayStreamError;

use crate::playback::{Playback, PlaybackStatus};
use crate::queue::PlayQueue;

use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time;

pub struct Player {
    status: Arc<Mutex<PlaybackStatus>>,
    playback: Arc<Mutex<Playback>>,
    queue: Arc<Mutex<PlayQueue>>,
}

impl Player {
    pub fn new() -> Self {
        let status = Arc::new(Mutex::new(PlaybackStatus::Stopped));
        let playback = Arc::new(Mutex::new(Playback::new(status.clone())));
        let queue = Arc::new(Mutex::new(PlayQueue::new()));

        Self {
            status: status,
            playback: playback,
            queue: queue,
        }
    }

    pub fn pause(&mut self) {
        *self.status.lock().unwrap() = PlaybackStatus::Paused;
    }

    pub fn play(&mut self) {
        *self.status.lock().unwrap() = PlaybackStatus::Playing;
    }

    pub fn stop(&mut self) {
        *self.status.lock().unwrap() = PlaybackStatus::Stopped;
    }

    pub fn set_queue(&mut self, queue: Arc<Mutex<PlayQueue>>) {
        self.queue = queue;
    }

    pub fn play_queue(&self) {
        let queue = self.queue.clone();
        let playback = self.playback.clone();
        let status = self.status.clone();
        thread::spawn(move || {
            println!("Spawned play_queue thread");
            loop {
                while !queue.lock().unwrap().is_empty() {
                    if *status.lock().unwrap() == PlaybackStatus::Stopped {
                        let fpath = queue.lock().unwrap().pop().unwrap();
                        println!("play_queue fpath: {}", fpath);
                        println!("thread: fpath acquired");
                        playback.lock().unwrap().start_playback(&fpath);
                    }
                }
                sleep(time::Duration::from_micros(250));
            }
        });
    }
}
