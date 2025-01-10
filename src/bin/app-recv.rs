use std::{
    env,
    io::{BufReader, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

use raspberry_pi_car_development_rs::mlib::{
    lib_drive::{PowDirc, PowMode, PowTrunMode, PwmDriver},
    lib_log,
};
use serde::{Deserialize, Serialize};
const RECV_BUF_SIZE: usize = 1024;
#[derive(Debug, Serialize, Deserialize)]
struct MsgBody {
    mode: PowMode,
    dirc: PowDirc,
    trun: PowTrunMode,
    speed: i32,
    combo: f64,
}
struct RecvPower {
    cmd: mpsc::Receiver<MsgBody>,
    power: Arc<PwmDriver>,
}
fn operator_loop(power: RecvPower) {
    loop {
        match power.cmd.recv() {
            Ok(nextcmd) => {
                power.power.drive(
                    nextcmd.mode,
                    nextcmd.dirc,
                    nextcmd.trun,
                    nextcmd.speed,
                    Duration::from_millis(1),
                    nextcmd.combo,
                );
            }
            Err(e) => {
                lib_log::log(&format!("Error On Exec cmd:{}", &e));
            }
        }
    }
}
fn message_recv(sender: mpsc::Sender<MsgBody>, conn: TcpStream, addr: SocketAddr) {
    lib_log::log(&format!("New Client Conn : {}", &addr));
    let mut buffer = BufReader::new(conn.try_clone().expect("Fail Clone Stream"));
    let mut buf_json = [0u8; RECV_BUF_SIZE];
    loop {
        let mut body_prefix = [0u8; 2];
        buffer
            .read_exact(&mut body_prefix)
            .expect("Read With Error");
        let len = ((body_prefix[0] as u16) << 8) + (body_prefix[1] as u16);
        buffer
            .read_exact(&mut buf_json[0..len as usize])
            .expect("Read With Error");
        lib_log::log(&format!(
            "Recv Json(len:{}) :{}",
            len,
            String::from_utf8(buf_json[0..len as usize].to_vec()).expect("Err on parse")
        ));
        let json = serde_json::from_slice::<MsgBody>(&buf_json[0..len as usize])
            .expect("Json Read Failed,Conn Close");
        sender.send(json).expect("Channel Failed");
    }
}
fn main() {
    let pi = wiringpi::setup();
    let power = Arc::new(PwmDriver::new(&pi));
    {
        let power_stop = power.clone();
        ctrlc::set_handler(move || {
            power_stop.stop();
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");
    }
    let (tx, rx) = mpsc::channel();
    let recv_pow = RecvPower { cmd: rx, power };
    thread::spawn(move || operator_loop(recv_pow));
    let conn = TcpListener::bind(
        env::args()
            .skip(1)
            .next()
            .unwrap_or_else(|| String::from("0.0.0.0:37037")),
    )
    .expect("在初始化时失败");

    loop {
        if let Ok((conn, addr)) = conn.accept() {
            let s = tx.clone();
            thread::spawn(move || message_recv(s, conn, addr));
        }
    }
}
