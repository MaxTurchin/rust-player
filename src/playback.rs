use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::{thread, time};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, Device, Stream, StreamConfig, SupportedStreamConfig, SampleFormat};

use std::sync::{Arc, Mutex};

use crate::decode;

#[derive(Clone, PartialEq, Debug)]
pub enum PlaybackStatus {
    Playing, //Represents a playback that's currently active.
    Paused,  //Represents a playback that's paused and can be resumed.
    Stopped, //Represents a stopped playback that cannot be resumed.
             //Used for identifying when a track is done playing so that
             //next track from queue can be played.
}

pub struct Playback {
    status: Arc<Mutex<PlaybackStatus>>,
    stream: Option<Stream>,
}

//A Single Playback struct is created in runtime
impl Playback {
    pub fn new(player_status: Arc<Mutex<PlaybackStatus>>) -> Self {
        Self {
            status: player_status,
            stream: None,
        }
    }

    pub fn start_playback(&mut self, fpath: &String) {

        println!("start_playback");

        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let sample_format = device.default_output_config().unwrap().sample_format();

        //Updating playback status
        *self.status.lock().unwrap() = PlaybackStatus::Playing;

        match sample_format {
            cpal::SampleFormat::I16 => self.decode_and_play::<i16>(device, fpath),
            cpal::SampleFormat::U16 => self.decode_and_play::<u16>(device, fpath),
            cpal::SampleFormat::F32 => self.decode_and_play::<f32>(device, fpath),
        }
    }

    fn decode_and_play<T: 'static + cpal::Sample + Send + Sync>(
        &mut self,
        device: Device,
        fpath: &String,
    ) {
        println!("decode_and_play");
        let (tx, rx) = channel::<T>();
        decode::start_decode(fpath, tx);

        self.stream = get_playback_stream::<T>(device, rx, self.status.clone());
        self.stream.as_ref().unwrap().play();
    }
}

unsafe impl Send for Playback {}



fn get_playback_stream<T: 'static + cpal::Sample + Send + Sync>(
    device: Device,
    samples_rx: Receiver<T>,
    stream_status: Arc<Mutex<PlaybackStatus>>,
) -> Option<Stream> {
    let out_conf = device.default_output_config().unwrap().config();

    return Some(
        device
            .build_output_stream(
                &out_conf,
                move |data, _: &_| {
                    playback_clb::<T>(data, &samples_rx, stream_status.clone());
                },
                err_clb,
            )
            .unwrap(),
    );
}


fn playback_clb<T: cpal::Sample>(
    data: &mut [T],
    samples_rx: &Receiver<T>,
    status: Arc<Mutex<PlaybackStatus>>,
) {
    let mut status_data = status.lock().unwrap();

    for sample in data {
        if *status_data != PlaybackStatus::Playing { //if Playback is paused play silence.
            *sample = Sample::from(&0.0);
            continue;
        }
        *sample = match samples_rx.try_recv() {
            Ok(s) => s,
            Err(_) => { //RecvErr means that the channel is empty and the playback is done.
                *status_data = PlaybackStatus::Stopped;
                println!("Stopped it!");
                return;
            }
        };
    }
}

fn err_clb(error: cpal::StreamError) {
    panic!("Oh no! I don't know what happened!")
}
