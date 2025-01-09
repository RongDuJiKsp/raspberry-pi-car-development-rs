use crate::{
    client::client_ins::ClientInstance,
    utils::{
        body::{MsgBody, PowDirc, PowMode, PowTrunMode},
        errmap::ErrMap,
    },
};
use serde::Serialize;
use std::net::SocketAddr;
fn write_json<T: Serialize>(addr: &str, x: &T) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(x)
        .map_err(ErrMap::string)
        .map_err(ErrMap::string)?;
    Ok(())
}
#[tauri::command]
pub fn go(addr: &str, speed: i32) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed,
            combo: 0.0,
        },
    )
}
#[tauri::command]
pub fn stop(addr: &str) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Stop,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed: 0,
            combo: 0.0,
        },
    )
}
#[tauri::command]
pub fn left(addr: &str, speed: i32) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Left,
            trun: PowTrunMode::Rev,
            speed,
            combo: 0.0,
        },
    )
}
#[tauri::command]
pub fn right(addr: &str, speed: i32) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Right,
            trun: PowTrunMode::Rev,
            speed,
            combo: 0.0,
        },
    )
}
#[tauri::command]
pub fn back(addr: &str, speed: i32) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Rev,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed,
            combo: 0.0,
        },
    )
}
#[tauri::command]
pub fn go_left(addr: &str, speed: i32, combo: f64) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Left,
            trun: PowTrunMode::Run,
            speed,
            combo,
        },
    )
}

#[tauri::command]
pub fn go_right(addr: &str, speed: i32, combo: f64) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Right,
            trun: PowTrunMode::Run,
            speed,
            combo,
        },
    )
}
#[tauri::command]
pub fn back_left(addr: &str, speed: i32, combo: f64) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Rev,
            dirc: PowDirc::Left,
            trun: PowTrunMode::Run,
            speed,
            combo,
        },
    )
}

#[tauri::command]
pub fn back_right(addr: &str, speed: i32, combo: f64) -> Result<(), String> {
    write_json(
        addr,
        &MsgBody {
            mode: PowMode::Rev,
            dirc: PowDirc::Right,
            trun: PowTrunMode::Run,
            speed,
            combo,
        },
    )
}
