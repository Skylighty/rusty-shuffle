//use log::{debug, error, info, warn};
use rand::Rng;
use std::{thread, time};
use zmq;

//----------------------------------------------------------
// STRING LIST SNIPPET
// const VARS: &[&str] = &[
//     "s1",
//     "s2",
//     "s3"
// ]
//----------------------------------------------------------
// RANDOM INT SNIPPET!
// let num = rand::thread_rng().gen_range(200,1721)
//----------------------------------------------------------
// CHECK FILE EXISTANCE SNIPPET!
//    let result = std::path::Path::new(<path>).exists();
//----------------------------------------------------------
// ZMQ SNIPPET!
// let ctx = zmq::Context::new();
// let socket = ctx.socket(zmq::REQ).unwrap();
// socket.connect("tcp://127.0.0.1:1234").unwrap();
// socket.send("hello world!", 0).unwrap();
//----------------------------------------------------------
// SYS ARGS SNIPPET
// As in Python - first element is name of package :)
// let args: Vec<String> = env::args().collect();
// dbg!(args);
//----------------------------------------------------------
// LOGGING SNIPPET

// Constants containing our FFMPEG vars
const OVERLAYS_PRIMARY: [&'static str; 6] = ["var1", "var2", "var3", "var4", "var5", "var6"];

fn main() {
    //env_logger::init();
    loop {
        let mut file_exists: bool =
            std::path::Path::new("/home/pablo/MISIEK-WM/watermarking-on").exists();
        let mut width: u32;
        let mut height: u32;
        if file_exists {
            let context: zmq::Context = zmq::Context::new();
            let socket: zmq::Socket = context.socket(zmq::REQ).unwrap();
            socket.connect("tcp://localhost:5555").unwrap();
            'watermarking: loop {
                file_exists =
                    std::path::Path::new("/home/pablo/MISIEK-WM/watermarking-on").exists();
                if file_exists {
                    width = rand::thread_rng().gen_range(400..=1320);
                    height = rand::thread_rng().gen_range(300..=780);

                    for i in 0..6 {
                        let mut msg =
                            format!("overlay@{} x {}", OVERLAYS_PRIMARY[i], width.to_string());
                        let mut rmsg = zmq::Message::new();

                        socket.send(msg.as_bytes(), zmq::DONTWAIT).unwrap();
                        socket.recv(&mut rmsg, 0).unwrap();

                        msg = format!("overlay@{} y {}", OVERLAYS_PRIMARY[i], height.to_string());
                        socket.send(msg.as_bytes(), zmq::DONTWAIT).unwrap();
                        socket.recv(&mut rmsg, 0).unwrap();

                        let sleep_dur: time::Duration = time::Duration::from_millis(300);
                        thread::sleep(sleep_dur);
                    }
                } else {
                    break 'watermarking;
                }
            }
        } else {
            let sleep_time: time::Duration = time::Duration::from_millis(100);
            thread::sleep(sleep_time);
        }
    }
}
