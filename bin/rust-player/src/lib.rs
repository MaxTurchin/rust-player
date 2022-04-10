

mod playback;
mod player;
mod queue;
mod decode;
mod track;

use core::time;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};

use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring, jarray, jlongArray};
use track::Track;

use crate::player::Player;
use crate::queue::PlayQueue;


// Creates Queue and Player structs and returns raw pointers to Java
#[no_mangle]
pub extern "system" fn Java_application_RustApi_rInit(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) -> jarray {
    let queue = Arc::new(Mutex::new(queue::PlayQueue::new()));
    let mut player = player::Player::new();
    
    player.set_queue(queue.clone());
    _init_poll_track_info(&env, callback, player.play_queue());

    let player_ptr = Box::new(player);
    let queue_ptr = Box::new(queue);

    let mut player_ptr_raw = Box::into_raw(player_ptr) as jlong;
    let mut queue_ptr_raw = Box::into_raw(queue_ptr) as jlong;
    let mut ptrs = env.new_long_array(2)
                                    .expect("Oh no! Failed to create jlongarray!");

    env.set_long_array_region(ptrs, 0, &[player_ptr_raw, queue_ptr_raw]).unwrap();
    return ptrs
}

#[no_mangle]
pub extern "system" fn Java_application_RustApi_rPlayOrPause(
    env: JNIEnv,
    _class: JClass,
    player: jlong
) {
    unsafe {
        let p = &mut *(player as *mut Player);
        p.play_or_pause();
    }
}

#[no_mangle]
pub extern "system" fn Java_application_RustApi_rAddToQueue(
    env: JNIEnv,
    _class: JClass,
    queue: jlong,
    fpath: JString
) {
    //TEMP:
    let fpath = env.get_string(fpath).expect("Oh no! why u no work?").into();
    unsafe {
        let rc = &mut *(queue as *mut Arc<Mutex<PlayQueue>>);
        let mut queue = rc.lock().unwrap();
        queue.add_to_queue(fpath);
    }
}

#[no_mangle]
pub extern "system" fn Java_application_RustApi_rPlayNext(
    env: JNIEnv,
    _class: JClass,
    player: jlong
) {
    unsafe {
        let p = &mut *(player as *mut Player);
        p.play_next();
    }
}

#[no_mangle]
pub extern "system" fn Java_application_RustApi_rPlayPrevious(
    env: JNIEnv,
    _class: JClass,
    player: jlong
) {
    unsafe {
        let p = &mut *(player as *mut Player);
        p.play_previous();
    }
}

fn _init_poll_track_info(env: &JNIEnv, obj: JObject, rx: Receiver<Option<Track>>) {
    let jvm = env.get_java_vm().unwrap();
    let clb = env.new_global_ref(obj).unwrap();
    thread::spawn(move || {
        let env = jvm.attach_current_thread_as_daemon().unwrap();
        let clb = clb.clone();
        loop {
            match rx.recv().unwrap() {
                Some(t) => {
                    println!("Updating track meta data: {:?}", &t);
                    let title = env.new_string(t.title).unwrap();
                    let artist = env.new_string(t.artist).unwrap();
                    let album = env.new_string(t.album).unwrap();

                    env.call_method(&clb, "updateNowPlaying", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", &[title.into(), artist.into(), album.into()]).unwrap();
                },
                None => sleep(time::Duration::from_micros(250))
            }
        }
    });
}
 