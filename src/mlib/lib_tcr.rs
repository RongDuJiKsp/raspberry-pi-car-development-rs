use super::{
    lib_alias::{InputPin, WiringPiHand},
    lib_pins::pins_wiringpi::{LEFT_TCR_PIN, MID_TCR_PIN, RIGHT_TCR_PIN},
    lib_utils::from_vol,
};
pub type Status = u16;
pub const LEFT_BIT: u16 = 4;
pub const MID_BIT: u16 = 2;
pub const RIGHT_BIT: u16 = 1;
pub struct TcrSense {
    tcr_l: InputPin,
    tcr_mid: InputPin,
    tcr_r: InputPin,
}
impl TcrSense {
    pub fn new(pi: &WiringPiHand) -> TcrSense {
        TcrSense {
            tcr_l: pi.input_pin(LEFT_TCR_PIN),
            tcr_mid: pi.input_pin(MID_TCR_PIN),
            tcr_r: pi.input_pin(RIGHT_TCR_PIN),
        }
    }
    pub fn read(&self) -> Status {
        (from_vol::<Status>(self.tcr_l.digital_read()) << 2)
            | (from_vol::<Status>(self.tcr_mid.digital_read()) << 1)
            | from_vol::<Status>(self.tcr_r.digital_read())
    }
}
