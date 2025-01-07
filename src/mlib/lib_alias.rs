use wiringpi::pin;

pub type WiringPiHand = wiringpi::WiringPi<pin::WiringPi>;
pub type InputPin = pin::InputPin<pin::WiringPi>;
pub type OutputPin = pin::OutputPin<pin::WiringPi>;
pub type PwmPin = pin::SoftPwmPin<pin::WiringPi>;
