use crate::{client::client_ins::ClientInstance, utils::errmap::ErrMap};

#[tauri::command]
pub fn connto(addr: &str) -> Result<(), String> {
    ClientInstance::conn(addr.parse().map_err(ErrMap::string)?).map_err(ErrMap::string)?;
    Ok(())
}
