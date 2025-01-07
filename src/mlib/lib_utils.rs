use std::ops::Mul;

use num::{One, Zero};
use wiringpi::pin;
pub fn bitvis<T: Into<u64>>(num: T, shf: u32) -> bool {
    ((num.into() >> shf) & 1) == 1
}

pub fn as_bool<T: Zero>(x: T) -> bool {
    !x.is_zero()
}

pub fn as_vol<T: Zero>(x: T) -> pin::Value {
    if x.is_zero() {
        pin::Value::Low
    } else {
        pin::Value::High
    }
}
pub fn from_vol<T: Zero + One>(x: pin::Value) -> T {
    match x {
        pin::Value::Low => T::zero(),
        pin::Value::High => T::one(),
    }
}
pub fn bool_vol<T: Into<bool>>(x: T) -> pin::Value {
    if x.into() {
        pin::Value::High
    } else {
        pin::Value::Low
    }
}
pub fn rev<T: Zero + One>(x: T) -> T {
    if x.is_zero() {
        T::one()
    } else {
        T::zero()
    }
}
pub fn irev<C: Into<bool>, T: Zero + One>(cond: C, x: T) -> T {
    if cond.into() {
        rev(x)
    } else {
        x
    }
}
pub fn irevb<C: Into<bool>>(cond: C, x: bool) -> bool {
    if cond.into() {
        !x
    } else {
        x
    }
}
pub fn icombo<C: Into<bool>, L, R: From<L> + Mul<R, Output = R>>(cond: C, num: L, combo: R) -> R {
    if cond.into() {
        R::from(num) * combo
    } else {
        R::from(num)
    }
}
