//! Unit conversion factors.
//!
//! This module provides common unit conversion factors as defined by NIST and IAU.

/// SI Prefixes
pub mod si_prefixes {
    pub const QUETTA: f64 = 1e30;
    pub const RONNA: f64 = 1e27;
    pub const YOTTA: f64 = 1e24;
    pub const ZETTA: f64 = 1e21;
    pub const EXA: f64 = 1e18;
    pub const PETA: f64 = 1e15;
    pub const TERA: f64 = 1e12;
    pub const GIGA: f64 = 1e9;
    pub const MEGA: f64 = 1e6;
    pub const KILO: f64 = 1e3;
    pub const HECTO: f64 = 1e2;
    pub const DEKA: f64 = 1e1;
    pub const DECI: f64 = 1e-1;
    pub const CENTI: f64 = 1e-2;
    pub const MILLI: f64 = 1e-3;
    pub const MICRO: f64 = 1e-6;
    pub const NANO: f64 = 1e-9;
    pub const PICO: f64 = 1e-12;
    pub const FEMTO: f64 = 1e-15;
    pub const ATTO: f64 = 1e-18;
    pub const ZEPTO: f64 = 1e-21;
    pub const YOCTO: f64 = 1e-24;
    pub const RONTO: f64 = 1e-27;
    pub const QUECTO: f64 = 1e-30;
}

/// Time conversion factors (to seconds)
pub mod time {
    pub const MINUTE: f64 = 60.0;
    pub const HOUR: f64 = 3600.0;
    pub const DAY: f64 = 86400.0;
    pub const WEEK: f64 = 604_800.0;
    /// Julian year (exactly 365.25 days)
    pub const YEAR: f64 = 31_557_600.0;
}

/// Length conversion factors (to meters)
pub mod length {
    pub const ANGSTROM: f64 = 1e-10;
    pub const INCH: f64 = 0.0254;
    pub const FOOT: f64 = 0.3048;
    pub const MILE: f64 = 1609.344;
    pub const NAUTICAL_MILE: f64 = 1852.0;
    pub const LIGHT_YEAR: f64 = 9_460_730_472_580_800.0;
}

/// Pressure conversion factors (to Pascals)
pub mod pressure {
    pub const BAR: f64 = 100_000.0;
    pub const ATM: f64 = 101_325.0;
    pub const TORR: f64 = 101_325.0 / 760.0;
    pub const PSI: f64 = 6_894.757_293_168_361;
}

/// Angle conversion factors (to radians)
pub mod angle {
    use core::f64::consts::PI;
    pub const DEGREE: f64 = PI / 180.0;
    pub const ARCMIN: f64 = PI / (180.0 * 60.0);
    pub const ARCSEC: f64 = PI / (180.0 * 3600.0);
}
