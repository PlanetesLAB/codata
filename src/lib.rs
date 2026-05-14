#![no_std]

pub mod astro;
mod constants;
pub mod units;

pub use astro::ASTRO_CONSTANTS;
pub use constants::*;
pub use constants::{CONSTANTS, Constant};

/// Common aliases for physical and astronomical constants.
pub mod aliases {
    pub use crate::astro::iau2015::{AU, L_SUN, M_EARTH, M_JUP, M_SUN, PC, R_EARTH, R_JUP, R_SUN};
    pub use crate::astro::iau2015::{
        AU_CGS, AU_SI, L_SUN_CGS, L_SUN_SI, M_EARTH_CGS, M_EARTH_SI, M_JUP_CGS, M_JUP_SI,
        M_SUN_CGS, M_SUN_SI, PC_CGS, PC_SI, R_EARTH_CGS, R_EARTH_SI, R_JUP_CGS, R_JUP_SI,
        R_SUN_CGS, R_SUN_SI,
    };
    use crate::constants::{
        AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT, ELECTRON_MASS, ELEMENTARY_CHARGE,
        MOLAR_GAS_CONSTANT, NEUTRON_MASS, NEWTONIAN_CONSTANT_OF_GRAVITATION, PLANCK_CONSTANT,
        PROTON_MASS, REDUCED_PLANCK_CONSTANT, SPEED_OF_LIGHT_IN_VACUUM, STEFAN_BOLTZMANN_CONSTANT,
    };
    pub use crate::constants::{
        BOLTZMANN_CONSTANT_CGS, BOLTZMANN_CONSTANT_SI,
        ELECTRON_MASS_CGS, ELECTRON_MASS_SI,
        MOLAR_GAS_CONSTANT_CGS, MOLAR_GAS_CONSTANT_SI, NEUTRON_MASS_CGS, NEUTRON_MASS_SI,
        NEWTONIAN_CONSTANT_OF_GRAVITATION_CGS, NEWTONIAN_CONSTANT_OF_GRAVITATION_SI,
        PLANCK_CONSTANT_CGS, PLANCK_CONSTANT_SI, PROTON_MASS_CGS, PROTON_MASS_SI,
        REDUCED_PLANCK_CONSTANT_CGS, REDUCED_PLANCK_CONSTANT_SI, SPEED_OF_LIGHT_IN_VACUUM_CGS,
        SPEED_OF_LIGHT_IN_VACUUM_SI, STEFAN_BOLTZMANN_CONSTANT_CGS, STEFAN_BOLTZMANN_CONSTANT_SI,
    };

    /// Speed of light in vacuum
    pub const C: f64 = SPEED_OF_LIGHT_IN_VACUUM;
    pub const C_SI: f64 = SPEED_OF_LIGHT_IN_VACUUM_SI;
    pub const C_CGS: f64 = SPEED_OF_LIGHT_IN_VACUUM_CGS;

    /// Newtonian constant of gravitation
    pub const G: f64 = NEWTONIAN_CONSTANT_OF_GRAVITATION;
    pub const G_SI: f64 = NEWTONIAN_CONSTANT_OF_GRAVITATION_SI;
    pub const G_CGS: f64 = NEWTONIAN_CONSTANT_OF_GRAVITATION_CGS;

    /// Planck constant
    pub const H: f64 = PLANCK_CONSTANT;
    pub const H_SI: f64 = PLANCK_CONSTANT_SI;
    pub const H_CGS: f64 = PLANCK_CONSTANT_CGS;

    /// Reduced Planck constant (h/2pi)
    pub const HBAR: f64 = REDUCED_PLANCK_CONSTANT;
    pub const HBAR_SI: f64 = REDUCED_PLANCK_CONSTANT_SI;
    pub const HBAR_CGS: f64 = REDUCED_PLANCK_CONSTANT_CGS;

    /// Elementary charge
    pub const E: f64 = ELEMENTARY_CHARGE;

    /// Electron mass
    pub const ME: f64 = ELECTRON_MASS;
    pub const ME_SI: f64 = ELECTRON_MASS_SI;
    pub const ME_CGS: f64 = ELECTRON_MASS_CGS;

    /// Proton mass
    pub const MP: f64 = PROTON_MASS;
    pub const MP_SI: f64 = PROTON_MASS_SI;
    pub const MP_CGS: f64 = PROTON_MASS_CGS;

