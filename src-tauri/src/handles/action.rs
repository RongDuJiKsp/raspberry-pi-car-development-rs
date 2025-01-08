use std::net::SocketAddr;

use crate::{
    client::client_ins::ClientInstance,
    utils::{
        body::{MsgBody, PowDirc, PowMode, PowTrunMode},
        errmap::ErrMap,
    },
};

#[tauri::command]
pub fn go(addr: &str) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(&MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed: 50,
            combo: 0.0,
        })
        .map_err(ErrMap::string)?;
    Ok(())
}
#[tauri::command]
pub fn stop(addr: &str) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(&MsgBody {
            mode: PowMode::Stop,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed: 30,
            combo: 0.0,
        })
        .map_err(ErrMap::string)?;
    Ok(())
}
#[tauri::command]
pub fn left(addr: &str) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(&MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Left,
            trun: PowTrunMode::Rev,
            speed: 30,
            combo: 0.0,
        })
        .map_err(ErrMap::string)?;
    Ok(())
}
#[tauri::command]
pub fn right(addr: &str) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(&MsgBody {
            mode: PowMode::Drive,
            dirc: PowDirc::Right,
            trun: PowTrunMode::Rev,
            speed: 30,
            combo: 0.0,
        })
        .map_err(ErrMap::string)?;
    Ok(())
}
#[tauri::command]
pub fn back(addr: &str) -> Result<(), String> {
    ClientInstance::instance(addr.parse::<SocketAddr>().map_err(ErrMap::string)?)
        .map_err(ErrMap::string)?
        .lock()
        .map_err(ErrMap::string)?
        .write_json(&MsgBody {
            mode: PowMode::Rev,
            dirc: PowDirc::Line,
            trun: PowTrunMode::Rev,
            speed: 30,
            combo: 0.0,
        })
        .map_err(ErrMap::string)?;
    Ok(())
}
