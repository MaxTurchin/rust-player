

mod playback;
mod player;
mod queue;
mod decode;

use std::sync::{Arc, Mutex};

use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jstring, jarray, jlongArray};

use crate::player::Player;
use crate::queue::PlayQueue;


// Creates Queue and Player structs and returns raw pointers to Java
#[no_mangle]
pub extern "system" fn Java_application_RustApi_rInit(
    env: JNIEnv,
    _class: JClass, 
) -> jarray {
    let queue = Arc::new(Mutex::new(queue::PlayQueue::new()));
    let mut player = player::Player::new();
    
    player.set_queue(queue.clone());
    player.play_queue();
    

    let mut player_ptr = Box::into_raw(Box::new(player)) as jlong;
    let mut queue_ptr = Box::into_raw(Box::new(queue)) as jlong;
    let mut ptrs = env.new_long_array(2)
                                    .expect("Oh no! Failed to create jlongarray!");

    env.set_long_array_region(ptrs, 0, &[player_ptr, queue_ptr]).unwrap();
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
 