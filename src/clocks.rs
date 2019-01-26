//https://github.com/nrf-rs/nrf52-hal/blob/9a1c75c66b26cb3b631f7ed0f7ec7b630e2ef4dd/nrf52-hal-common/src/clocks.rs
use crate::time::{Hertz, U32Ext};
use efm32::CMU;

pub trait ClocksExt {
    fn constrain(self) -> ClocksCfg;
}

pub struct ClocksCfg {
    pub hfclk: HFCLK,
    pub lfclk: LFCLK,
}

pub struct HFCLK {
    _0: (),
}

pub struct LFCLK {
    _0: (),
}

pub struct Clocks {
    hfclk: Hertz,
    lfclk: Hertz,
}

impl HFCLK {
    // TODO: allow external clock selection?
}

impl LFCLK {
    // TODO: allow external clock selection? Calibration?
}

impl ClocksExt for CMU {
    fn constrain(self) -> ClocksCfg {
        ClocksCfg {
            hfclk: HFCLK { _0: () },
            lfclk: LFCLK { _0: () },
        }
    }
}

impl ClocksCfg {
    pub fn freeze(self) -> Clocks {
        // TODO - this isn't very useful, can you actually change internal clock speeds?
        Clocks {
            hfclk: 21_000_000.hz(),
            lfclk: 32_768.hz(),
        }
    }
}

impl Clocks {
    pub fn hfclk(&self) -> Hertz {
        self.hfclk
    }

    pub fn lfclk(&self) -> Hertz {
        self.lfclk
    }
}