    /// Neutron mass
    pub const MN: f64 = NEUTRON_MASS;
    pub const MN_SI: f64 = NEUTRON_MASS_SI;
    pub const MN_CGS: f64 = NEUTRON_MASS_CGS;

    /// Avogadro constant
    pub const NA: f64 = AVOGADRO_CONSTANT;

    /// Molar gas constant
    pub const R: f64 = MOLAR_GAS_CONSTANT;
    pub const R_SI: f64 = MOLAR_GAS_CONSTANT_SI;
    pub const R_CGS: f64 = MOLAR_GAS_CONSTANT_CGS;

    /// Boltzmann constant
    pub const K: f64 = BOLTZMANN_CONSTANT;
    pub const K_SI: f64 = BOLTZMANN_CONSTANT_SI;
    pub const K_CGS: f64 = BOLTZMANN_CONSTANT_CGS;

    /// Stefan-Boltzmann constant
    pub const SIGMA: f64 = STEFAN_BOLTZMANN_CONSTANT;
    pub const SIGMA_SI: f64 = STEFAN_BOLTZMANN_CONSTANT_SI;
    pub const SIGMA_CGS: f64 = STEFAN_BOLTZMANN_CONSTANT_CGS;
}

/// Find a constant by its official name.
///
/// This searches both physical (CODATA) and astronomical (IAU) constants.
///
/// Returns `Some(Constant)` if found, `None` otherwise.
///
/// # Example
/// ```
/// use codata::find;
/// let c = find("speed of light in vacuum").unwrap();
/// assert_eq!(c.value, 299792458.0);
///
/// let au = find("Astronomical Unit").unwrap();
/// assert_eq!(au.value, 149597870700.0);
/// ```
#[must_use]
pub fn find(name: &str) -> Option<Constant> {
    CONSTANTS
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, c)| *c)
        .or_else(|| {
            ASTRO_CONSTANTS
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, c)| *c)
        })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
    use crate::constants::SPEED_OF_LIGHT_IN_VACUUM;

    #[test]
    fn test_speed_of_light() {
        assert_eq!(SPEED_OF_LIGHT_IN_VACUUM, 299_792_458.0);
        assert_eq!(aliases::C, 299_792_458.0);
    }

    #[test]
    fn test_find() {
        let c = find("speed of light in vacuum").unwrap();
        assert_eq!(c.value, 299_792_458.0);
        assert_eq!(c.unit, "m s^-1");
        assert_eq!(c.uncertainty, 0.0);

        let au = find("Astronomical Unit").unwrap();
        assert_eq!(au.value, 149_597_870_700.0);
    }

    #[test]
    fn test_astro_aliases() {
        assert_eq!(aliases::AU, 149_597_870_700.0);
        const { assert!(aliases::M_SUN > 1.9e30) };
    }

    #[test]
    fn test_gravitation() {
        let g = find("Newtonian constant of gravitation").unwrap();
        // CODATA 2022 value is 6.674 30(15) x 10^-11 m^3 kg^-1 s^-2
        assert!((g.value - 6.674_30e-11).abs() < 1e-16);
    }

    #[test]
    fn test_cgs_conversions() {
        // G_cgs = G_si * 10^3
        assert!((aliases::G_CGS / aliases::G_SI - 1000.0).abs() < 1e-10);
        // c_cgs = c_si * 100
        assert_eq!(aliases::C_CGS, aliases::C_SI * 100.0);
        // h_cgs = h_si * 10^7 (J -> erg)
        assert!((aliases::H_CGS / aliases::H_SI - 1e7).abs() < 1e-10);
        // sigma_cgs = sigma_si * 10^3 (W/m^2/K^4 -> erg/s/cm^2/K^4)
        // 10^7 (erg/s) / 10^4 (cm^2) = 10^3
        assert!((aliases::SIGMA_CGS / aliases::SIGMA_SI - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_units() {
        use crate::units::{length, time};
        assert_eq!(time::MINUTE, 60.0);
        assert_eq!(time::HOUR, 3600.0);
        assert_eq!(length::INCH, 0.0254);
        assert_eq!(length::LIGHT_YEAR, 9_460_730_472_580_800.0);
    }
}
