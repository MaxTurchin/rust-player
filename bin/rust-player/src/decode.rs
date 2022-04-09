use std::fs::File;
use std::thread;
use std::sync::mpsc::Sender;
use minimp3::{Decoder, Frame, Error};


pub fn start_decode<T: 'static + cpal::Sample + Send + Sync>(fpath: &String, samples_tx: Sender<T>) {
    let mut dec = Decoder::new(File::open(fpath).unwrap());
    
    thread::spawn(move ||{
        loop {
            println!("decode");
            match dec.next_frame() {
                Ok(Frame {data, sample_rate, channels, ..}) => {
                    for sample in data {
                        samples_tx.send(cpal::Sample::from(&sample));
                    }
                },
                Err(Error::Eof) => break,
                Err(_) => break
            }
        }
    });
}