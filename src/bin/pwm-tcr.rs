use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use raspberry_pi_car_development_rs::deb;
use raspberry_pi_car_development_rs::mlib::lib_dbg::vol;
use raspberry_pi_car_development_rs::mlib::lib_light::RGLight;
use raspberry_pi_car_development_rs::mlib::lib_tcr::{TcrSense, MID_BIT};
use raspberry_pi_car_development_rs::mlib::{
    lib_drive::{PowDirc, PowMode, PowTrunMode, PwmDriver, POW_TUN_COMBO_NONE},
    lib_tcr::{Status, LEFT_BIT, RIGHT_BIT},
    lib_utils::as_bool,
};
use wiringpi::setup;
//这一段是小车运行的参数，这个参数相对保守（60秒跑完轨道），卷皮可以适当调整
//读取状态的tick，单位为毫秒，越小越灵敏
//在一般运行状态读取状态的tick
const P_DELAY: u64 = 8;
//在转弯状态读取状态的tick
const P_TRANSDELAY: u64 = 5;
//在纠正状态读取状态的tick
const P_TRYDELAY: u64 = 3;
//这一段是在各个状态的运行速度，越大越快
//在一般运行状态的速度
const P_FASTSPEED: i32 = 42;
//在转弯状态的速度
const P_TRYTRANSPEED: i32 = 34;
//在纠正状态的速度
const P_TRYSPEED: i32 = 36;
//这一段是防抖参数，越小越灵敏
//在读取不到任何信息尝试偏转纠正的tick数
const P_NEED_TRANS_TICK: i32 = 6;
//在读取不到任何信息尝试转弯的tick数
const P_UNHEAD_TICK: i32 = 6;
//认为是转弯的tick数
const P_TRANS_TICK: i32 = 6;
//认为是直道的tick数
const P_LINE_TICK: i32 = 12;
#[derive(Clone, Copy, PartialEq, Eq)]
enum Sports {
    Left,
    Line,
    Right,
}
impl Sports {
    fn should_ctx(stat: Status) -> Sports {
        if as_bool(stat & LEFT_BIT) && !as_bool(stat & RIGHT_BIT) {
            return Sports::Left;
        } else if !as_bool(stat & LEFT_BIT) && as_bool(stat & RIGHT_BIT) {
            return Sports::Right;
        }
        Sports::Line
    }
    fn exec(self, drive: &PwmDriver) {
        match self {
            Sports::Left => drive.drive(
                PowMode::Drive,
                PowDirc::Left,
                PowTrunMode::Rev,
                P_TRYSPEED,
                Duration::from_millis(P_TRYDELAY),
                POW_TUN_COMBO_NONE,
            ),
            Sports::Line => drive.drive(
                PowMode::Drive,
                PowDirc::Line,
                PowTrunMode::Rev,
                P_TRYTRANSPEED,
                Duration::from_millis(P_TRANSDELAY),
                POW_TUN_COMBO_NONE,
            ),
            Sports::Right => drive.drive(
                PowMode::Drive,
                PowDirc::Right,
                PowTrunMode::Rev,
                P_TRYTRANSPEED,
                Duration::from_millis(P_TRANSDELAY),
                POW_TUN_COMBO_NONE,
            ),
        }
    }
}
struct CpuContext {
    sport: Sports,
    turning: Sports,
    deb_line: i32,
    deb_unhead: i32,
    deb_left: i32,
    deb_right: i32,
    pwm: Arc<PwmDriver>,
}
impl CpuContext {
    fn new(pow: Arc<PwmDriver>) -> CpuContext {
        CpuContext {
            sport: Sports::Line,
            turning: Sports::Line,
            deb_left: 0,
            deb_right: 0,
            deb_line: 0,
            deb_unhead: 0,
            pwm: pow,
        }
    }
    fn clean_deb(&mut self) {
        self.deb_left = 0;
        self.deb_right = 0;
        self.deb_line = 0;
        self.deb_unhead = 0;
    }
    fn run(&mut self, stat: Status) {
        if stat == 0 {
            if self.turning == Sports::Line {
                self.deb_unhead += 1;
                self.turning.exec(&self.pwm);
                return;
            }
            deb!(self.deb_unhead, P_UNHEAD_TICK, {
                self.sport = self.turning
            });
            self.sport.exec(&self.pwm);
        } else if Sports::should_ctx(stat) == Sports::Left && self.sport == Sports::Line {
            deb!(self.deb_left, P_TRANS_TICK, {
                self.turning = Sports::Left;
                if stat == (LEFT_BIT | MID_BIT) {
                    self.deb_unhead = 0;
                }
                deb!(self.deb_unhead, P_NEED_TRANS_TICK, {
                    self.sport = self.turning;
                    self.clean_deb();
                });
            });
            self.sport.exec(&self.pwm);
        } else if Sports::should_ctx(stat) == Sports::Right && self.sport == Sports::Line {
            deb!(self.deb_right, P_TRANS_TICK, {
                self.turning = Sports::Right;
                if stat == (MID_BIT | RIGHT_BIT) {
                    self.deb_unhead = 0;
                }
                deb!(self.deb_unhead, P_NEED_TRANS_TICK, {
                    self.sport = self.turning;
                    self.clean_deb();
                });
            });
            self.sport.exec(&self.pwm);
        } else if stat == MID_BIT && self.sport != Sports::Line {
            deb!(self.deb_line, P_LINE_TICK, {
                self.pwm.drive(
                    PowMode::Drive,
                    PowDirc::Line,
                    PowTrunMode::Rev,
                    P_FASTSPEED,
                    Duration::from_millis(P_DELAY),
                    POW_TUN_COMBO_NONE,
                );
                self.sport = Sports::Line;
                self.turning = Sports::Line;
                self.clean_deb();
            });
            self.sport.exec(&self.pwm);
        } else {
            self.sport.exec(&self.pwm);
            self.clean_deb();
        }
    }
}

fn main() {
    let pi = setup();
    let pwm = Arc::new(PwmDriver::new(&pi));
    {
        let pwm_hd = pwm.clone();
        ctrlc::set_handler(move || {
            pwm_hd.stop();
            exit(0);
        })
        .expect("E");
    }
    let mut ctx = CpuContext::new(pwm);
    let tcr = TcrSense::new(&pi);
    let led = RGLight::new(&pi);
    loop {
        let stat = tcr.read();
        led.write(
            Some(as_bool(stat & LEFT_BIT)),
            Some(as_bool(stat & RIGHT_BIT)),
        );
        if env::args().skip(1).next().is_some() {
            println!(
                "{}{}{}",
                vol(stat & LEFT_BIT),
                vol(stat & MID_BIT),
                vol(stat & RIGHT_BIT)
            );
        } else {
            ctx.run(stat);
        }
    }
}
