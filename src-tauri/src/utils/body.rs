use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum PowMode {
    Stop = 0,
    Drive = 5,
    Rev = 6,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum PowDirc {
    Line = 0,
    Left = 2,
    Right = 1,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum PowTrunMode {
    Rev = 1,
    Run = 0,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MsgBody {
    pub mode: PowMode,
    pub dirc: PowDirc,
    pub trun: PowTrunMode,
    pub speed: i32,
    pub combo: f64,
}
