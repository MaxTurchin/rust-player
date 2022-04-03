

mod playback;
mod player;
mod queue;
mod decode;

use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{thread, time};

fn main() {
    println!("Running!");

    let mut p = player::Player::new();
    println!("Player created!");
    let q = Arc::new(Mutex::new(queue::PlayQueue::new()));
    println!("Queue created!");
    q.lock().unwrap().push("C:\\Users\\Max\\Music\\third ever.mp3".to_string());

    p.set_queue(q.clone());
    println!("Queue set!");
    p.play_queue();
    
    

    sleep(time::Duration::from_secs(10));
    p.pause();
    sleep(time::Duration::from_secs(4));
    p.play();

    q.lock().unwrap().play_next("C:\\Users\\Max\\Music\\first ever.mp3".to_string());
    println!("added to queue!!!!");

    sleep(time::Duration::from_secs(1000000));
}
 