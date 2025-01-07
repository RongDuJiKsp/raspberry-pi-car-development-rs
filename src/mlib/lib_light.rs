use super::{
    lib_alias::{OutputPin, WiringPiHand},
    lib_pins::pins_wiringpi::{GREEN_PIN, RED_PIN},
    lib_utils::bool_vol,
};

pub struct RGLight {
    red: OutputPin,
    green: OutputPin,
}
impl RGLight {
    pub fn new(pi: &WiringPiHand) -> RGLight {
        RGLight {
            red: pi.output_pin(RED_PIN),
            green: pi.output_pin(GREEN_PIN),
        }
    }
    pub fn write(&self, red_open: Option<bool>, green_open: Option<bool>) {
        if let Some(x) = red_open {
            self.red.digital_write(bool_vol(x));
        }
        if let Some(x) = green_open {
            self.green.digital_write(bool_vol(x));
        }
    }
}
