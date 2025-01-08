use serde::{Deserialize, Serialize};

use super::{
    lib_alias::{OutputPin, PwmPin, WiringPiHand},
    lib_pins::pins_wiringpi::{
        POW_IN1_L_PIN, POW_IN1_R_PIN, POW_IN2_L_PIN, POW_IN2_R_PIN, POW_PWM_L_PIN, POW_PWM_R_PIN,
    },
    lib_utils::{as_bool, bitvis, bool_vol, icombo, irevb},
};
use std::thread;
use std::time::Duration;
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
pub const POW_TUN_COMBO_NONE: f64 = 0f64;
pub struct PwmDriver {
    pwm_l: PwmPin,
    in2_l: OutputPin,
    in1_l: OutputPin,
    pwm_r: PwmPin,
    in2_r: OutputPin,
    in1_r: OutputPin,
}
impl PwmDriver {
    pub fn new(pi: &WiringPiHand) -> PwmDriver {
        PwmDriver {
            pwm_l: pi.soft_pwm_pin(POW_PWM_L_PIN),
            in2_l: pi.output_pin(POW_IN2_L_PIN),
            in1_l: pi.output_pin(POW_IN1_L_PIN),
            pwm_r: pi.soft_pwm_pin(POW_PWM_R_PIN),
            in2_r: pi.output_pin(POW_IN2_R_PIN),
            in1_r: pi.output_pin(POW_IN1_R_PIN),
        }
    }
    pub fn drive(
        &self,
        mode: PowMode,
        dirc: PowDirc,
        turn: PowTrunMode,
        speed: i32,
        dura: Duration,
        speed_combo: f64,
    ) {
        let mode = mode as u16;
        let dirc = dirc as u16;
        let turn = turn as u16;
        let (pin2, pin1) = (bitvis(mode, 1), bitvis(mode, 0));
        let (revl, revr) = (bitvis(dirc, 1), bitvis(dirc, 0));
        self.pwm_l
            .pwm_write(icombo(!as_bool(turn) && revl, speed, speed_combo) as i32);
        self.in2_l
            .digital_write(bool_vol(irevb(as_bool(turn) && revl, pin2)));
        self.in1_l
            .digital_write(bool_vol(irevb(as_bool(turn) && revl, pin1)));

        self.pwm_r
            .pwm_write(icombo(!as_bool(turn) && revr, speed, speed_combo) as i32);
        self.in2_r
            .digital_write(bool_vol(irevb(as_bool(turn) && revr, pin2)));
        self.in1_r
            .digital_write(bool_vol(irevb(as_bool(turn) && revr, pin1)));
        thread::sleep(dura);
    }
    pub fn stop(&self) {
        self.drive(
            PowMode::Stop,
            PowDirc::Line,
            PowTrunMode::Rev,
            0,
            Duration::ZERO,
            POW_TUN_COMBO_NONE,
        );
    }
}
