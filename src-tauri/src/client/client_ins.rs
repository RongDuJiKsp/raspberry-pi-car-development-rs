use std::{
    collections::HashMap,
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{LazyLock, Mutex},
};

use serde::Serialize;

use crate::utils::alias::{EResult, Ptr, SharePtr};

pub struct ClientInstance {
    addr: SocketAddr,
    stream: TcpStream,
}
static CLIENT_INSTANCE: LazyLock<Mutex<HashMap<SocketAddr, SharePtr<ClientInstance>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
impl ClientInstance {
    pub fn conn(addr: SocketAddr) -> EResult<()> {
        let conn = TcpStream::connect(addr)?;
        CLIENT_INSTANCE
            .lock()?
            .insert(addr, Ptr::shared(ClientInstance { addr, stream: conn }));
        Ok(())
    }
    pub fn instance(addr: SocketAddr) -> EResult<SharePtr<ClientInstance>> {
        Ok(CLIENT_INSTANCE
            .lock()?
            .get(&addr)
            .ok_or_else(|| anyhow::anyhow!("Error!"))?
            .clone())
    }
    pub fn write_block(&mut self, s: &[u8]) -> EResult<()> {
        let prefixlen = s.len() as u16;
        self.stream
            .write(&[(prefixlen >> 8) as u8, prefixlen as u8])?;
        self.stream.write(s)?;
        Ok(())
    }
    pub fn write_json<T: Serialize>(&mut self, s: &T) -> EResult<()> {
        self.write_block(&serde_json::to_vec::<T>(s)?)?;
        Ok(())
    }
}
