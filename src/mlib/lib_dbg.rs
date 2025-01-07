use num::Zero;

pub fn vol<T: Zero>(x: T) -> &'static str {
    if x.is_zero() {
        "Low"
    } else {
        "Heigh"
    }
}
