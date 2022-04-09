use cpal::PlayStreamError;

use crate::playback::{Playback, PlaybackStatus};
use crate::queue::PlayQueue;
use crate::track::Track;

use crate::*;

use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time;

pub struct Player {
    current_track: Box<Option<Track>>,
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
            current_track: Box::new(None),
            status: status,
            playback: playback,
            queue: queue,
        }
    }

    pub fn play_or_pause(&mut self) {
        let mut status = self.status.lock().unwrap();
        if *status != PlaybackStatus::Playing {
            *status = PlaybackStatus::Playing
        } else {
            *status = PlaybackStatus::Paused;
        }
    }

    pub fn stop(&mut self) {
        *self.status.lock().unwrap() = PlaybackStatus::Stopped;
    }

    pub fn set_queue(&mut self, queue: Arc<Mutex<PlayQueue>>) {
        self.queue = queue;
    }

    pub fn play_queue(&self) -> Receiver<Option<Track>>{
        let queue = self.queue.clone();
        let playback = self.playback.clone();
        let status = self.status.clone();

        let (tx, rx) = channel::<Option<Track>>();
        //Sends current Track once playback of new track is started
        thread::spawn(move || {
            println!("Spawned play_queue thread");
            loop {
                while !queue.lock().unwrap().is_empty() {
                    if *status.lock().unwrap() == PlaybackStatus::Stopped {
                        let track = match queue.lock().unwrap().get_next() {
                            Some(t) => t,
                            None => {
                                continue;
                            }
                        };
                        playback.lock().unwrap().start_playback(&track.fpath);
                        tx.send(Some(track.clone()));
                    }
                }
                sleep(time::Duration::from_micros(250));
            }
        });
        return rx
    }
}
