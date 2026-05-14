// Generated from CODATA 2022 constants data
// Source: https://physics.nist.gov/cuu/Constants/Table/allascii.txt

#![allow(clippy::excessive_precision)]

/// A physical constant value with its uncertainty and unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constant {
    pub value: f64,
    pub uncertainty: f64,
    pub unit: &'static str,
}

/// alpha particle-electron mass ratio
/// Uncertainty: 0.00000017
pub const ALPHA_PARTICLE_ELECTRON_MASS_RATIO: f64 = 7_294.299_541_710_000_09;

/// alpha particle mass
/// Unit: kg
/// Uncertainty: 0.0000000021e-27
pub const ALPHA_PARTICLE_MASS_SI: f64 = 6.644_657_344_999_999_81e-27;
/// alpha particle mass (CGS)
pub const ALPHA_PARTICLE_MASS_CGS: f64 = 6.644_657_344_999_999_13e-24;
pub const ALPHA_PARTICLE_MASS: f64 = ALPHA_PARTICLE_MASS_SI;

/// alpha particle mass energy equivalent
/// Unit: J
/// Uncertainty: 0.0000000019e-10
pub const ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_SI: f64 = 5.971_920_199_700_000_05e-10;
/// alpha particle mass energy equivalent (CGS)
pub const ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.005_971_920_199_699_999_95;
pub const ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT: f64 = ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_SI;

/// alpha particle mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.0000012
pub const ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 3_727.379_411_800_000_07;

/// alpha particle mass in u
/// Unit: u
/// Uncertainty: 0.000000000062
pub const ALPHA_PARTICLE_MASS_IN_U: f64 = 4.001_506_179_128_999_64;

/// alpha particle molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.0000000012e-3
pub const ALPHA_PARTICLE_MOLAR_MASS_SI: f64 = 0.004_001_506_183_300_000_34;
/// alpha particle molar mass (CGS)
pub const ALPHA_PARTICLE_MOLAR_MASS_CGS: f64 = 4.001_506_183_300_000_07;
pub const ALPHA_PARTICLE_MOLAR_MASS: f64 = ALPHA_PARTICLE_MOLAR_MASS_SI;

/// alpha particle-proton mass ratio
/// Uncertainty: 0.000000000070
pub const ALPHA_PARTICLE_PROTON_MASS_RATIO: f64 = 3.972_599_690_252_000_22;

/// alpha particle relative atomic mass
/// Uncertainty: 0.000000000062
pub const ALPHA_PARTICLE_RELATIVE_ATOMIC_MASS: f64 = 4.001_506_179_128_999_64;

/// alpha particle rms charge radius
/// Unit: m
/// Uncertainty: 0.0021e-15
pub const ALPHA_PARTICLE_RMS_CHARGE_RADIUS_SI: f64 = 1.678_499_999_999_999_96e-15;
/// alpha particle rms charge radius (CGS)
pub const ALPHA_PARTICLE_RMS_CHARGE_RADIUS_CGS: f64 = 1.678_499_999_999_999_87e-13;
pub const ALPHA_PARTICLE_RMS_CHARGE_RADIUS: f64 = ALPHA_PARTICLE_RMS_CHARGE_RADIUS_SI;

/// Angstrom star
/// Unit: m
/// Uncertainty: 0.00000090e-10
pub const ANGSTROM_STAR_SI: f64 = 1.000_014_949_999_999_94e-10;
/// Angstrom star (CGS)
pub const ANGSTROM_STAR_CGS: f64 = 1.000_014_949_999_999_99e-08;
pub const ANGSTROM_STAR: f64 = ANGSTROM_STAR_SI;

/// atomic mass constant
/// Unit: kg
/// Uncertainty: 0.00000000052e-27
pub const ATOMIC_MASS_CONSTANT_SI: f64 = 1.660_539_068_919_999_93e-27;
/// atomic mass constant (CGS)
pub const ATOMIC_MASS_CONSTANT_CGS: f64 = 1.660_539_068_920_000_11e-24;
pub const ATOMIC_MASS_CONSTANT: f64 = ATOMIC_MASS_CONSTANT_SI;

/// atomic mass constant energy equivalent
/// Unit: J
/// Uncertainty: 0.00000000046e-10
pub const ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_SI: f64 = 1.492_418_087_680_000_05e-10;
/// atomic mass constant energy equivalent (CGS)
pub const ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_CGS: f64 = 0.001_492_418_087_680_000_11;
pub const ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT: f64 = ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_SI;

/// atomic mass constant energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000029
pub const ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEV: f64 = 931.494_103_719_999_998;

/// atomic mass unit-electron volt relationship
/// Unit: eV
/// Uncertainty: 0.0000000029e8
pub const ATOMIC_MASS_UNIT_ELECTRON_VOLT_RELATIONSHIP: f64 = 931_494_103.720_000_029;

/// atomic mass unit-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000011e7
pub const ATOMIC_MASS_UNIT_HARTREE_RELATIONSHIP: f64 = 34_231_776.921_999_998_4;

/// atomic mass unit-hertz relationship
/// Unit: Hz
/// Uncertainty: 0.00000000070e23
pub const ATOMIC_MASS_UNIT_HERTZ_RELATIONSHIP: f64 = 2.252_342_721_849_999_96e+23;

/// atomic mass unit-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: 0.0000000023e14
pub const ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP_SI: f64 = 751_300_662_090_000.0;
/// atomic mass unit-inverse meter relationship (CGS)
pub const ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP_CGS: f64 = 7_513_006_620_900.0;
pub const ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP: f64 = ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP_SI;

/// atomic mass unit-joule relationship
/// Unit: J
/// Uncertainty: 0.00000000046e-10
pub const ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP_SI: f64 = 1.492_418_087_680_000_05e-10;
/// atomic mass unit-joule relationship (CGS)
pub const ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP_CGS: f64 = 0.001_492_418_087_680_000_11;
pub const ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP: f64 = ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP_SI;

/// atomic mass unit-kelvin relationship
/// Unit: K
/// Uncertainty: 0.00000000034e13
pub const ATOMIC_MASS_UNIT_KELVIN_RELATIONSHIP: f64 = 10_809_540_206_700.0;

/// atomic mass unit-kilogram relationship
/// Unit: kg
/// Uncertainty: 0.00000000052e-27
pub const ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP_SI: f64 = 1.660_539_068_919_999_93e-27;
/// atomic mass unit-kilogram relationship (CGS)
pub const ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP_CGS: f64 = 1.660_539_068_920_000_11e-24;
pub const ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP: f64 = ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP_SI;

/// atomic unit of 1st hyperpolarizability
/// Unit: C^3 m^3 J^-2
/// Uncertainty: 0.0000000015e-53
pub const ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY_SI: f64 = 3.206_361_299_599_999_94e-53;
/// atomic unit of 1st hyperpolarizability (CGS)
pub const ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY_CGS: f64 = 3.206_361_299_599_999_92e-61;
pub const ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY: f64 = ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY_SI;

/// atomic unit of 2nd hyperpolarizability
/// Unit: C^4 m^4 J^-3
/// Uncertainty: 0.0000000039e-65
pub const ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY_SI: f64 = 6.235_379_973_500_000_3e-65;
/// atomic unit of 2nd hyperpolarizability (CGS)
pub const ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY_CGS: f64 = 6.235_379_973_499_999_87e-78;
pub const ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY: f64 = ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY_SI;

/// atomic unit of action
/// Unit: J s
/// Uncertainty: (exact)
pub const ATOMIC_UNIT_OF_ACTION_SI: f64 = 1.054_571_816_999_999_99e-34;
/// atomic unit of action (CGS)
pub const ATOMIC_UNIT_OF_ACTION_CGS: f64 = 1.054_571_816_999_999_96e-27;
pub const ATOMIC_UNIT_OF_ACTION: f64 = ATOMIC_UNIT_OF_ACTION_SI;

/// atomic unit of charge
/// Unit: C
/// Uncertainty: (exact)
pub const ATOMIC_UNIT_OF_CHARGE: f64 = 1.602_176_633_999_999_89e-19;

/// atomic unit of charge density
/// Unit: C m^-3
/// Uncertainty: 0.00000000051e12
pub const ATOMIC_UNIT_OF_CHARGE_DENSITY_SI: f64 = 1_081_202_386_770.0;
/// atomic unit of charge density (CGS)
pub const ATOMIC_UNIT_OF_CHARGE_DENSITY_CGS: f64 = 1_081_202.386_769_999_98;
pub const ATOMIC_UNIT_OF_CHARGE_DENSITY: f64 = ATOMIC_UNIT_OF_CHARGE_DENSITY_SI;

/// atomic unit of current
/// Unit: A
/// Uncertainty: 0.0000000000072e-3
pub const ATOMIC_UNIT_OF_CURRENT: f64 = 0.006_623_618_237_508_200_13;

/// atomic unit of electric dipole mom.
/// Unit: C m
/// Uncertainty: 0.0000000013e-30
pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOM_SI: f64 = 8.478_353_619_800_000_69e-30;
/// atomic unit of electric dipole mom. (CGS)
pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOM_CGS: f64 = 8.478_353_619_800_000_97e-28;
pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOM: f64 = ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOM_SI;

/// atomic unit of electric field
/// Unit: V m^-1
/// Uncertainty: 0.00000000080e11
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD_SI: f64 = 514_220_675_112.0;
/// atomic unit of electric field (CGS)
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD_CGS: f64 = 5_142_206_751.119_999_89;
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: f64 = ATOMIC_UNIT_OF_ELECTRIC_FIELD_SI;

/// atomic unit of electric field gradient
/// Unit: V m^-2
/// Uncertainty: 0.0000000030e21
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT_SI: f64 = 9.717_362_442_400_000_11e+21;
/// atomic unit of electric field gradient (CGS)
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT_CGS: f64 = 971_736_244_240_000_000.0;
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT: f64 = ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT_SI;

/// atomic unit of electric polarizability
/// Unit: C^2 m^2 J^-1
/// Uncertainty: 0.00000000051e-41
pub const ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY_SI: f64 = 1.648_777_272_12e-41;
/// atomic unit of electric polarizability (CGS)
pub const ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY_CGS: f64 = 1.648_777_272_120_000_12e-44;
pub const ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY: f64 = ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY_SI;

/// atomic unit of electric potential
/// Unit: V
/// Uncertainty: 0.000000000030
pub const ATOMIC_UNIT_OF_ELECTRIC_POTENTIAL: f64 = 27.211_386_245_981_000_1;

/// atomic unit of electric quadrupole mom.
/// Unit: C m^2
/// Uncertainty: 0.0000000014e-40
pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOM_SI: f64 = 4.486_551_518_500_000_02e-40;
/// atomic unit of electric quadrupole mom. (CGS)
pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOM_CGS: f64 = 4.486_551_518_500_000_3e-36;
pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOM: f64 = ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOM_SI;

/// atomic unit of energy
/// Unit: J
/// Uncertainty: 0.0000000000048e-18
pub const ATOMIC_UNIT_OF_ENERGY_SI: f64 = 4.359_744_722_205_999_89e-18;
/// atomic unit of energy (CGS)
pub const ATOMIC_UNIT_OF_ENERGY_CGS: f64 = 4.359_744_722_205_999_88e-11;
pub const ATOMIC_UNIT_OF_ENERGY: f64 = ATOMIC_UNIT_OF_ENERGY_SI;

/// atomic unit of force
/// Unit: N
/// Uncertainty: 0.0000000013e-8
pub const ATOMIC_UNIT_OF_FORCE_SI: f64 = 8.238_723_503_800_000_3e-08;
/// atomic unit of force (CGS)
pub const ATOMIC_UNIT_OF_FORCE_CGS: f64 = 0.008_238_723_503_799_999_86;
pub const ATOMIC_UNIT_OF_FORCE: f64 = ATOMIC_UNIT_OF_FORCE_SI;

/// atomic unit of length
/// Unit: m
/// Uncertainty: 0.00000000082e-11
pub const ATOMIC_UNIT_OF_LENGTH_SI: f64 = 5.291_772_105_439_999_77e-11;
/// atomic unit of length (CGS)
pub const ATOMIC_UNIT_OF_LENGTH_CGS: f64 = 5.291_772_105_440_000_1e-09;
pub const ATOMIC_UNIT_OF_LENGTH: f64 = ATOMIC_UNIT_OF_LENGTH_SI;

/// atomic unit of mag. dipole mom.
/// Unit: J T^-1
/// Uncertainty: 0.00000000058e-23
pub const ATOMIC_UNIT_OF_MAG_DIPOLE_MOM_SI: f64 = 1.854_802_013_149_999_92e-23;
/// atomic unit of mag. dipole mom. (CGS)
pub const ATOMIC_UNIT_OF_MAG_DIPOLE_MOM_CGS: f64 = 1.854_802_013_149_999_94e-20;
pub const ATOMIC_UNIT_OF_MAG_DIPOLE_MOM: f64 = ATOMIC_UNIT_OF_MAG_DIPOLE_MOM_SI;

/// atomic unit of mag. flux density
/// Unit: T
/// Uncertainty: 0.00000000073e5
pub const ATOMIC_UNIT_OF_MAG_FLUX_DENSITY_SI: f64 = 235_051.757_076_999_987;
/// atomic unit of mag. flux density (CGS)
pub const ATOMIC_UNIT_OF_MAG_FLUX_DENSITY_CGS: f64 = 2_350_517_570.769_999_98;
pub const ATOMIC_UNIT_OF_MAG_FLUX_DENSITY: f64 = ATOMIC_UNIT_OF_MAG_FLUX_DENSITY_SI;

/// atomic unit of magnetizability
/// Unit: J T^-2
/// Uncertainty: 0.0000000049e-29
pub const ATOMIC_UNIT_OF_MAGNETIZABILITY_SI: f64 = 7.891_036_579_399_999_45e-29;
/// atomic unit of magnetizability (CGS)
pub const ATOMIC_UNIT_OF_MAGNETIZABILITY_CGS: f64 = 7.891_036_579_400_000_01e-30;
pub const ATOMIC_UNIT_OF_MAGNETIZABILITY: f64 = ATOMIC_UNIT_OF_MAGNETIZABILITY_SI;

/// atomic unit of mass
/// Unit: kg
/// Uncertainty: 0.0000000028e-31
pub const ATOMIC_UNIT_OF_MASS_SI: f64 = 9.109_383_713_899_999_82e-31;
/// atomic unit of mass (CGS)
pub const ATOMIC_UNIT_OF_MASS_CGS: f64 = 9.109_383_713_899_999_97e-28;
pub const ATOMIC_UNIT_OF_MASS: f64 = ATOMIC_UNIT_OF_MASS_SI;

/// atomic unit of momentum
/// Unit: kg m s^-1
/// Uncertainty: 0.00000000031e-24
pub const ATOMIC_UNIT_OF_MOMENTUM_SI: f64 = 1.992_851_915_449_999_82e-24;
/// atomic unit of momentum (CGS)
pub const ATOMIC_UNIT_OF_MOMENTUM_CGS: f64 = 1.992_851_915_449_999_73e-19;
pub const ATOMIC_UNIT_OF_MOMENTUM: f64 = ATOMIC_UNIT_OF_MOMENTUM_SI;

/// atomic unit of permittivity
/// Unit: F m^-1
/// Uncertainty: 0.00000000017e-10
pub const ATOMIC_UNIT_OF_PERMITTIVITY_SI: f64 = 1.112_650_056_200_000_06e-10;
/// atomic unit of permittivity (CGS)
pub const ATOMIC_UNIT_OF_PERMITTIVITY_CGS: f64 = 1.112_650_056_200_000_14e-12;
pub const ATOMIC_UNIT_OF_PERMITTIVITY: f64 = ATOMIC_UNIT_OF_PERMITTIVITY_SI;

/// atomic unit of time
/// Unit: s
/// Uncertainty: 0.0000000000026e-17
pub const ATOMIC_UNIT_OF_TIME: f64 = 2.418_884_326_586_399_85e-17;

/// atomic unit of velocity
/// Unit: m s^-1
/// Uncertainty: 0.00000000034e6
pub const ATOMIC_UNIT_OF_VELOCITY_SI: f64 = 2_187_691.262_159_999_93;
/// atomic unit of velocity (CGS)
pub const ATOMIC_UNIT_OF_VELOCITY_CGS: f64 = 218_769_126.215_999_991;
pub const ATOMIC_UNIT_OF_VELOCITY: f64 = ATOMIC_UNIT_OF_VELOCITY_SI;

/// Avogadro constant
/// Unit: mol^-1
/// Uncertainty: (exact)
pub const AVOGADRO_CONSTANT: f64 = 6.022_140_759_999_999_87e+23;

/// Bohr magneton
/// Unit: J T^-1
/// Uncertainty: 0.0000000029e-24
pub const BOHR_MAGNETON_SI: f64 = 9.274_010_065_699_999_92e-24;
/// Bohr magneton (CGS)
pub const BOHR_MAGNETON_CGS: f64 = 9.274_010_065_699_999_26e-21;
pub const BOHR_MAGNETON: f64 = BOHR_MAGNETON_SI;

/// Bohr magneton in eV/T
/// Unit: eV T^-1
/// Uncertainty: 0.0000000018e-5
pub const BOHR_MAGNETON_IN_EV_OVER_T_SI: f64 = 5.788_381_798_199_999_99e-05;
/// Bohr magneton in eV/T (CGS)
pub const BOHR_MAGNETON_IN_EV_OVER_T_CGS: f64 = 5.788_381_798_199_999_96e-09;
pub const BOHR_MAGNETON_IN_EV_OVER_T: f64 = BOHR_MAGNETON_IN_EV_OVER_T_SI;

/// Bohr magneton in Hz/T
/// Unit: Hz T^-1
/// Uncertainty: 0.00000000044e10
pub const BOHR_MAGNETON_IN_HZ_OVER_T_SI: f64 = 13_996_244_917.100_000_4;
/// Bohr magneton in Hz/T (CGS)
pub const BOHR_MAGNETON_IN_HZ_OVER_T_CGS: f64 = 1_399_624.491_710_000_21;
pub const BOHR_MAGNETON_IN_HZ_OVER_T: f64 = BOHR_MAGNETON_IN_HZ_OVER_T_SI;

/// Bohr magneton in inverse meter per tesla
/// Unit: m^-1 T^-1
/// Uncertainty: 0.000000015
pub const BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA_SI: f64 = 46.686_447_719_000_000_2;
/// Bohr magneton in inverse meter per tesla (CGS)
pub const BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA_CGS: f64 = 4.668_644_771_900_001_01e-05;
pub const BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA: f64 = BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA_SI;

/// Bohr magneton in K/T
/// Unit: K T^-1
/// Uncertainty: 0.00000000021
pub const BOHR_MAGNETON_IN_K_OVER_T_SI: f64 = 0.671_713_814_720_000_024;
/// Bohr magneton in K/T (CGS)
pub const BOHR_MAGNETON_IN_K_OVER_T_CGS: f64 = 6.717_138_147_200_000_93e-05;
pub const BOHR_MAGNETON_IN_K_OVER_T: f64 = BOHR_MAGNETON_IN_K_OVER_T_SI;

/// Bohr radius
/// Unit: m
/// Uncertainty: 0.00000000082e-11
pub const BOHR_RADIUS_SI: f64 = 5.291_772_105_439_999_77e-11;
/// Bohr radius (CGS)
pub const BOHR_RADIUS_CGS: f64 = 5.291_772_105_440_000_1e-09;
pub const BOHR_RADIUS: f64 = BOHR_RADIUS_SI;

/// Boltzmann constant
/// Unit: J K^-1
/// Uncertainty: (exact)
pub const BOLTZMANN_CONSTANT_SI: f64 = 1.380_649_000_000_000_09e-23;
/// Boltzmann constant (CGS)
pub const BOLTZMANN_CONSTANT_CGS: f64 = 1.380_648_999_999_999_99e-16;
pub const BOLTZMANN_CONSTANT: f64 = BOLTZMANN_CONSTANT_SI;

/// Boltzmann constant in eV/K
/// Unit: eV K^-1
/// Uncertainty: (exact)
pub const BOLTZMANN_CONSTANT_IN_EV_OVER_K: f64 = 8.617_333_262_000_000_06e-05;

/// Boltzmann constant in Hz/K
/// Unit: Hz K^-1
/// Uncertainty: (exact)
pub const BOLTZMANN_CONSTANT_IN_HZ_OVER_K: f64 = 20_836_619_120.0;

/// Boltzmann constant in inverse meter per kelvin
/// Unit: m^-1 K^-1
/// Uncertainty: (exact)
pub const BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_KELVIN_SI: f64 = 69.503_480_039_999_999_5;
/// Boltzmann constant in inverse meter per kelvin (CGS)
pub const BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_KELVIN_CGS: f64 = 0.695_034_800_399_999_986;
pub const BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_KELVIN: f64 = BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_KELVIN_SI;

/// characteristic impedance of vacuum
/// Unit: ohm
/// Uncertainty: 0.000000059
pub const CHARACTERISTIC_IMPEDANCE_OF_VACUUM: f64 = 376.730_313_411_999_987;

/// classical electron radius
/// Unit: m
/// Uncertainty: 0.0000000013e-15
pub const CLASSICAL_ELECTRON_RADIUS_SI: f64 = 2.817_940_320_499_999_9e-15;
/// classical electron radius (CGS)
pub const CLASSICAL_ELECTRON_RADIUS_CGS: f64 = 2.817_940_320_499_999_83e-13;
pub const CLASSICAL_ELECTRON_RADIUS: f64 = CLASSICAL_ELECTRON_RADIUS_SI;

/// Compton wavelength
/// Unit: m
/// Uncertainty: 0.00000000076e-12
pub const COMPTON_WAVELENGTH_SI: f64 = 2.426_310_235_380_000_03e-12;
/// Compton wavelength (CGS)
pub const COMPTON_WAVELENGTH_CGS: f64 = 2.426_310_235_380_000_24e-10;
pub const COMPTON_WAVELENGTH: f64 = COMPTON_WAVELENGTH_SI;

/// conductance quantum
/// Unit: S
/// Uncertainty: (exact)
pub const CONDUCTANCE_QUANTUM: f64 = 7.748_091_729_000_000_37e-05;

/// conventional value of ampere-90
/// Unit: A
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_AMPERE_90: f64 = 1.000_000_088_870_000_03;

/// conventional value of coulomb-90
/// Unit: C
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_COULOMB_90: f64 = 1.000_000_088_870_000_03;

/// conventional value of farad-90
/// Unit: F
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_FARAD_90: f64 = 0.999_999_982_199_999_971;

/// conventional value of henry-90
/// Unit: H
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_HENRY_90: f64 = 1.000_000_017_789_999_92;

/// conventional value of Josephson constant
/// Unit: Hz V^-1
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_JOSEPHSON_CONSTANT: f64 = 483_597_900_000_000.0;

/// conventional value of ohm-90
/// Unit: ohm
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_OHM_90: f64 = 1.000_000_017_789_999_92;

/// conventional value of volt-90
/// Unit: V
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_VOLT_90: f64 = 1.000_000_106_659_999_94;

/// conventional value of von Klitzing constant
/// Unit: ohm
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_VON_KLITZING_CONSTANT: f64 = 25_812.807_000_000_000_7;

/// conventional value of watt-90
/// Unit: W
/// Uncertainty: (exact)
pub const CONVENTIONAL_VALUE_OF_WATT_90_SI: f64 = 1.000_000_195_529_999_97;
/// conventional value of watt-90 (CGS)
pub const CONVENTIONAL_VALUE_OF_WATT_90_CGS: f64 = 10_000_001.955_299_999_6;
pub const CONVENTIONAL_VALUE_OF_WATT_90: f64 = CONVENTIONAL_VALUE_OF_WATT_90_SI;

/// Copper x unit
/// Unit: m
/// Uncertainty: 0.00000028e-13
pub const COPPER_X_UNIT_SI: f64 = 1.002_076_970_000_000_01e-13;
/// Copper x unit (CGS)
pub const COPPER_X_UNIT_CGS: f64 = 1.002_076_969_999_999_97e-11;
pub const COPPER_X_UNIT: f64 = COPPER_X_UNIT_SI;

/// deuteron-electron mag. mom. ratio
/// Uncertainty: 0.000000012e-4
pub const DEUTERON_ELECTRON_MAG_MOM_RATIO: f64 = -0.000_466_434_555_000_000_005;

/// deuteron-electron mass ratio
/// Uncertainty: 0.000000063
pub const DEUTERON_ELECTRON_MASS_RATIO: f64 = 3_670.482_967_654_999_81;

/// deuteron g factor
/// Uncertainty: 0.0000000022
pub const DEUTERON_G_FACTOR: f64 = 0.857_438_233_500_000_035;

/// deuteron mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.000000011e-27
pub const DEUTERON_MAG_MOM_SI: f64 = 4.330_735_086_999_999_71e-27;
/// deuteron mag. mom. (CGS)
pub const DEUTERON_MAG_MOM_CGS: f64 = 4.330_735_086_999_999_68e-24;
pub const DEUTERON_MAG_MOM: f64 = DEUTERON_MAG_MOM_SI;

/// deuteron mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.000000012e-4
pub const DEUTERON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = 0.000_466_975_456_800_000_014;

/// deuteron mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.0000000022
pub const DEUTERON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = 0.857_438_233_500_000_035;

/// deuteron mass
/// Unit: kg
/// Uncertainty: 0.0000000010e-27
pub const DEUTERON_MASS_SI: f64 = 3.343_583_776_800_000_06e-27;
/// deuteron mass (CGS)
pub const DEUTERON_MASS_CGS: f64 = 3.343_583_776_799_999_81e-24;
pub const DEUTERON_MASS: f64 = DEUTERON_MASS_SI;

/// deuteron mass energy equivalent
/// Unit: J
/// Uncertainty: 0.00000000094e-10
pub const DEUTERON_MASS_ENERGY_EQUIVALENT_SI: f64 = 3.005_063_234_909_999_96e-10;
/// deuteron mass energy equivalent (CGS)
pub const DEUTERON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.003_005_063_234_909_999_96;
pub const DEUTERON_MASS_ENERGY_EQUIVALENT: f64 = DEUTERON_MASS_ENERGY_EQUIVALENT_SI;

/// deuteron mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000058
pub const DEUTERON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 1_875.612_945_000_000_08;

/// deuteron mass in u
/// Unit: u
/// Uncertainty: 0.000000000015
pub const DEUTERON_MASS_IN_U: f64 = 2.013_553_212_544_000_1;

/// deuteron molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000063e-3
pub const DEUTERON_MOLAR_MASS_SI: f64 = 0.002_013_553_214_659_999_8;
/// deuteron molar mass (CGS)
pub const DEUTERON_MOLAR_MASS_CGS: f64 = 2.013_553_214_659_999_92;
pub const DEUTERON_MOLAR_MASS: f64 = DEUTERON_MOLAR_MASS_SI;

/// deuteron-neutron mag. mom. ratio
/// Uncertainty: 0.00000011
pub const DEUTERON_NEUTRON_MAG_MOM_RATIO: f64 = -0.448_206_519_999_999_997;

/// deuteron-proton mag. mom. ratio
/// Uncertainty: 0.00000000079
pub const DEUTERON_PROTON_MAG_MOM_RATIO: f64 = 0.307_012_209_300_000_005;

/// deuteron-proton mass ratio
/// Uncertainty: 0.0000000000084
pub const DEUTERON_PROTON_MASS_RATIO: f64 = 1.999_007_501_269_900_01;

/// deuteron relative atomic mass
/// Uncertainty: 0.000000000015
pub const DEUTERON_RELATIVE_ATOMIC_MASS: f64 = 2.013_553_212_544_000_1;

/// deuteron rms charge radius
/// Unit: m
/// Uncertainty: 0.00027e-15
pub const DEUTERON_RMS_CHARGE_RADIUS_SI: f64 = 2.127_780_000_000_000_03e-15;
/// deuteron rms charge radius (CGS)
pub const DEUTERON_RMS_CHARGE_RADIUS_CGS: f64 = 2.127_780_000_000_000_1e-13;
pub const DEUTERON_RMS_CHARGE_RADIUS: f64 = DEUTERON_RMS_CHARGE_RADIUS_SI;

/// electron charge to mass quotient
/// Unit: C kg^-1
/// Uncertainty: 0.00000000055e11
pub const ELECTRON_CHARGE_TO_MASS_QUOTIENT_SI: f64 = -175_882_000_838.0;
/// electron charge to mass quotient (CGS)
pub const ELECTRON_CHARGE_TO_MASS_QUOTIENT_CGS: f64 = -175_882_000.838;
pub const ELECTRON_CHARGE_TO_MASS_QUOTIENT: f64 = ELECTRON_CHARGE_TO_MASS_QUOTIENT_SI;

/// electron-deuteron mag. mom. ratio
/// Uncertainty: 0.0000056
pub const ELECTRON_DEUTERON_MAG_MOM_RATIO: f64 = -2_143.923_492_100_000_2;

/// electron-deuteron mass ratio
/// Uncertainty: 0.000000000047e-4
pub const ELECTRON_DEUTERON_MASS_RATIO: f64 = 0.000_272_443_710_762_900_013;

/// electron g factor
/// Uncertainty: 0.00000000000036
pub const ELECTRON_G_FACTOR: f64 = -2.002_319_304_360_919_98;

/// electron gyromag. ratio
/// Unit: s^-1 T^-1
/// Uncertainty: 0.00000000055e11
pub const ELECTRON_GYROMAG_RATIO_SI: f64 = 176_085_962_784.0;
/// electron gyromag. ratio (CGS)
pub const ELECTRON_GYROMAG_RATIO_CGS: f64 = 17_608_596.278_400_000_2;
pub const ELECTRON_GYROMAG_RATIO: f64 = ELECTRON_GYROMAG_RATIO_SI;

/// electron gyromag. ratio in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.0000087
pub const ELECTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI: f64 = 28_024.951_386_100_001_4;
/// electron gyromag. ratio in MHz/T (CGS)
pub const ELECTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_CGS: f64 = 2.802_495_138_610_000_32;
pub const ELECTRON_GYROMAG_RATIO_IN_MHZ_OVER_T: f64 = ELECTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI;

/// electron-helion mass ratio
/// Uncertainty: 0.000000000053e-4
pub const ELECTRON_HELION_MASS_RATIO: f64 = 0.000_181_954_307_464_899_989;

/// electron mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.0000000029e-24
pub const ELECTRON_MAG_MOM_SI: f64 = -9.284_764_691_700_000_03e-24;
/// electron mag. mom. (CGS)
pub const ELECTRON_MAG_MOM_CGS: f64 = -9.284_764_691_700_000_51e-21;
pub const ELECTRON_MAG_MOM: f64 = ELECTRON_MAG_MOM_SI;

/// electron mag. mom. anomaly
/// Uncertainty: 0.00000000018e-3
pub const ELECTRON_MAG_MOM_ANOMALY: f64 = 0.001_159_652_180_459_999_97;

/// electron mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000000000018
pub const ELECTRON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = -1.001_159_652_180_459_99;

/// electron mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.000000032
pub const ELECTRON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = -1_838.281_971_876_999_93;

/// electron mass
/// Unit: kg
/// Uncertainty: 0.0000000028e-31
pub const ELECTRON_MASS_SI: f64 = 9.109_383_713_899_999_82e-31;
/// electron mass (CGS)
pub const ELECTRON_MASS_CGS: f64 = 9.109_383_713_899_999_97e-28;
pub const ELECTRON_MASS: f64 = ELECTRON_MASS_SI;

/// electron mass energy equivalent
/// Unit: J
/// Uncertainty: 0.0000000026e-14
pub const ELECTRON_MASS_ENERGY_EQUIVALENT_SI: f64 = 8.187_105_788_000_000_12e-14;
/// electron mass energy equivalent (CGS)
pub const ELECTRON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 8.187_105_787_999_999_96e-07;
pub const ELECTRON_MASS_ENERGY_EQUIVALENT: f64 = ELECTRON_MASS_ENERGY_EQUIVALENT_SI;

/// electron mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000000016
pub const ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 0.510_998_950_690_000_009;

/// electron mass in u
/// Unit: u
/// Uncertainty: 0.000000000097e-4
pub const ELECTRON_MASS_IN_U: f64 = 0.000_548_579_909_044_099_971;

/// electron molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.0000000017e-7
pub const ELECTRON_MOLAR_MASS_SI: f64 = 5.485_799_096_199_999_47e-07;
/// electron molar mass (CGS)
pub const ELECTRON_MOLAR_MASS_CGS: f64 = 0.000_548_579_909_619_999_976;
pub const ELECTRON_MOLAR_MASS: f64 = ELECTRON_MOLAR_MASS_SI;

/// electron-muon mag. mom. ratio
/// Uncertainty: 0.0000046
pub const ELECTRON_MUON_MAG_MOM_RATIO: f64 = 206.766_988_099_999_992;

/// electron-muon mass ratio
/// Uncertainty: 0.00000011e-3
pub const ELECTRON_MUON_MASS_RATIO: f64 = 0.004_836_331_699_999_999_94;

/// electron-neutron mag. mom. ratio
/// Uncertainty: 0.00023
pub const ELECTRON_NEUTRON_MAG_MOM_RATIO: f64 = 960.920_479_999_999_998;

/// electron-neutron mass ratio
/// Uncertainty: 0.0000000022e-4
pub const ELECTRON_NEUTRON_MASS_RATIO: f64 = 0.000_543_867_344_160_000_048;

/// electron-proton mag. mom. ratio
/// Uncertainty: 0.00000019
pub const ELECTRON_PROTON_MAG_MOM_RATIO: f64 = -658.210_687_890_000_031;

/// electron-proton mass ratio
/// Uncertainty: 0.000000000094e-4
pub const ELECTRON_PROTON_MASS_RATIO: f64 = 0.000_544_617_021_488_900_022;

/// electron relative atomic mass
/// Uncertainty: 0.000000000097e-4
pub const ELECTRON_RELATIVE_ATOMIC_MASS: f64 = 0.000_548_579_909_044_099_971;

/// electron-tau mass ratio
/// Uncertainty: 0.00019e-4
pub const ELECTRON_TAU_MASS_RATIO: f64 = 0.000_287_584_999_999_999_978;

/// electron to alpha particle mass ratio
/// Uncertainty: 0.000000000032e-4
pub const ELECTRON_TO_ALPHA_PARTICLE_MASS_RATIO: f64 = 0.000_137_093_355_473_299_989;

/// electron to shielded helion mag. mom. ratio
/// Uncertainty: 0.00000070
pub const ELECTRON_TO_SHIELDED_HELION_MAG_MOM_RATIO: f64 = 864.058_239_859_999_958;

/// electron to shielded proton mag. mom. ratio
/// Uncertainty: 0.0000027
pub const ELECTRON_TO_SHIELDED_PROTON_MAG_MOM_RATIO: f64 = -658.227_585_599_999_998;

/// electron-triton mass ratio
/// Uncertainty: 0.000000000068e-4
pub const ELECTRON_TRITON_MASS_RATIO: f64 = 0.000_181_920_006_232_700_004;

/// electron volt
/// Unit: J
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_SI: f64 = 1.602_176_633_999_999_89e-19;
/// electron volt (CGS)
pub const ELECTRON_VOLT_CGS: f64 = 1.602_176_633_999_999_93e-12;
pub const ELECTRON_VOLT: f64 = ELECTRON_VOLT_SI;

/// electron volt-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.00000000033e-9
pub const ELECTRON_VOLT_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 1.073_544_100_829_999_91e-09;

/// electron volt-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000040e-2
pub const ELECTRON_VOLT_HARTREE_RELATIONSHIP: f64 = 0.036_749_322_175_664_997_4;

/// electron volt-hertz relationship
/// Unit: Hz
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_HERTZ_RELATIONSHIP: f64 = 241_798_924_200_000.0;

/// electron volt-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP_SI: f64 = 806_554.393_700_000_015;
/// electron volt-inverse meter relationship (CGS)
pub const ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP_CGS: f64 = 8_065.543_937_000_000_37;
pub const ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP: f64 = ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP_SI;

/// electron volt-joule relationship
/// Unit: J
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_JOULE_RELATIONSHIP_SI: f64 = 1.602_176_633_999_999_89e-19;
/// electron volt-joule relationship (CGS)
pub const ELECTRON_VOLT_JOULE_RELATIONSHIP_CGS: f64 = 1.602_176_633_999_999_93e-12;
pub const ELECTRON_VOLT_JOULE_RELATIONSHIP: f64 = ELECTRON_VOLT_JOULE_RELATIONSHIP_SI;

/// electron volt-kelvin relationship
/// Unit: K
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_KELVIN_RELATIONSHIP: f64 = 11_604.518_120_000_000_6;

/// electron volt-kilogram relationship
/// Unit: kg
/// Uncertainty: (exact)
pub const ELECTRON_VOLT_KILOGRAM_RELATIONSHIP_SI: f64 = 1.782_661_920_999_999_95e-36;
/// electron volt-kilogram relationship (CGS)
pub const ELECTRON_VOLT_KILOGRAM_RELATIONSHIP_CGS: f64 = 1.782_661_920_999_999_86e-33;
pub const ELECTRON_VOLT_KILOGRAM_RELATIONSHIP: f64 = ELECTRON_VOLT_KILOGRAM_RELATIONSHIP_SI;

/// elementary charge
/// Unit: C
/// Uncertainty: (exact)
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_633_999_999_89e-19;

/// elementary charge over h-bar
/// Unit: A J^-1
/// Uncertainty: (exact)
pub const ELEMENTARY_CHARGE_OVER_H_BAR_SI: f64 = 1_519_267_447_000_000.0;
/// elementary charge over h-bar (CGS)
pub const ELEMENTARY_CHARGE_OVER_H_BAR_CGS: f64 = 151_926_744.699_999_988;
pub const ELEMENTARY_CHARGE_OVER_H_BAR: f64 = ELEMENTARY_CHARGE_OVER_H_BAR_SI;

/// Faraday constant
/// Unit: C mol^-1
/// Uncertainty: (exact)
pub const FARADAY_CONSTANT: f64 = 96_485.332_120_000_006_4;

/// Fermi coupling constant
/// Unit: `GeV`^-2
/// Uncertainty: 0.0000006e-5
pub const FERMI_COUPLING_CONSTANT: f64 = 1.166_378_699_999_999_95e-05;

/// fine-structure constant
/// Uncertainty: 0.0000000011e-3
pub const FINE_STRUCTURE_CONSTANT: f64 = 0.007_297_352_564_299_999_96;

/// first radiation constant
/// Unit: W m^2
/// Uncertainty: (exact)
pub const FIRST_RADIATION_CONSTANT_SI: f64 = 3.741_771_852_000_000_16e-16;
/// first radiation constant (CGS)
pub const FIRST_RADIATION_CONSTANT_CGS: f64 = 3.741_771_851_999_999_95e-05;
pub const FIRST_RADIATION_CONSTANT: f64 = FIRST_RADIATION_CONSTANT_SI;

/// first radiation constant for spectral radiance
/// Unit: W m^2 sr^-1
/// Uncertainty: (exact)
pub const FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE_SI: f64 = 1.191_042_972_000_000_12e-16;
/// first radiation constant for spectral radiance (CGS)
pub const FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE_CGS: f64 = 1.191_042_972_000_000_06e-05;
pub const FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE: f64 = FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE_SI;

/// hartree-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.00000000091e-8
pub const HARTREE_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 2.921_262_317_969_999_9e-08;

/// hartree-electron volt relationship
/// Unit: eV
/// Uncertainty: 0.000000000030
pub const HARTREE_ELECTRON_VOLT_RELATIONSHIP: f64 = 27.211_386_245_981_000_1;

/// Hartree energy
/// Unit: J
/// Uncertainty: 0.0000000000048e-18
pub const HARTREE_ENERGY_SI: f64 = 4.359_744_722_205_999_89e-18;
/// Hartree energy (CGS)
pub const HARTREE_ENERGY_CGS: f64 = 4.359_744_722_205_999_88e-11;
pub const HARTREE_ENERGY: f64 = HARTREE_ENERGY_SI;

/// Hartree energy in eV
/// Unit: eV
/// Uncertainty: 0.000000000030
pub const HARTREE_ENERGY_IN_EV: f64 = 27.211_386_245_981_000_1;

/// hartree-hertz relationship
/// Unit: Hz
/// Uncertainty: 0.0000000000072e15
pub const HARTREE_HERTZ_RELATIONSHIP: f64 = 6_579_683_920_499_900.0;

/// hartree-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: 0.0000000000024e7
pub const HARTREE_INVERSE_METER_RELATIONSHIP_SI: f64 = 21_947_463.136_314_000_9;
/// hartree-inverse meter relationship (CGS)
pub const HARTREE_INVERSE_METER_RELATIONSHIP_CGS: f64 = 219_474.631_363_140_012;
pub const HARTREE_INVERSE_METER_RELATIONSHIP: f64 = HARTREE_INVERSE_METER_RELATIONSHIP_SI;

/// hartree-joule relationship
/// Unit: J
/// Uncertainty: 0.0000000000048e-18
pub const HARTREE_JOULE_RELATIONSHIP_SI: f64 = 4.359_744_722_205_999_89e-18;
/// hartree-joule relationship (CGS)
pub const HARTREE_JOULE_RELATIONSHIP_CGS: f64 = 4.359_744_722_205_999_88e-11;
pub const HARTREE_JOULE_RELATIONSHIP: f64 = HARTREE_JOULE_RELATIONSHIP_SI;

/// hartree-kelvin relationship
/// Unit: K
/// Uncertainty: 0.0000000000034e5
pub const HARTREE_KELVIN_RELATIONSHIP: f64 = 315_775.024_803_980_021;

/// hartree-kilogram relationship
/// Unit: kg
/// Uncertainty: 0.0000000000053e-35
pub const HARTREE_KILOGRAM_RELATIONSHIP_SI: f64 = 4.850_870_209_541_900_36e-35;
/// hartree-kilogram relationship (CGS)
pub const HARTREE_KILOGRAM_RELATIONSHIP_CGS: f64 = 4.850_870_209_541_900_15e-32;
pub const HARTREE_KILOGRAM_RELATIONSHIP: f64 = HARTREE_KILOGRAM_RELATIONSHIP_SI;

/// helion-electron mass ratio
/// Uncertainty: 0.00000016
pub const HELION_ELECTRON_MASS_RATIO: f64 = 5_495.885_279_840_000_29;

/// helion g factor
/// Uncertainty: 0.0000000034
pub const HELION_G_FACTOR: f64 = -4.255_250_699_500_000_34;

/// helion mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.00000000093e-26
pub const HELION_MAG_MOM_SI: f64 = -1.074_617_551_979_999_98e-26;
/// helion mag. mom. (CGS)
pub const HELION_MAG_MOM_CGS: f64 = -1.074_617_551_979_999_94e-23;
pub const HELION_MAG_MOM: f64 = HELION_MAG_MOM_SI;

/// helion mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000000094e-3
pub const HELION_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = -0.001_158_740_980_830_000_02;

/// helion mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.0000000017
pub const HELION_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = -2.127_625_349_800_000_18;

/// helion mass
/// Unit: kg
/// Uncertainty: 0.0000000016e-27
pub const HELION_MASS_SI: f64 = 5.006_412_786_199_999_95e-27;
/// helion mass (CGS)
pub const HELION_MASS_CGS: f64 = 5.006_412_786_200_000_01e-24;
pub const HELION_MASS: f64 = HELION_MASS_SI;

/// helion mass energy equivalent
/// Unit: J
/// Uncertainty: 0.0000000014e-10
pub const HELION_MASS_ENERGY_EQUIVALENT_SI: f64 = 4.499_539_418_499_999_94e-10;
/// helion mass energy equivalent (CGS)
pub const HELION_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.004_499_539_418_499_999_73;
pub const HELION_MASS_ENERGY_EQUIVALENT: f64 = HELION_MASS_ENERGY_EQUIVALENT_SI;

/// helion mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000088
pub const HELION_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 2_808.391_611_120_000_11;

/// helion mass in u
/// Unit: u
/// Uncertainty: 0.000000000074
pub const HELION_MASS_IN_U: f64 = 3.014_932_246_932_000_13;

/// helion molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000094e-3
pub const HELION_MOLAR_MASS_SI: f64 = 0.003_014_932_250_099_999_85;
/// helion molar mass (CGS)
pub const HELION_MOLAR_MASS_CGS: f64 = 3.014_932_250_099_999_77;
pub const HELION_MOLAR_MASS: f64 = HELION_MOLAR_MASS_SI;

/// helion-proton mass ratio
/// Uncertainty: 0.000000000070
pub const HELION_PROTON_MASS_RATIO: f64 = 2.993_152_671_552_000_04;

/// helion relative atomic mass
/// Uncertainty: 0.000000000074
pub const HELION_RELATIVE_ATOMIC_MASS: f64 = 3.014_932_246_932_000_13;

/// helion shielding shift
/// Uncertainty: 0.0000023e-5
pub const HELION_SHIELDING_SHIFT: f64 = 5.996_702_900_000_000_3e-05;

/// hertz-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.0000000014e-24
pub const HERTZ_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 4.439_821_659_000_000_06e-24;

/// hertz-electron volt relationship
/// Unit: eV
/// Uncertainty: (exact)
pub const HERTZ_ELECTRON_VOLT_RELATIONSHIP: f64 = 4.135_667_696_000_000_34e-15;

/// hertz-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000017e-16
pub const HERTZ_HARTREE_RELATIONSHIP: f64 = 1.519_829_846_057_400_02e-16;

/// hertz-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: (exact)
pub const HERTZ_INVERSE_METER_RELATIONSHIP_SI: f64 = 3.335_640_950_999_999_91e-09;
/// hertz-inverse meter relationship (CGS)
pub const HERTZ_INVERSE_METER_RELATIONSHIP_CGS: f64 = 3.335_640_950_999_999_98e-11;
pub const HERTZ_INVERSE_METER_RELATIONSHIP: f64 = HERTZ_INVERSE_METER_RELATIONSHIP_SI;

/// hertz-joule relationship
/// Unit: J
/// Uncertainty: (exact)
pub const HERTZ_JOULE_RELATIONSHIP_SI: f64 = 6.626_070_149_999_999_83e-34;
/// hertz-joule relationship (CGS)
pub const HERTZ_JOULE_RELATIONSHIP_CGS: f64 = 6.626_070_149_999_999_92e-27;
pub const HERTZ_JOULE_RELATIONSHIP: f64 = HERTZ_JOULE_RELATIONSHIP_SI;

/// hertz-kelvin relationship
/// Unit: K
/// Uncertainty: (exact)
pub const HERTZ_KELVIN_RELATIONSHIP: f64 = 4.799_243_072_999_999_71e-11;

/// hertz-kilogram relationship
/// Unit: kg
/// Uncertainty: (exact)
pub const HERTZ_KILOGRAM_RELATIONSHIP_SI: f64 = 7.372_497_322_999_999_41e-51;
/// hertz-kilogram relationship (CGS)
pub const HERTZ_KILOGRAM_RELATIONSHIP_CGS: f64 = 7.372_497_322_999_999_03e-48;
pub const HERTZ_KILOGRAM_RELATIONSHIP: f64 = HERTZ_KILOGRAM_RELATIONSHIP_SI;

/// hyperfine transition frequency of Cs-133
/// Unit: Hz
/// Uncertainty: (exact)
pub const HYPERFINE_TRANSITION_FREQUENCY_OF_CS_133: f64 = 9_192_631_770.0;

/// inverse fine-structure constant
/// Uncertainty: 0.000000021
pub const INVERSE_FINE_STRUCTURE_CONSTANT: f64 = 137.035_999_177_000_008;

/// inverse meter-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.00000000041e-15
pub const INVERSE_METER_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 1.331_025_048_24e-15;

/// inverse meter-electron volt relationship
/// Unit: eV
/// Uncertainty: (exact)
pub const INVERSE_METER_ELECTRON_VOLT_RELATIONSHIP: f64 = 1.239_841_984_000_000_01e-06;

/// inverse meter-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000050e-8
pub const INVERSE_METER_HARTREE_RELATIONSHIP: f64 = 4.556_335_252_913_200_15e-08;

/// inverse meter-hertz relationship
/// Unit: Hz
/// Uncertainty: (exact)
pub const INVERSE_METER_HERTZ_RELATIONSHIP: f64 = 299_792_458.0;

/// inverse meter-joule relationship
/// Unit: J
/// Uncertainty: (exact)
pub const INVERSE_METER_JOULE_RELATIONSHIP_SI: f64 = 1.986_445_856_999_999_92e-25;
/// inverse meter-joule relationship (CGS)
pub const INVERSE_METER_JOULE_RELATIONSHIP_CGS: f64 = 1.986_445_857_000_000_11e-18;
pub const INVERSE_METER_JOULE_RELATIONSHIP: f64 = INVERSE_METER_JOULE_RELATIONSHIP_SI;

/// inverse meter-kelvin relationship
/// Unit: K
/// Uncertainty: (exact)
pub const INVERSE_METER_KELVIN_RELATIONSHIP: f64 = 0.014_387_768_77;

/// inverse meter-kilogram relationship
/// Unit: kg
/// Uncertainty: (exact)
pub const INVERSE_METER_KILOGRAM_RELATIONSHIP_SI: f64 = 2.210_219_094_000_000_11e-42;
/// inverse meter-kilogram relationship (CGS)
pub const INVERSE_METER_KILOGRAM_RELATIONSHIP_CGS: f64 = 2.210_219_094_000_000_1e-39;
pub const INVERSE_METER_KILOGRAM_RELATIONSHIP: f64 = INVERSE_METER_KILOGRAM_RELATIONSHIP_SI;

/// inverse of conductance quantum
/// Unit: ohm
/// Uncertainty: (exact)
pub const INVERSE_OF_CONDUCTANCE_QUANTUM: f64 = 12_906.403_720_000_000_2;

/// Josephson constant
/// Unit: Hz V^-1
/// Uncertainty: (exact)
pub const JOSEPHSON_CONSTANT: f64 = 483_597_848_400_000.0;

/// joule-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.0000000021e9
pub const JOULE_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 6_700_535_247.100_000_38;

/// joule-electron volt relationship
/// Unit: eV
/// Uncertainty: (exact)
pub const JOULE_ELECTRON_VOLT_RELATIONSHIP: f64 = 6.241_509_074e+18;

/// joule-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000025e17
pub const JOULE_HARTREE_RELATIONSHIP: f64 = 229_371_227_839_689_984.0;

/// joule-hertz relationship
/// Unit: Hz
/// Uncertainty: (exact)
pub const JOULE_HERTZ_RELATIONSHIP: f64 = 1.509_190_178_999_999_89e+33;

/// joule-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: (exact)
pub const JOULE_INVERSE_METER_RELATIONSHIP_SI: f64 = 5.034_116_566_999_999_95e+24;
/// joule-inverse meter relationship (CGS)
pub const JOULE_INVERSE_METER_RELATIONSHIP_CGS: f64 = 5.034_116_567_000_000_18e+22;
pub const JOULE_INVERSE_METER_RELATIONSHIP: f64 = JOULE_INVERSE_METER_RELATIONSHIP_SI;

/// joule-kelvin relationship
/// Unit: K
/// Uncertainty: (exact)
pub const JOULE_KELVIN_RELATIONSHIP: f64 = 7.242_970_516_000_000_18e+22;

/// joule-kilogram relationship
/// Unit: kg
/// Uncertainty: (exact)
pub const JOULE_KILOGRAM_RELATIONSHIP_SI: f64 = 1.112_650_055_999_999_99e-17;
/// joule-kilogram relationship (CGS)
pub const JOULE_KILOGRAM_RELATIONSHIP_CGS: f64 = 1.112_650_055_999_999_96e-14;
pub const JOULE_KILOGRAM_RELATIONSHIP: f64 = JOULE_KILOGRAM_RELATIONSHIP_SI;

/// kelvin-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.0000000029e-14
pub const KELVIN_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 9.251_087_288_400_000_04e-14;

/// kelvin-electron volt relationship
/// Unit: eV
/// Uncertainty: (exact)
pub const KELVIN_ELECTRON_VOLT_RELATIONSHIP: f64 = 8.617_333_262_000_000_06e-05;

/// kelvin-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000035e-6
pub const KELVIN_HARTREE_RELATIONSHIP: f64 = 3.166_811_563_456_400_01e-06;

/// kelvin-hertz relationship
/// Unit: Hz
/// Uncertainty: (exact)
pub const KELVIN_HERTZ_RELATIONSHIP: f64 = 20_836_619_120.0;

/// kelvin-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: (exact)
pub const KELVIN_INVERSE_METER_RELATIONSHIP_SI: f64 = 69.503_480_039_999_999_5;
/// kelvin-inverse meter relationship (CGS)
pub const KELVIN_INVERSE_METER_RELATIONSHIP_CGS: f64 = 0.695_034_800_399_999_986;
pub const KELVIN_INVERSE_METER_RELATIONSHIP: f64 = KELVIN_INVERSE_METER_RELATIONSHIP_SI;

/// kelvin-joule relationship
/// Unit: J
/// Uncertainty: (exact)
pub const KELVIN_JOULE_RELATIONSHIP_SI: f64 = 1.380_649_000_000_000_09e-23;
/// kelvin-joule relationship (CGS)
pub const KELVIN_JOULE_RELATIONSHIP_CGS: f64 = 1.380_648_999_999_999_99e-16;
pub const KELVIN_JOULE_RELATIONSHIP: f64 = KELVIN_JOULE_RELATIONSHIP_SI;

/// kelvin-kilogram relationship
/// Unit: kg
/// Uncertainty: (exact)
pub const KELVIN_KILOGRAM_RELATIONSHIP_SI: f64 = 1.536_179_187_000_000_09e-40;
/// kelvin-kilogram relationship (CGS)
pub const KELVIN_KILOGRAM_RELATIONSHIP_CGS: f64 = 1.536_179_187_000_000_19e-37;
pub const KELVIN_KILOGRAM_RELATIONSHIP: f64 = KELVIN_KILOGRAM_RELATIONSHIP_SI;

/// kilogram-atomic mass unit relationship
/// Unit: u
/// Uncertainty: 0.0000000019e26
pub const KILOGRAM_ATOMIC_MASS_UNIT_RELATIONSHIP: f64 = 6.022_140_753_699_999_96e+26;

/// kilogram-electron volt relationship
/// Unit: eV
/// Uncertainty: (exact)
pub const KILOGRAM_ELECTRON_VOLT_RELATIONSHIP: f64 = 5.609_588_602_999_999_76e+35;

/// kilogram-hartree relationship
/// Unit: `E_h`
/// Uncertainty: 0.0000000000022e34
pub const KILOGRAM_HARTREE_RELATIONSHIP: f64 = 2.061_485_788_741_500_06e+34;

/// kilogram-hertz relationship
/// Unit: Hz
/// Uncertainty: (exact)
pub const KILOGRAM_HERTZ_RELATIONSHIP: f64 = 1.356_392_488_999_999_99e+50;

/// kilogram-inverse meter relationship
/// Unit: m^-1
/// Uncertainty: (exact)
pub const KILOGRAM_INVERSE_METER_RELATIONSHIP_SI: f64 = 4.524_438_334_999_999_82e+41;
/// kilogram-inverse meter relationship (CGS)
pub const KILOGRAM_INVERSE_METER_RELATIONSHIP_CGS: f64 = 4.524_438_334_999_999_84e+39;
pub const KILOGRAM_INVERSE_METER_RELATIONSHIP: f64 = KILOGRAM_INVERSE_METER_RELATIONSHIP_SI;

/// kilogram-joule relationship
/// Unit: J
/// Uncertainty: (exact)
pub const KILOGRAM_JOULE_RELATIONSHIP_SI: f64 = 89_875_517_870_000_000.0;
/// kilogram-joule relationship (CGS)
pub const KILOGRAM_JOULE_RELATIONSHIP_CGS: f64 = 8.987_551_787_000_000_25e+23;
pub const KILOGRAM_JOULE_RELATIONSHIP: f64 = KILOGRAM_JOULE_RELATIONSHIP_SI;

/// kilogram-kelvin relationship
/// Unit: K
/// Uncertainty: (exact)
pub const KILOGRAM_KELVIN_RELATIONSHIP: f64 = 6.509_657_260_000_000_32e+39;

/// lattice parameter of silicon
/// Unit: m
/// Uncertainty: 0.000000089e-10
pub const LATTICE_PARAMETER_OF_SILICON_SI: f64 = 5.431_020_511_000_000_28e-10;
/// lattice parameter of silicon (CGS)
pub const LATTICE_PARAMETER_OF_SILICON_CGS: f64 = 5.431_020_511_000_000_36e-08;
pub const LATTICE_PARAMETER_OF_SILICON: f64 = LATTICE_PARAMETER_OF_SILICON_SI;

/// lattice spacing of ideal Si (220)
/// Unit: m
/// Uncertainty: 0.000000032e-10
pub const LATTICE_SPACING_OF_IDEAL_SI_220_SI: f64 = 1.920_155_715_999_999_89e-10;
/// lattice spacing of ideal Si (220) (CGS)
pub const LATTICE_SPACING_OF_IDEAL_SI_220_CGS: f64 = 1.920_155_716_000_000_06e-08;
pub const LATTICE_SPACING_OF_IDEAL_SI_220: f64 = LATTICE_SPACING_OF_IDEAL_SI_220_SI;

/// Loschmidt constant (273.15 K, 100 kPa)
/// Unit: m^-3
/// Uncertainty: (exact)
pub const LOSCHMIDT_CONSTANT_273_15_K_100_KPA_SI: f64 = 2.651_645_803_999_999_87e+25;
/// Loschmidt constant (273.15 K, 100 kPa) (CGS)
pub const LOSCHMIDT_CONSTANT_273_15_K_100_KPA_CGS: f64 = 2.651_645_803_999_999_59e+19;
pub const LOSCHMIDT_CONSTANT_273_15_K_100_KPA: f64 = LOSCHMIDT_CONSTANT_273_15_K_100_KPA_SI;

/// Loschmidt constant (273.15 K, 101.325 kPa)
/// Unit: m^-3
/// Uncertainty: (exact)
pub const LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA_SI: f64 = 2.686_780_111_000_000_06e+25;
/// Loschmidt constant (273.15 K, 101.325 kPa) (CGS)
pub const LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA_CGS: f64 = 2.686_780_111_000_000_1e+19;
pub const LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA: f64 = LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA_SI;

/// luminous efficacy
/// Unit: lm W^-1
/// Uncertainty: (exact)
pub const LUMINOUS_EFFICACY_SI: f64 = 683.0;
/// luminous efficacy (CGS)
pub const LUMINOUS_EFFICACY_CGS: f64 = 6.829_999_999_999_999_32e-05;
pub const LUMINOUS_EFFICACY: f64 = LUMINOUS_EFFICACY_SI;

/// mag. flux quantum
/// Unit: Wb
/// Uncertainty: (exact)
pub const MAG_FLUX_QUANTUM: f64 = 2.067_833_848_000_000_17e-15;

/// molar gas constant
/// Unit: J mol^-1 K^-1
/// Uncertainty: (exact)
pub const MOLAR_GAS_CONSTANT_SI: f64 = 8.314_462_618_000_000_3;
/// molar gas constant (CGS)
pub const MOLAR_GAS_CONSTANT_CGS: f64 = 83_144_626.180_000_007_2;
pub const MOLAR_GAS_CONSTANT: f64 = MOLAR_GAS_CONSTANT_SI;

/// molar mass constant
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000031e-3
pub const MOLAR_MASS_CONSTANT_SI: f64 = 0.001_000_000_001_049_999_95;
/// molar mass constant (CGS)
pub const MOLAR_MASS_CONSTANT_CGS: f64 = 1.000_000_001_049_999_86;
pub const MOLAR_MASS_CONSTANT: f64 = MOLAR_MASS_CONSTANT_SI;

/// molar mass of carbon-12
/// Unit: kg mol^-1
/// Uncertainty: 0.0000000037e-3
pub const MOLAR_MASS_OF_CARBON_12_SI: f64 = 0.012_000_000_012_599_999_4;
/// molar mass of carbon-12 (CGS)
pub const MOLAR_MASS_OF_CARBON_12_CGS: f64 = 12.000_000_012_599_999_3;
pub const MOLAR_MASS_OF_CARBON_12: f64 = MOLAR_MASS_OF_CARBON_12_SI;

/// molar Planck constant
/// Unit: J Hz^-1 mol^-1
/// Uncertainty: (exact)
pub const MOLAR_PLANCK_CONSTANT_SI: f64 = 3.990_312_712_000_000_01e-10;
/// molar Planck constant (CGS)
pub const MOLAR_PLANCK_CONSTANT_CGS: f64 = 0.003_990_312_711_999_999_96;
pub const MOLAR_PLANCK_CONSTANT: f64 = MOLAR_PLANCK_CONSTANT_SI;

/// molar volume of ideal gas (273.15 K, 100 kPa)
/// Unit: m^3 mol^-1
/// Uncertainty: (exact)
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_100_KPA_SI: f64 = 0.022_710_954_639_999_999_4;
/// molar volume of ideal gas (273.15 K, 100 kPa) (CGS)
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_100_KPA_CGS: f64 = 22_710.954_639_999_999_9;
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_100_KPA: f64 = MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_100_KPA_SI;

/// molar volume of ideal gas (273.15 K, 101.325 kPa)
/// Unit: m^3 mol^-1
/// Uncertainty: (exact)
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_101_325_KPA_SI: f64 = 0.022_413_969_539_999_998_4;
/// molar volume of ideal gas (273.15 K, 101.325 kPa) (CGS)
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_101_325_KPA_CGS: f64 = 22_413.969_539_999_998_2;
pub const MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_101_325_KPA: f64 = MOLAR_VOLUME_OF_IDEAL_GAS_273_15_K_101_325_KPA_SI;

/// molar volume of silicon
/// Unit: m^3 mol^-1
/// Uncertainty: 0.000000060e-5
pub const MOLAR_VOLUME_OF_SILICON_SI: f64 = 1.205_883_198_999_999_99e-05;
/// molar volume of silicon (CGS)
pub const MOLAR_VOLUME_OF_SILICON_CGS: f64 = 12.058_831_989_999_999_8;
pub const MOLAR_VOLUME_OF_SILICON: f64 = MOLAR_VOLUME_OF_SILICON_SI;

/// Molybdenum x unit
/// Unit: m
/// Uncertainty: 0.00000053e-13
pub const MOLYBDENUM_X_UNIT_SI: f64 = 1.002_099_520_000_000_05e-13;
/// Molybdenum x unit (CGS)
pub const MOLYBDENUM_X_UNIT_CGS: f64 = 1.002_099_520_000_000_07e-11;
pub const MOLYBDENUM_X_UNIT: f64 = MOLYBDENUM_X_UNIT_SI;

/// muon Compton wavelength
/// Unit: m
/// Uncertainty: 0.000000026e-14
pub const MUON_COMPTON_WAVELENGTH_SI: f64 = 1.173_444_109_999_999_96e-14;
/// muon Compton wavelength (CGS)
pub const MUON_COMPTON_WAVELENGTH_CGS: f64 = 1.173_444_109_999_999_93e-12;
pub const MUON_COMPTON_WAVELENGTH: f64 = MUON_COMPTON_WAVELENGTH_SI;

/// muon-electron mass ratio
/// Uncertainty: 0.0000046
pub const MUON_ELECTRON_MASS_RATIO: f64 = 206.768_282_699_999_986;

/// muon g factor
/// Uncertainty: 0.00000000082
pub const MUON_G_FACTOR: f64 = -2.002_331_841_230_000_19;

/// muon mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.00000010e-26
pub const MUON_MAG_MOM_SI: f64 = -4.490_448_299_999_999_82e-26;
/// muon mag. mom. (CGS)
pub const MUON_MAG_MOM_CGS: f64 = -4.490_448_299_999_999_54e-23;
pub const MUON_MAG_MOM: f64 = MUON_MAG_MOM_SI;

/// muon mag. mom. anomaly
/// Uncertainty: 0.00000041e-3
pub const MUON_MAG_MOM_ANOMALY: f64 = 0.001_165_920_620_000_000_1;

/// muon mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000011e-3
pub const MUON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = -0.004_841_970_479_999_999_94;

/// muon mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.00000020
pub const MUON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = -8.890_597_039_999_999_4;

/// muon mass
/// Unit: kg
/// Uncertainty: 0.000000042e-28
pub const MUON_MASS_SI: f64 = 1.883_531_627_000_000_11e-28;
/// muon mass (CGS)
pub const MUON_MASS_CGS: f64 = 1.883_531_627_000_000_13e-25;
pub const MUON_MASS: f64 = MUON_MASS_SI;

/// muon mass energy equivalent
/// Unit: J
/// Uncertainty: 0.000000038e-11
pub const MUON_MASS_ENERGY_EQUIVALENT_SI: f64 = 1.692_833_803_999_999_89e-11;
/// muon mass energy equivalent (CGS)
pub const MUON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.000_169_283_380_399_999_991;
pub const MUON_MASS_ENERGY_EQUIVALENT: f64 = MUON_MASS_ENERGY_EQUIVALENT_SI;

/// muon mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.0000023
pub const MUON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 105.658_375_500_000_005;

/// muon mass in u
/// Unit: u
/// Uncertainty: 0.0000000025
pub const MUON_MASS_IN_U: f64 = 0.113_428_925_700_000_002;

/// muon molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.000000025e-4
pub const MUON_MOLAR_MASS_SI: f64 = 0.000_113_428_925_800_000_002;
/// muon molar mass (CGS)
pub const MUON_MOLAR_MASS_CGS: f64 = 0.113_428_925_799_999_997;
pub const MUON_MOLAR_MASS: f64 = MUON_MOLAR_MASS_SI;

/// muon-neutron mass ratio
/// Uncertainty: 0.0000000025
pub const MUON_NEUTRON_MASS_RATIO: f64 = 0.112_454_516_800_000_001;

/// muon-proton mag. mom. ratio
/// Uncertainty: 0.000000071
pub const MUON_PROTON_MAG_MOM_RATIO: f64 = -3.183_345_146_000_000_18;

/// muon-proton mass ratio
/// Uncertainty: 0.0000000025
pub const MUON_PROTON_MASS_RATIO: f64 = 0.112_609_526_200_000_004;

/// muon-tau mass ratio
/// Uncertainty: 0.00040e-2
pub const MUON_TAU_MASS_RATIO: f64 = 0.059_463_500_000_000_002_5;

/// natural unit of action
/// Unit: J s
/// Uncertainty: (exact)
pub const NATURAL_UNIT_OF_ACTION_SI: f64 = 1.054_571_816_999_999_99e-34;
/// natural unit of action (CGS)
pub const NATURAL_UNIT_OF_ACTION_CGS: f64 = 1.054_571_816_999_999_96e-27;
pub const NATURAL_UNIT_OF_ACTION: f64 = NATURAL_UNIT_OF_ACTION_SI;

/// natural unit of action in eV s
/// Unit: eV s
/// Uncertainty: (exact)
pub const NATURAL_UNIT_OF_ACTION_IN_EV_S: f64 = 6.582_119_569_000_000_31e-16;

/// natural unit of energy
/// Unit: J
/// Uncertainty: 0.0000000026e-14
pub const NATURAL_UNIT_OF_ENERGY_SI: f64 = 8.187_105_788_000_000_12e-14;
/// natural unit of energy (CGS)
pub const NATURAL_UNIT_OF_ENERGY_CGS: f64 = 8.187_105_787_999_999_96e-07;
pub const NATURAL_UNIT_OF_ENERGY: f64 = NATURAL_UNIT_OF_ENERGY_SI;

/// natural unit of energy in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000000016
pub const NATURAL_UNIT_OF_ENERGY_IN_MEV: f64 = 0.510_998_950_690_000_009;

/// natural unit of length
/// Unit: m
/// Uncertainty: 0.0000000012e-13
pub const NATURAL_UNIT_OF_LENGTH_SI: f64 = 3.861_592_674_399_999_85e-13;
/// natural unit of length (CGS)
pub const NATURAL_UNIT_OF_LENGTH_CGS: f64 = 3.861_592_674_400_000_1e-11;
pub const NATURAL_UNIT_OF_LENGTH: f64 = NATURAL_UNIT_OF_LENGTH_SI;

/// natural unit of mass
/// Unit: kg
/// Uncertainty: 0.0000000028e-31
pub const NATURAL_UNIT_OF_MASS_SI: f64 = 9.109_383_713_899_999_82e-31;
/// natural unit of mass (CGS)
pub const NATURAL_UNIT_OF_MASS_CGS: f64 = 9.109_383_713_899_999_97e-28;
pub const NATURAL_UNIT_OF_MASS: f64 = NATURAL_UNIT_OF_MASS_SI;

/// natural unit of momentum
/// Unit: kg m s^-1
/// Uncertainty: 0.00000000085e-22
pub const NATURAL_UNIT_OF_MOMENTUM_SI: f64 = 2.730_924_534_459_999_83e-22;
/// natural unit of momentum (CGS)
pub const NATURAL_UNIT_OF_MOMENTUM_CGS: f64 = 2.730_924_534_459_999_88e-17;
pub const NATURAL_UNIT_OF_MOMENTUM: f64 = NATURAL_UNIT_OF_MOMENTUM_SI;

/// natural unit of momentum in `MeV`/c
/// Unit: `MeV`/c
/// Uncertainty: 0.00000000016
pub const NATURAL_UNIT_OF_MOMENTUM_IN_MEV_OVER_C: f64 = 0.510_998_950_690_000_009;

/// natural unit of time
/// Unit: s
/// Uncertainty: 0.00000000040e-21
pub const NATURAL_UNIT_OF_TIME: f64 = 1.288_088_666_440_000_02e-21;

/// natural unit of velocity
/// Unit: m s^-1
/// Uncertainty: (exact)
pub const NATURAL_UNIT_OF_VELOCITY_SI: f64 = 299_792_458.0;
/// natural unit of velocity (CGS)
pub const NATURAL_UNIT_OF_VELOCITY_CGS: f64 = 29_979_245_800.0;
pub const NATURAL_UNIT_OF_VELOCITY: f64 = NATURAL_UNIT_OF_VELOCITY_SI;

/// neutron Compton wavelength
/// Unit: m
/// Uncertainty: 0.00000000067e-15
pub const NEUTRON_COMPTON_WAVELENGTH_SI: f64 = 1.319_590_903_820_000_01e-15;
/// neutron Compton wavelength (CGS)
pub const NEUTRON_COMPTON_WAVELENGTH_CGS: f64 = 1.319_590_903_819_999_99e-13;
pub const NEUTRON_COMPTON_WAVELENGTH: f64 = NEUTRON_COMPTON_WAVELENGTH_SI;

/// neutron-electron mag. mom. ratio
/// Uncertainty: 0.00000024e-3
pub const NEUTRON_ELECTRON_MAG_MOM_RATIO: f64 = 0.001_040_668_839_999_999_91;

/// neutron-electron mass ratio
/// Uncertainty: 0.00000074
pub const NEUTRON_ELECTRON_MASS_RATIO: f64 = 1_838.683_661_999_999_91;

/// neutron g factor
/// Uncertainty: 0.00000090
pub const NEUTRON_G_FACTOR: f64 = -3.826_085_519_999_999_91;

/// neutron gyromag. ratio
/// Unit: s^-1 T^-1
/// Uncertainty: 0.00000043e8
pub const NEUTRON_GYROMAG_RATIO_SI: f64 = 183_247_174.0;
/// neutron gyromag. ratio (CGS)
pub const NEUTRON_GYROMAG_RATIO_CGS: f64 = 18_324.717_400_000_001_4;
pub const NEUTRON_GYROMAG_RATIO: f64 = NEUTRON_GYROMAG_RATIO_SI;

/// neutron gyromag. ratio in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.0000069
pub const NEUTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI: f64 = 29.164_693_499_999_998_5;
/// neutron gyromag. ratio in MHz/T (CGS)
pub const NEUTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_CGS: f64 = 0.002_916_469_35;
pub const NEUTRON_GYROMAG_RATIO_IN_MHZ_OVER_T: f64 = NEUTRON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI;

/// neutron mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.0000023e-27
pub const NEUTRON_MAG_MOM_SI: f64 = -9.662_365_3e-27;
/// neutron mag. mom. (CGS)
pub const NEUTRON_MAG_MOM_CGS: f64 = -9.662_365_299_999_999_55e-24;
pub const NEUTRON_MAG_MOM: f64 = NEUTRON_MAG_MOM_SI;

/// neutron mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000025e-3
pub const NEUTRON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = -0.001_041_875_650_000_000_05;

/// neutron mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.00000045
pub const NEUTRON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = -1.913_042_759_999_999_95;

/// neutron mass
/// Unit: kg
/// Uncertainty: 0.00000000085e-27
pub const NEUTRON_MASS_SI: f64 = 1.674_927_500_560_000_11e-27;
/// neutron mass (CGS)
pub const NEUTRON_MASS_CGS: f64 = 1.674_927_500_560_000_27e-24;
pub const NEUTRON_MASS: f64 = NEUTRON_MASS_SI;

/// neutron mass energy equivalent
/// Unit: J
/// Uncertainty: 0.00000000076e-10
pub const NEUTRON_MASS_ENERGY_EQUIVALENT_SI: f64 = 1.505_349_765_140_000_02e-10;
/// neutron mass energy equivalent (CGS)
pub const NEUTRON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.001_505_349_765_139_999_98;
pub const NEUTRON_MASS_ENERGY_EQUIVALENT: f64 = NEUTRON_MASS_ENERGY_EQUIVALENT_SI;

/// neutron mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000048
pub const NEUTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 939.565_421_939_999_965;

/// neutron mass in u
/// Unit: u
/// Uncertainty: 0.00000000040
pub const NEUTRON_MASS_IN_U: f64 = 1.008_664_916_060_000_08;

/// neutron molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000051e-3
pub const NEUTRON_MOLAR_MASS_SI: f64 = 0.001_008_664_917_12;
/// neutron molar mass (CGS)
pub const NEUTRON_MOLAR_MASS_CGS: f64 = 1.008_664_917_119_999_95;
pub const NEUTRON_MOLAR_MASS: f64 = NEUTRON_MOLAR_MASS_SI;

/// neutron-muon mass ratio
/// Uncertainty: 0.00000020
pub const NEUTRON_MUON_MASS_RATIO: f64 = 8.892_484_079_999_999_12;

/// neutron-proton mag. mom. ratio
/// Uncertainty: 0.00000016
pub const NEUTRON_PROTON_MAG_MOM_RATIO: f64 = -0.684_979_349_999_999_987;

/// neutron-proton mass difference
/// Unit: kg
/// Uncertainty: 0.00000067e-30
pub const NEUTRON_PROTON_MASS_DIFFERENCE_SI: f64 = 2.305_574_610_000_000_14e-30;
/// neutron-proton mass difference (CGS)
pub const NEUTRON_PROTON_MASS_DIFFERENCE_CGS: f64 = 2.305_574_610_000_000_09e-27;
pub const NEUTRON_PROTON_MASS_DIFFERENCE: f64 = NEUTRON_PROTON_MASS_DIFFERENCE_SI;

/// neutron-proton mass difference energy equivalent
/// Unit: J
/// Uncertainty: 0.00000060e-13
pub const NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_SI: f64 = 2.072_147_119_999_999_92e-13;
/// neutron-proton mass difference energy equivalent (CGS)
pub const NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_CGS: f64 = 2.072_147_119_999_999_86e-06;
pub const NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT: f64 = NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_SI;

/// neutron-proton mass difference energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000038
pub const NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_IN_MEV: f64 = 1.293_332_509_999_999_94;

/// neutron-proton mass difference in u
/// Unit: u
/// Uncertainty: 0.00000040e-3
pub const NEUTRON_PROTON_MASS_DIFFERENCE_IN_U: f64 = 0.001_388_449_479_999_999_97;

/// neutron-proton mass ratio
/// Uncertainty: 0.00000000040
pub const NEUTRON_PROTON_MASS_RATIO: f64 = 1.001_378_419_459_999_95;

/// neutron relative atomic mass
/// Uncertainty: 0.00000000040
pub const NEUTRON_RELATIVE_ATOMIC_MASS: f64 = 1.008_664_916_060_000_08;

/// neutron-tau mass ratio
/// Uncertainty: 0.000036
pub const NEUTRON_TAU_MASS_RATIO: f64 = 0.528_778_999_999_999_999;

/// neutron to shielded proton mag. mom. ratio
/// Uncertainty: 0.00000016
pub const NEUTRON_TO_SHIELDED_PROTON_MAG_MOM_RATIO: f64 = -0.684_996_939_999_999_999;

/// Newtonian constant of gravitation
/// Unit: m^3 kg^-1 s^-2
/// Uncertainty: 0.00015e-11
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION_SI: f64 = 6.674_299_999_999_999_38e-11;
/// Newtonian constant of gravitation (CGS)
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION_CGS: f64 = 6.674_299_999_999_999_09e-08;
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: f64 = NEWTONIAN_CONSTANT_OF_GRAVITATION_SI;

/// Newtonian constant of gravitation over h-bar c
/// Unit: (`GeV`/c^2)^-2
/// Uncertainty: 0.00015e-39
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION_OVER_H_BAR_C: f64 = 6.708_830_000_000_000_55e-39;

/// nuclear magneton
/// Unit: J T^-1
/// Uncertainty: 0.0000000016e-27
pub const NUCLEAR_MAGNETON_SI: f64 = 5.050_783_739_300_000_11e-27;
/// nuclear magneton (CGS)
pub const NUCLEAR_MAGNETON_CGS: f64 = 5.050_783_739_300_000_15e-24;
pub const NUCLEAR_MAGNETON: f64 = NUCLEAR_MAGNETON_SI;

/// nuclear magneton in eV/T
/// Unit: eV T^-1
/// Uncertainty: 0.00000000098e-8
pub const NUCLEAR_MAGNETON_IN_EV_OVER_T_SI: f64 = 3.152_451_254_169_999_89e-08;
/// nuclear magneton in eV/T (CGS)
pub const NUCLEAR_MAGNETON_IN_EV_OVER_T_CGS: f64 = 3.152_451_254_169_999_9e-12;
pub const NUCLEAR_MAGNETON_IN_EV_OVER_T: f64 = NUCLEAR_MAGNETON_IN_EV_OVER_T_SI;

/// nuclear magneton in inverse meter per tesla
/// Unit: m^-1 T^-1
/// Uncertainty: 0.00000000079e-2
pub const NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA_SI: f64 = 0.025_426_234_100_899_999_1;
/// nuclear magneton in inverse meter per tesla (CGS)
pub const NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA_CGS: f64 = 2.542_623_410_090_000_46e-08;
pub const NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA: f64 = NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA_SI;

/// nuclear magneton in K/T
/// Unit: K T^-1
/// Uncertainty: 0.0000000011e-4
pub const NUCLEAR_MAGNETON_IN_K_OVER_T_SI: f64 = 0.000_365_826_777_059_999_981;
/// nuclear magneton in K/T (CGS)
pub const NUCLEAR_MAGNETON_IN_K_OVER_T_CGS: f64 = 3.658_267_770_599_999_98e-08;
pub const NUCLEAR_MAGNETON_IN_K_OVER_T: f64 = NUCLEAR_MAGNETON_IN_K_OVER_T_SI;

/// nuclear magneton in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.0000000024
pub const NUCLEAR_MAGNETON_IN_MHZ_OVER_T_SI: f64 = 7.622_593_218_799_999_63;
/// nuclear magneton in MHz/T (CGS)
pub const NUCLEAR_MAGNETON_IN_MHZ_OVER_T_CGS: f64 = 0.000_762_259_321_880_000_024;
pub const NUCLEAR_MAGNETON_IN_MHZ_OVER_T: f64 = NUCLEAR_MAGNETON_IN_MHZ_OVER_T_SI;

/// Planck constant
/// Unit: J Hz^-1
/// Uncertainty: (exact)
pub const PLANCK_CONSTANT_SI: f64 = 6.626_070_149_999_999_83e-34;
/// Planck constant (CGS)
pub const PLANCK_CONSTANT_CGS: f64 = 6.626_070_149_999_999_92e-27;
pub const PLANCK_CONSTANT: f64 = PLANCK_CONSTANT_SI;

/// Planck constant in eV/Hz
/// Unit: eV Hz^-1
/// Uncertainty: (exact)
pub const PLANCK_CONSTANT_IN_EV_OVER_HZ: f64 = 4.135_667_696_000_000_34e-15;

/// Planck length
/// Unit: m
/// Uncertainty: 0.000018e-35
pub const PLANCK_LENGTH_SI: f64 = 1.616_255_000_000_000_07e-35;
/// Planck length (CGS)
pub const PLANCK_LENGTH_CGS: f64 = 1.616_255e-33;
pub const PLANCK_LENGTH: f64 = PLANCK_LENGTH_SI;

/// Planck mass
/// Unit: kg
/// Uncertainty: 0.000024e-8
pub const PLANCK_MASS_SI: f64 = 2.176_434_000_000_000_14e-08;
/// Planck mass (CGS)
pub const PLANCK_MASS_CGS: f64 = 2.176_434_000_000_000_04e-05;
pub const PLANCK_MASS: f64 = PLANCK_MASS_SI;

/// Planck mass energy equivalent in `GeV`
/// Unit: `GeV`
/// Uncertainty: 0.000014e19
pub const PLANCK_MASS_ENERGY_EQUIVALENT_IN_GEV: f64 = 1.220_89e+19;

/// Planck temperature
/// Unit: K
/// Uncertainty: 0.000016e32
pub const PLANCK_TEMPERATURE: f64 = 1.416_784_000_000_000_01e+32;

/// Planck time
/// Unit: s
/// Uncertainty: 0.000060e-44
pub const PLANCK_TIME: f64 = 5.391_247_000_000_000_16e-44;

/// proton charge to mass quotient
/// Unit: C kg^-1
/// Uncertainty: 0.0000000030e7
pub const PROTON_CHARGE_TO_MASS_QUOTIENT_SI: f64 = 95_788_331.430_000_007_2;
/// proton charge to mass quotient (CGS)
pub const PROTON_CHARGE_TO_MASS_QUOTIENT_CGS: f64 = 95_788.331_430_000_005_6;
pub const PROTON_CHARGE_TO_MASS_QUOTIENT: f64 = PROTON_CHARGE_TO_MASS_QUOTIENT_SI;

/// proton Compton wavelength
/// Unit: m
/// Uncertainty: 0.00000000041e-15
pub const PROTON_COMPTON_WAVELENGTH_SI: f64 = 1.321_409_853_600_000_01e-15;
/// proton Compton wavelength (CGS)
pub const PROTON_COMPTON_WAVELENGTH_CGS: f64 = 1.321_409_853_599_999_92e-13;
pub const PROTON_COMPTON_WAVELENGTH: f64 = PROTON_COMPTON_WAVELENGTH_SI;

/// proton-electron mass ratio
/// Uncertainty: 0.000000032
pub const PROTON_ELECTRON_MASS_RATIO: f64 = 1_836.152_673_426_000_09;

/// proton g factor
/// Uncertainty: 0.0000000016
pub const PROTON_G_FACTOR: f64 = 5.585_694_689_300_000_35;

/// proton gyromag. ratio
/// Unit: s^-1 T^-1
/// Uncertainty: 0.0000000011e8
pub const PROTON_GYROMAG_RATIO_SI: f64 = 267_522_187.080_000_013;
/// proton gyromag. ratio (CGS)
pub const PROTON_GYROMAG_RATIO_CGS: f64 = 26_752.218_708_000_004;
pub const PROTON_GYROMAG_RATIO: f64 = PROTON_GYROMAG_RATIO_SI;

/// proton gyromag. ratio in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.000000018
pub const PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI: f64 = 42.577_478_460_999_998_3;
/// proton gyromag. ratio in MHz/T (CGS)
pub const PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_CGS: f64 = 0.004_257_747_846_100_000_05;
pub const PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T: f64 = PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI;

/// proton mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.00000000060e-26
pub const PROTON_MAG_MOM_SI: f64 = 1.410_606_795_450_000_13e-26;
/// proton mag. mom. (CGS)
pub const PROTON_MAG_MOM_CGS: f64 = 1.410_606_795_450_000_03e-23;
pub const PROTON_MAG_MOM: f64 = PROTON_MAG_MOM_SI;

/// proton mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000000045e-3
pub const PROTON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = 0.001_521_032_202_299_999_96;

/// proton mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.00000000082
pub const PROTON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = 2.792_847_344_630_000_17;

/// proton mag. shielding correction
/// Uncertainty: 0.00041e-5
pub const PROTON_MAG_SHIELDING_CORRECTION: f64 = 2.567_149_999_999_999_98e-05;

/// proton mass
/// Unit: kg
/// Uncertainty: 0.00000000052e-27
pub const PROTON_MASS_SI: f64 = 1.672_621_925_949_999_91e-27;
/// proton mass (CGS)
pub const PROTON_MASS_CGS: f64 = 1.672_621_925_949_999_88e-24;
pub const PROTON_MASS: f64 = PROTON_MASS_SI;

/// proton mass energy equivalent
/// Unit: J
/// Uncertainty: 0.00000000047e-10
pub const PROTON_MASS_ENERGY_EQUIVALENT_SI: f64 = 1.503_277_618_02e-10;
/// proton mass energy equivalent (CGS)
pub const PROTON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.001_503_277_618_020_000_11;
pub const PROTON_MASS_ENERGY_EQUIVALENT: f64 = PROTON_MASS_ENERGY_EQUIVALENT_SI;

/// proton mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000029
pub const PROTON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 938.272_089_430_000_051;

/// proton mass in u
/// Unit: u
/// Uncertainty: 0.0000000000083
pub const PROTON_MASS_IN_U: f64 = 1.007_276_466_578_900_02;

/// proton molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000031e-3
pub const PROTON_MOLAR_MASS_SI: f64 = 0.001_007_276_467_639_999_95;
/// proton molar mass (CGS)
pub const PROTON_MOLAR_MASS_CGS: f64 = 1.007_276_467_639_999_9;
pub const PROTON_MOLAR_MASS: f64 = PROTON_MOLAR_MASS_SI;

/// proton-muon mass ratio
/// Uncertainty: 0.00000020
pub const PROTON_MUON_MASS_RATIO: f64 = 8.880_243_379_999_999_58;

/// proton-neutron mag. mom. ratio
/// Uncertainty: 0.00000034
pub const PROTON_NEUTRON_MAG_MOM_RATIO: f64 = -1.459_898_020_000_000_02;

/// proton-neutron mass ratio
/// Uncertainty: 0.00000000040
pub const PROTON_NEUTRON_MASS_RATIO: f64 = 0.998_623_477_969_999_951;

/// proton relative atomic mass
/// Uncertainty: 0.0000000000083
pub const PROTON_RELATIVE_ATOMIC_MASS: f64 = 1.007_276_466_578_900_02;

/// proton rms charge radius
/// Unit: m
/// Uncertainty: 0.0064e-16
pub const PROTON_RMS_CHARGE_RADIUS_SI: f64 = 8.407_500_000_000_000_04e-16;
/// proton rms charge radius (CGS)
pub const PROTON_RMS_CHARGE_RADIUS_CGS: f64 = 8.407_499_999_999_999_77e-14;
pub const PROTON_RMS_CHARGE_RADIUS: f64 = PROTON_RMS_CHARGE_RADIUS_SI;

/// proton-tau mass ratio
/// Uncertainty: 0.000036
pub const PROTON_TAU_MASS_RATIO: f64 = 0.528_051_000_000_000_048;

/// quantum of circulation
/// Unit: m^2 s^-1
/// Uncertainty: 0.0000000011e-4
pub const QUANTUM_OF_CIRCULATION_SI: f64 = 0.000_363_694_754_670_000_012;
/// quantum of circulation (CGS)
pub const QUANTUM_OF_CIRCULATION_CGS: f64 = 3.636_947_546_700_000_09;
pub const QUANTUM_OF_CIRCULATION: f64 = QUANTUM_OF_CIRCULATION_SI;

/// quantum of circulation times 2
/// Unit: m^2 s^-1
/// Uncertainty: 0.0000000023e-4
pub const QUANTUM_OF_CIRCULATION_TIMES_2_SI: f64 = 0.000_727_389_509_340_000_025;
/// quantum of circulation times 2 (CGS)
pub const QUANTUM_OF_CIRCULATION_TIMES_2_CGS: f64 = 7.273_895_093_400_000_18;
pub const QUANTUM_OF_CIRCULATION_TIMES_2: f64 = QUANTUM_OF_CIRCULATION_TIMES_2_SI;

/// reduced Compton wavelength
/// Unit: m
/// Uncertainty: 0.0000000012e-13
pub const REDUCED_COMPTON_WAVELENGTH_SI: f64 = 3.861_592_674_399_999_85e-13;
/// reduced Compton wavelength (CGS)
pub const REDUCED_COMPTON_WAVELENGTH_CGS: f64 = 3.861_592_674_400_000_1e-11;
pub const REDUCED_COMPTON_WAVELENGTH: f64 = REDUCED_COMPTON_WAVELENGTH_SI;

/// reduced muon Compton wavelength
/// Unit: m
/// Uncertainty: 0.000000042e-15
pub const REDUCED_MUON_COMPTON_WAVELENGTH_SI: f64 = 1.867_594_305_999_999_98e-15;
/// reduced muon Compton wavelength (CGS)
pub const REDUCED_MUON_COMPTON_WAVELENGTH_CGS: f64 = 1.867_594_305_999_999_99e-13;
pub const REDUCED_MUON_COMPTON_WAVELENGTH: f64 = REDUCED_MUON_COMPTON_WAVELENGTH_SI;

/// reduced neutron Compton wavelength
/// Unit: m
/// Uncertainty: 0.0000000011e-16
pub const REDUCED_NEUTRON_COMPTON_WAVELENGTH_SI: f64 = 2.100_194_151_999_999_95e-16;
/// reduced neutron Compton wavelength (CGS)
pub const REDUCED_NEUTRON_COMPTON_WAVELENGTH_CGS: f64 = 2.100_194_151_999_999_96e-14;
pub const REDUCED_NEUTRON_COMPTON_WAVELENGTH: f64 = REDUCED_NEUTRON_COMPTON_WAVELENGTH_SI;

/// reduced Planck constant
/// Unit: J s
/// Uncertainty: (exact)
pub const REDUCED_PLANCK_CONSTANT_SI: f64 = 1.054_571_816_999_999_99e-34;
/// reduced Planck constant (CGS)
pub const REDUCED_PLANCK_CONSTANT_CGS: f64 = 1.054_571_816_999_999_96e-27;
pub const REDUCED_PLANCK_CONSTANT: f64 = REDUCED_PLANCK_CONSTANT_SI;

/// reduced Planck constant in eV s
/// Unit: eV s
/// Uncertainty: (exact)
pub const REDUCED_PLANCK_CONSTANT_IN_EV_S: f64 = 6.582_119_569_000_000_31e-16;

/// reduced Planck constant times c in `MeV` fm
/// Unit: `MeV` fm
/// Uncertainty: (exact)
pub const REDUCED_PLANCK_CONSTANT_TIMES_C_IN_MEV_FM: f64 = 197.326_980_399_999_997;

/// reduced proton Compton wavelength
/// Unit: m
/// Uncertainty: 0.00000000066e-16
pub const REDUCED_PROTON_COMPTON_WAVELENGTH_SI: f64 = 2.103_089_100_509_999_89e-16;
/// reduced proton Compton wavelength (CGS)
pub const REDUCED_PROTON_COMPTON_WAVELENGTH_CGS: f64 = 2.103_089_100_509_999_86e-14;
pub const REDUCED_PROTON_COMPTON_WAVELENGTH: f64 = REDUCED_PROTON_COMPTON_WAVELENGTH_SI;

/// reduced tau Compton wavelength
/// Unit: m
/// Uncertainty: 0.000075e-16
pub const REDUCED_TAU_COMPTON_WAVELENGTH_SI: f64 = 1.110_538_000_000_000_06e-16;
/// reduced tau Compton wavelength (CGS)
pub const REDUCED_TAU_COMPTON_WAVELENGTH_CGS: f64 = 1.110_538_000_000_000_14e-14;
pub const REDUCED_TAU_COMPTON_WAVELENGTH: f64 = REDUCED_TAU_COMPTON_WAVELENGTH_SI;

/// Rydberg constant
/// Unit: m^-1
/// Uncertainty: 0.000012
pub const RYDBERG_CONSTANT_SI: f64 = 10_973_731.568_157_000_5;
/// Rydberg constant (CGS)
pub const RYDBERG_CONSTANT_CGS: f64 = 109_737.315_681_570_006;
pub const RYDBERG_CONSTANT: f64 = RYDBERG_CONSTANT_SI;

/// Rydberg constant times c in Hz
/// Unit: Hz
/// Uncertainty: 0.0000000000036e15
pub const RYDBERG_CONSTANT_TIMES_C_IN_HZ: f64 = 3_289_841_960_250_000.0;

/// Rydberg constant times hc in eV
/// Unit: eV
/// Uncertainty: 0.000000000015
pub const RYDBERG_CONSTANT_TIMES_HC_IN_EV: f64 = 13.605_693_122_990_000_9;

/// Rydberg constant times hc in J
/// Unit: J
/// Uncertainty: 0.0000000000024e-18
pub const RYDBERG_CONSTANT_TIMES_HC_IN_J_SI: f64 = 2.179_872_361_102_999_95e-18;
/// Rydberg constant times hc in J (CGS)
pub const RYDBERG_CONSTANT_TIMES_HC_IN_J_CGS: f64 = 2.179_872_361_102_999_94e-11;
pub const RYDBERG_CONSTANT_TIMES_HC_IN_J: f64 = RYDBERG_CONSTANT_TIMES_HC_IN_J_SI;

/// Sackur-Tetrode constant (1 K, 100 kPa)
/// Uncertainty: 0.00000000047
pub const SACKUR_TETRODE_CONSTANT_1_K_100_KPA: f64 = -1.151_707_534_959_999_89;

/// Sackur-Tetrode constant (1 K, 101.325 kPa)
/// Uncertainty: 0.00000000047
pub const SACKUR_TETRODE_CONSTANT_1_K_101_325_KPA: f64 = -1.164_870_521_489_999_92;

/// second radiation constant
/// Unit: m K
/// Uncertainty: (exact)
pub const SECOND_RADIATION_CONSTANT_SI: f64 = 0.014_387_768_77;
/// second radiation constant (CGS)
pub const SECOND_RADIATION_CONSTANT_CGS: f64 = 1.438_776_877_000_000_01;
pub const SECOND_RADIATION_CONSTANT: f64 = SECOND_RADIATION_CONSTANT_SI;

/// shielded helion gyromag. ratio
/// Unit: s^-1 T^-1
/// Uncertainty: 0.0000000018e8
pub const SHIELDED_HELION_GYROMAG_RATIO_SI: f64 = 203_789_460.780_000_001;
/// shielded helion gyromag. ratio (CGS)
pub const SHIELDED_HELION_GYROMAG_RATIO_CGS: f64 = 20_378.946_078_000_000_9;
pub const SHIELDED_HELION_GYROMAG_RATIO: f64 = SHIELDED_HELION_GYROMAG_RATIO_SI;

/// shielded helion gyromag. ratio in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.000000028
pub const SHIELDED_HELION_GYROMAG_RATIO_IN_MHZ_OVER_T_SI: f64 = 32.434_100_033;
/// shielded helion gyromag. ratio in MHz/T (CGS)
pub const SHIELDED_HELION_GYROMAG_RATIO_IN_MHZ_OVER_T_CGS: f64 = 0.003_243_410_003_299_999_99;
pub const SHIELDED_HELION_GYROMAG_RATIO_IN_MHZ_OVER_T: f64 = SHIELDED_HELION_GYROMAG_RATIO_IN_MHZ_OVER_T_SI;

/// shielded helion mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.00000000093e-26
pub const SHIELDED_HELION_MAG_MOM_SI: f64 = -1.074_553_110_350_000_05e-26;
/// shielded helion mag. mom. (CGS)
pub const SHIELDED_HELION_MAG_MOM_CGS: f64 = -1.074_553_110_350_000_06e-23;
pub const SHIELDED_HELION_MAG_MOM: f64 = SHIELDED_HELION_MAG_MOM_SI;

/// shielded helion mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.00000000094e-3
pub const SHIELDED_HELION_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = -0.001_158_671_494_569_999_98;

/// shielded helion mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.0000000017
pub const SHIELDED_HELION_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = -2.127_497_762_399_999_99;

/// shielded helion to proton mag. mom. ratio
/// Uncertainty: 0.00000000066
pub const SHIELDED_HELION_TO_PROTON_MAG_MOM_RATIO: f64 = -0.761_766_577_209_999_962;

/// shielded helion to shielded proton mag. mom. ratio
/// Uncertainty: 0.0000000031
pub const SHIELDED_HELION_TO_SHIELDED_PROTON_MAG_MOM_RATIO: f64 = -0.761_786_133_400_000_01;

/// shielded proton gyromag. ratio
/// Unit: s^-1 T^-1
/// Uncertainty: 0.000000011e8
pub const SHIELDED_PROTON_GYROMAG_RATIO_SI: f64 = 267_515_319.400_000_006;
/// shielded proton gyromag. ratio (CGS)
pub const SHIELDED_PROTON_GYROMAG_RATIO_CGS: f64 = 26_751.531_940_000_000_8;
pub const SHIELDED_PROTON_GYROMAG_RATIO: f64 = SHIELDED_PROTON_GYROMAG_RATIO_SI;

/// shielded proton gyromag. ratio in MHz/T
/// Unit: MHz T^-1
/// Uncertainty: 0.00000017
pub const SHIELDED_PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI: f64 = 42.576_385_430_000_002;
/// shielded proton gyromag. ratio in MHz/T (CGS)
pub const SHIELDED_PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_CGS: f64 = 0.004_257_638_543_000_000_22;
pub const SHIELDED_PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T: f64 = SHIELDED_PROTON_GYROMAG_RATIO_IN_MHZ_OVER_T_SI;

/// shielded proton mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.0000000058e-26
pub const SHIELDED_PROTON_MAG_MOM_SI: f64 = 1.410_570_583_000_000_01e-26;
/// shielded proton mag. mom. (CGS)
pub const SHIELDED_PROTON_MAG_MOM_CGS: f64 = 1.410_570_583_000_000_05e-23;
pub const SHIELDED_PROTON_MAG_MOM: f64 = SHIELDED_PROTON_MAG_MOM_SI;

/// shielded proton mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.0000000062e-3
pub const SHIELDED_PROTON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = 0.001_520_993_155_099_999_9;

/// shielded proton mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.000000011
pub const SHIELDED_PROTON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = 2.792_775_648_000_000_11;

/// shielding difference of d and p in HD
/// Uncertainty: 0.00010e-8
pub const SHIELDING_DIFFERENCE_OF_D_AND_P_IN_HD: f64 = 1.987_699_999_999_999_99e-08;

/// shielding difference of t and p in HT
/// Uncertainty: 0.00020e-8
pub const SHIELDING_DIFFERENCE_OF_T_AND_P_IN_HT: f64 = 2.394_500_000_000_000_03e-08;

/// speed of light in vacuum
/// Unit: m s^-1
/// Uncertainty: (exact)
pub const SPEED_OF_LIGHT_IN_VACUUM_SI: f64 = 299_792_458.0;
/// speed of light in vacuum (CGS)
pub const SPEED_OF_LIGHT_IN_VACUUM_CGS: f64 = 29_979_245_800.0;
pub const SPEED_OF_LIGHT_IN_VACUUM: f64 = SPEED_OF_LIGHT_IN_VACUUM_SI;

/// standard acceleration of gravity
/// Unit: m s^-2
/// Uncertainty: (exact)
pub const STANDARD_ACCELERATION_OF_GRAVITY_SI: f64 = 9.806_649_999_999_999_42;
/// standard acceleration of gravity (CGS)
pub const STANDARD_ACCELERATION_OF_GRAVITY_CGS: f64 = 980.664_999_999_999_964;
pub const STANDARD_ACCELERATION_OF_GRAVITY: f64 = STANDARD_ACCELERATION_OF_GRAVITY_SI;

/// standard atmosphere
/// Unit: Pa
/// Uncertainty: (exact)
pub const STANDARD_ATMOSPHERE_SI: f64 = 101_325.0;
/// standard atmosphere (CGS)
pub const STANDARD_ATMOSPHERE_CGS: f64 = 1_013_250.0;
pub const STANDARD_ATMOSPHERE: f64 = STANDARD_ATMOSPHERE_SI;

/// standard-state pressure
/// Unit: Pa
/// Uncertainty: (exact)
pub const STANDARD_STATE_PRESSURE_SI: f64 = 100_000.0;
/// standard-state pressure (CGS)
pub const STANDARD_STATE_PRESSURE_CGS: f64 = 1_000_000.0;
pub const STANDARD_STATE_PRESSURE: f64 = STANDARD_STATE_PRESSURE_SI;

/// Stefan-Boltzmann constant
/// Unit: W m^-2 K^-4
/// Uncertainty: (exact)
pub const STEFAN_BOLTZMANN_CONSTANT_SI: f64 = 5.670_374_418_999_999_91e-08;
/// Stefan-Boltzmann constant (CGS)
pub const STEFAN_BOLTZMANN_CONSTANT_CGS: f64 = 5.670_374_418_999_999_76e-05;
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = STEFAN_BOLTZMANN_CONSTANT_SI;

/// tau Compton wavelength
/// Unit: m
/// Uncertainty: 0.00047e-16
pub const TAU_COMPTON_WAVELENGTH_SI: f64 = 6.977_710_000_000_000_14e-16;
/// tau Compton wavelength (CGS)
pub const TAU_COMPTON_WAVELENGTH_CGS: f64 = 6.977_709_999_999_999_86e-14;
pub const TAU_COMPTON_WAVELENGTH: f64 = TAU_COMPTON_WAVELENGTH_SI;

/// tau-electron mass ratio
/// Uncertainty: 0.23
pub const TAU_ELECTRON_MASS_RATIO: f64 = 3_477.230_000_000_000_02;

/// tau energy equivalent
/// Unit: `MeV`
/// Uncertainty: 0.12
pub const TAU_ENERGY_EQUIVALENT: f64 = 1_776.859_999_999_999_9;

/// tau mass
/// Unit: kg
/// Uncertainty: 0.00021e-27
pub const TAU_MASS_SI: f64 = 3.167_539_999_999_999_91e-27;
/// tau mass (CGS)
pub const TAU_MASS_CGS: f64 = 3.167_539_999_999_999_87e-24;
pub const TAU_MASS: f64 = TAU_MASS_SI;

/// tau mass energy equivalent
/// Unit: J
/// Uncertainty: 0.00019e-10
pub const TAU_MASS_ENERGY_EQUIVALENT_SI: f64 = 2.846_839_999_999_999_76e-10;
/// tau mass energy equivalent (CGS)
pub const TAU_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.002_846_839_999_999_999_79;
pub const TAU_MASS_ENERGY_EQUIVALENT: f64 = TAU_MASS_ENERGY_EQUIVALENT_SI;

/// tau mass in u
/// Unit: u
/// Uncertainty: 0.00013
pub const TAU_MASS_IN_U: f64 = 1.907_540_000_000_000_01;

/// tau molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00013e-3
pub const TAU_MOLAR_MASS_SI: f64 = 0.001_907_540_000_000_000_1;
/// tau molar mass (CGS)
pub const TAU_MOLAR_MASS_CGS: f64 = 1.907_540_000_000_000_01;
pub const TAU_MOLAR_MASS: f64 = TAU_MOLAR_MASS_SI;

/// tau-muon mass ratio
/// Uncertainty: 0.0011
pub const TAU_MUON_MASS_RATIO: f64 = 16.817_000_000_000_000_2;

/// tau-neutron mass ratio
/// Uncertainty: 0.00013
pub const TAU_NEUTRON_MASS_RATIO: f64 = 1.891_150_000_000_000_11;

/// tau-proton mass ratio
/// Uncertainty: 0.00013
pub const TAU_PROTON_MASS_RATIO: f64 = 1.893_760_000_000_000_11;

/// Thomson cross section
/// Unit: m^2
/// Uncertainty: 0.0000000062e-29
pub const THOMSON_CROSS_SECTION_SI: f64 = 6.652_458_705_100_000_15e-29;
/// Thomson cross section (CGS)
pub const THOMSON_CROSS_SECTION_CGS: f64 = 6.652_458_705_100_000_03e-25;
pub const THOMSON_CROSS_SECTION: f64 = THOMSON_CROSS_SECTION_SI;

/// triton-electron mass ratio
/// Uncertainty: 0.00000021
pub const TRITON_ELECTRON_MASS_RATIO: f64 = 5_496.921_535_509_999_56;

/// triton g factor
/// Uncertainty: 0.000000012
pub const TRITON_G_FACTOR: f64 = 5.957_924_929_999_999_9;

/// triton mag. mom.
/// Unit: J T^-1
/// Uncertainty: 0.0000000030e-26
pub const TRITON_MAG_MOM_SI: f64 = 1.504_609_517_799_999_9e-26;
/// triton mag. mom. (CGS)
pub const TRITON_MAG_MOM_CGS: f64 = 1.504_609_517_800_000_02e-23;
pub const TRITON_MAG_MOM: f64 = TRITON_MAG_MOM_SI;

/// triton mag. mom. to Bohr magneton ratio
/// Uncertainty: 0.0000000032e-3
pub const TRITON_MAG_MOM_TO_BOHR_MAGNETON_RATIO: f64 = 0.001_622_393_664_799_999_96;

/// triton mag. mom. to nuclear magneton ratio
/// Uncertainty: 0.0000000059
pub const TRITON_MAG_MOM_TO_NUCLEAR_MAGNETON_RATIO: f64 = 2.978_962_464_999_999_95;

/// triton mass
/// Unit: kg
/// Uncertainty: 0.0000000016e-27
pub const TRITON_MASS_SI: f64 = 5.007_356_751_199_999_82e-27;
/// triton mass (CGS)
pub const TRITON_MASS_CGS: f64 = 5.007_356_751_199_999_94e-24;
pub const TRITON_MASS: f64 = TRITON_MASS_SI;

/// triton mass energy equivalent
/// Unit: J
/// Uncertainty: 0.0000000014e-10
pub const TRITON_MASS_ENERGY_EQUIVALENT_SI: f64 = 4.500_387_811_899_999_92e-10;
/// triton mass energy equivalent (CGS)
pub const TRITON_MASS_ENERGY_EQUIVALENT_CGS: f64 = 0.004_500_387_811_900_000_15;
pub const TRITON_MASS_ENERGY_EQUIVALENT: f64 = TRITON_MASS_ENERGY_EQUIVALENT_SI;

/// triton mass energy equivalent in `MeV`
/// Unit: `MeV`
/// Uncertainty: 0.00000088
pub const TRITON_MASS_ENERGY_EQUIVALENT_IN_MEV: f64 = 2_808.921_136_680_000_02;

/// triton mass in u
/// Unit: u
/// Uncertainty: 0.00000000010
pub const TRITON_MASS_IN_U: f64 = 3.015_500_715_970_000_02;

/// triton molar mass
/// Unit: kg mol^-1
/// Uncertainty: 0.00000000094e-3
pub const TRITON_MOLAR_MASS_SI: f64 = 0.003_015_500_719_129_999_86;
/// triton molar mass (CGS)
pub const TRITON_MOLAR_MASS_CGS: f64 = 3.015_500_719_129_999_84;
pub const TRITON_MOLAR_MASS: f64 = TRITON_MOLAR_MASS_SI;

/// triton-proton mass ratio
/// Uncertainty: 0.00000000010
pub const TRITON_PROTON_MASS_RATIO: f64 = 2.993_717_034_029_999_89;

/// triton relative atomic mass
/// Uncertainty: 0.00000000010
pub const TRITON_RELATIVE_ATOMIC_MASS: f64 = 3.015_500_715_970_000_02;

/// triton to proton mag. mom. ratio
/// Uncertainty: 0.0000000021
pub const TRITON_TO_PROTON_MAG_MOM_RATIO: f64 = 1.066_639_918_899_999_98;

/// unified atomic mass unit
/// Unit: kg
/// Uncertainty: 0.00000000052e-27
pub const UNIFIED_ATOMIC_MASS_UNIT_SI: f64 = 1.660_539_068_919_999_93e-27;
/// unified atomic mass unit (CGS)
pub const UNIFIED_ATOMIC_MASS_UNIT_CGS: f64 = 1.660_539_068_920_000_11e-24;
pub const UNIFIED_ATOMIC_MASS_UNIT: f64 = UNIFIED_ATOMIC_MASS_UNIT_SI;

/// vacuum electric permittivity
/// Unit: F m^-1
/// Uncertainty: 0.0000000014e-12
pub const VACUUM_ELECTRIC_PERMITTIVITY_SI: f64 = 8.854_187_818_800_000_41e-12;
/// vacuum electric permittivity (CGS)
pub const VACUUM_ELECTRIC_PERMITTIVITY_CGS: f64 = 8.854_187_818_800_000_26e-14;
pub const VACUUM_ELECTRIC_PERMITTIVITY: f64 = VACUUM_ELECTRIC_PERMITTIVITY_SI;

/// vacuum mag. permeability
/// Unit: N A^-2
/// Uncertainty: 0.00000000020e-6
pub const VACUUM_MAG_PERMEABILITY_SI: f64 = 1.256_637_061_270_000_05e-06;
/// vacuum mag. permeability (CGS)
pub const VACUUM_MAG_PERMEABILITY_CGS: f64 = 0.125_663_706_127_000_008;
pub const VACUUM_MAG_PERMEABILITY: f64 = VACUUM_MAG_PERMEABILITY_SI;

/// von Klitzing constant
/// Unit: ohm
/// Uncertainty: (exact)
pub const VON_KLITZING_CONSTANT: f64 = 25_812.807_450_000_000_2;

/// weak mixing angle
/// Uncertainty: 0.00023
pub const WEAK_MIXING_ANGLE: f64 = 0.223_049_999_999_999_998;

/// Wien frequency displacement law constant
/// Unit: Hz K^-1
/// Uncertainty: (exact)
pub const WIEN_FREQUENCY_DISPLACEMENT_LAW_CONSTANT: f64 = 58_789_257_570.0;

/// Wien wavelength displacement law constant
/// Unit: m K
/// Uncertainty: (exact)
pub const WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT_SI: f64 = 0.002_897_771_955_000_000_03;
/// Wien wavelength displacement law constant (CGS)
pub const WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT_CGS: f64 = 0.289_777_195_500_000_029;
pub const WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT: f64 = WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT_SI;

/// W to Z mass ratio
/// Uncertainty: 0.00013
pub const W_TO_Z_MASS_RATIO: f64 = 0.881_449_999_999_999_956;

/// Detailed constant information including value, uncertainty, and unit.
#[allow(clippy::unreadable_literal)]
pub const CONSTANTS: &[(&str, Constant)] = &[
    ("alpha particle-electron mass ratio", Constant { value: 7_294.299_541_710_000_09, uncertainty: 1.699_999_999_999_999_87e-07, unit: "" }),
    ("alpha particle mass", Constant { value: 6.644_657_344_999_999_81e-27, uncertainty: 2.099_999_999_999_999_91e-36, unit: "kg" }),
    ("alpha particle mass energy equivalent", Constant { value: 5.971_920_199_700_000_05e-10, uncertainty: 1.9e-19, unit: "J" }),
    ("alpha particle mass energy equivalent in MeV", Constant { value: 3_727.379_411_800_000_07, uncertainty: 1.199_999_999_999_999_95e-06, unit: "MeV" }),
    ("alpha particle mass in u", Constant { value: 4.001_506_179_128_999_64, uncertainty: 6.200_000_000_000_000_56e-11, unit: "u" }),
    ("alpha particle molar mass", Constant { value: 0.004_001_506_183_300_000_34, uncertainty: 1.199_999_999_999_999_94e-12, unit: "kg mol^-1" }),
    ("alpha particle-proton mass ratio", Constant { value: 3.972_599_690_252_000_22, uncertainty: 7.000_000_000_000_000_38e-11, unit: "" }),
    ("alpha particle relative atomic mass", Constant { value: 4.001_506_179_128_999_64, uncertainty: 6.200_000_000_000_000_56e-11, unit: "" }),
    ("alpha particle rms charge radius", Constant { value: 1.678_499_999_999_999_96e-15, uncertainty: 2.1e-18, unit: "m" }),
    ("Angstrom star", Constant { value: 1.000_014_949_999_999_94e-10, uncertainty: 8.999_999_999_999_999_57e-17, unit: "m" }),
    ("atomic mass constant", Constant { value: 1.660_539_068_919_999_93e-27, uncertainty: 5.199_999_999_999_999_59e-37, unit: "kg" }),
    ("atomic mass constant energy equivalent", Constant { value: 1.492_418_087_680_000_05e-10, uncertainty: 4.599_999_999_999_999_84e-20, unit: "J" }),
    ("atomic mass constant energy equivalent in MeV", Constant { value: 931.494_103_719_999_998, uncertainty: 2.899_999_999_999_999_76e-07, unit: "MeV" }),
    ("atomic mass unit-electron volt relationship", Constant { value: 931_494_103.720_000_029, uncertainty: 0.289_999_999_999_999_98, unit: "eV" }),
    ("atomic mass unit-hartree relationship", Constant { value: 34_231_776.921_999_998_4, uncertainty: 0.010_999_999_999_999_999_4, unit: "E_h" }),
    ("atomic mass unit-hertz relationship", Constant { value: 2.252_342_721_849_999_96e+23, uncertainty: 70_000_000_000_000.0, unit: "Hz" }),
    ("atomic mass unit-inverse meter relationship", Constant { value: 751_300_662_090_000.0, uncertainty: 230_000.0, unit: "m^-1" }),
    ("atomic mass unit-joule relationship", Constant { value: 1.492_418_087_680_000_05e-10, uncertainty: 4.599_999_999_999_999_84e-20, unit: "J" }),
    ("atomic mass unit-kelvin relationship", Constant { value: 10_809_540_206_700.0, uncertainty: 3_400.0, unit: "K" }),
    ("atomic mass unit-kilogram relationship", Constant { value: 1.660_539_068_919_999_93e-27, uncertainty: 5.199_999_999_999_999_59e-37, unit: "kg" }),
    ("atomic unit of 1st hyperpolarizability", Constant { value: 3.206_361_299_599_999_94e-53, uncertainty: 1.500_000_000_000_000_06e-62, unit: "C^3 m^3 J^-2" }),
    ("atomic unit of 2nd hyperpolarizability", Constant { value: 6.235_379_973_500_000_3e-65, uncertainty: 3.900_000_000_000_000_13e-74, unit: "C^4 m^4 J^-3" }),
    ("atomic unit of action", Constant { value: 1.054_571_816_999_999_99e-34, uncertainty: 0.0, unit: "J s" }),
    ("atomic unit of charge", Constant { value: 1.602_176_633_999_999_89e-19, uncertainty: 0.0, unit: "C" }),
    ("atomic unit of charge density", Constant { value: 1_081_202_386_770.0, uncertainty: 510.0, unit: "C m^-3" }),
    ("atomic unit of current", Constant { value: 0.006_623_618_237_508_200_13, uncertainty: 7.200_000_000_000_000_24e-15, unit: "A" }),
    ("atomic unit of electric dipole mom.", Constant { value: 8.478_353_619_800_000_69e-30, uncertainty: 1.299_999_999_999_999_99e-39, unit: "C m" }),
    ("atomic unit of electric field", Constant { value: 514_220_675_112.0, uncertainty: 80.0, unit: "V m^-1" }),
    ("atomic unit of electric field gradient", Constant { value: 9.717_362_442_400_000_11e+21, uncertainty: 3_000_000_000_000.0, unit: "V m^-2" }),
    ("atomic unit of electric polarizability", Constant { value: 1.648_777_272_12e-41, uncertainty: 5.099_999_999_999_999_74e-51, unit: "C^2 m^2 J^-1" }),
    ("atomic unit of electric potential", Constant { value: 27.211_386_245_981_000_1, uncertainty: 2.999_999_999_999_999_98e-11, unit: "V" }),
    ("atomic unit of electric quadrupole mom.", Constant { value: 4.486_551_518_500_000_02e-40, uncertainty: 1.399_999_999_999_999_99e-49, unit: "C m^2" }),
    ("atomic unit of energy", Constant { value: 4.359_744_722_205_999_89e-18, uncertainty: 4.799_999_999_999_999_7e-30, unit: "J" }),
    ("atomic unit of force", Constant { value: 8.238_723_503_800_000_3e-08, uncertainty: 1.300_000_000_000_000_02e-17, unit: "N" }),
    ("atomic unit of length", Constant { value: 5.291_772_105_439_999_77e-11, uncertainty: 8.199_999_999_999_999_88e-21, unit: "m" }),
    ("atomic unit of mag. dipole mom.", Constant { value: 1.854_802_013_149_999_92e-23, uncertainty: 5.800_000_000_000_000_05e-33, unit: "J T^-1" }),
    ("atomic unit of mag. flux density", Constant { value: 235_051.757_076_999_987, uncertainty: 7.299_999_999_999_999_9e-05, unit: "T" }),
    ("atomic unit of magnetizability", Constant { value: 7.891_036_579_399_999_45e-29, uncertainty: 4.900_000_000_000_000_39e-38, unit: "J T^-2" }),
    ("atomic unit of mass", Constant { value: 9.109_383_713_899_999_82e-31, uncertainty: 2.799_999_999_999_999_97e-40, unit: "kg" }),
    ("atomic unit of momentum", Constant { value: 1.992_851_915_449_999_82e-24, uncertainty: 3.099_999_999_999_999_8e-34, unit: "kg m s^-1" }),
    ("atomic unit of permittivity", Constant { value: 1.112_650_056_200_000_06e-10, uncertainty: 1.699_999_999_999_999_86e-20, unit: "F m^-1" }),
    ("atomic unit of time", Constant { value: 2.418_884_326_586_399_85e-17, uncertainty: 2.600_000_000_000_000_22e-29, unit: "s" }),
    ("atomic unit of velocity", Constant { value: 2_187_691.262_159_999_93, uncertainty: 0.000_340_000_000_000_000_024, unit: "m s^-1" }),
    ("Avogadro constant", Constant { value: 6.022_140_759_999_999_87e+23, uncertainty: 0.0, unit: "mol^-1" }),
    ("Bohr magneton", Constant { value: 9.274_010_065_699_999_92e-24, uncertainty: 2.900_000_000_000_000_03e-33, unit: "J T^-1" }),
    ("Bohr magneton in eV/T", Constant { value: 5.788_381_798_199_999_99e-05, uncertainty: 1.799_999_999_999_999_9e-14, unit: "eV T^-1" }),
    ("Bohr magneton in Hz/T", Constant { value: 13_996_244_917.100_000_4, uncertainty: 4.400_000_000_000_000_36, unit: "Hz T^-1" }),
    ("Bohr magneton in inverse meter per tesla", Constant { value: 46.686_447_719_000_000_2, uncertainty: 1.499_999_999_999_999_87e-08, unit: "m^-1 T^-1" }),
    ("Bohr magneton in K/T", Constant { value: 0.671_713_814_720_000_024, uncertainty: 2.099_999_999_999_999_99e-10, unit: "K T^-1" }),
    ("Bohr radius", Constant { value: 5.291_772_105_439_999_77e-11, uncertainty: 8.199_999_999_999_999_88e-21, unit: "m" }),
    ("Boltzmann constant", Constant { value: 1.380_649_000_000_000_09e-23, uncertainty: 0.0, unit: "J K^-1" }),
    ("Boltzmann constant in eV/K", Constant { value: 8.617_333_262_000_000_06e-05, uncertainty: 0.0, unit: "eV K^-1" }),
    ("Boltzmann constant in Hz/K", Constant { value: 20_836_619_120.0, uncertainty: 0.0, unit: "Hz K^-1" }),
    ("Boltzmann constant in inverse meter per kelvin", Constant { value: 69.503_480_039_999_999_5, uncertainty: 0.0, unit: "m^-1 K^-1" }),
    ("characteristic impedance of vacuum", Constant { value: 376.730_313_411_999_987, uncertainty: 5.899_999_999_999_999_89e-08, unit: "ohm" }),
    ("classical electron radius", Constant { value: 2.817_940_320_499_999_9e-15, uncertainty: 1.299_999_999_999_999_99e-24, unit: "m" }),
    ("Compton wavelength", Constant { value: 2.426_310_235_380_000_03e-12, uncertainty: 7.600_000_000_000_000_46e-22, unit: "m" }),
    ("conductance quantum", Constant { value: 7.748_091_729_000_000_37e-05, uncertainty: 0.0, unit: "S" }),
    ("conventional value of ampere-90", Constant { value: 1.000_000_088_870_000_03, uncertainty: 0.0, unit: "A" }),
    ("conventional value of coulomb-90", Constant { value: 1.000_000_088_870_000_03, uncertainty: 0.0, unit: "C" }),
    ("conventional value of farad-90", Constant { value: 0.999_999_982_199_999_971, uncertainty: 0.0, unit: "F" }),
    ("conventional value of henry-90", Constant { value: 1.000_000_017_789_999_92, uncertainty: 0.0, unit: "H" }),
    ("conventional value of Josephson constant", Constant { value: 483_597_900_000_000.0, uncertainty: 0.0, unit: "Hz V^-1" }),
    ("conventional value of ohm-90", Constant { value: 1.000_000_017_789_999_92, uncertainty: 0.0, unit: "ohm" }),
    ("conventional value of volt-90", Constant { value: 1.000_000_106_659_999_94, uncertainty: 0.0, unit: "V" }),
    ("conventional value of von Klitzing constant", Constant { value: 25_812.807_000_000_000_7, uncertainty: 0.0, unit: "ohm" }),
    ("conventional value of watt-90", Constant { value: 1.000_000_195_529_999_97, uncertainty: 0.0, unit: "W" }),
    ("Copper x unit", Constant { value: 1.002_076_970_000_000_01e-13, uncertainty: 2.800_000_000_000_000_27e-20, unit: "m" }),
    ("deuteron-electron mag. mom. ratio", Constant { value: -0.000_466_434_555_000_000_005, uncertainty: 1.199_999_999_999_999_94e-12, unit: "" }),
    ("deuteron-electron mass ratio", Constant { value: 3_670.482_967_654_999_81, uncertainty: 6.299_999_999_999_999_5e-08, unit: "" }),
    ("deuteron g factor", Constant { value: 0.857_438_233_500_000_035, uncertainty: 2.199_999_999_999_999_85e-09, unit: "" }),
    ("deuteron mag. mom.", Constant { value: 4.330_735_086_999_999_71e-27, uncertainty: 1.099_999_999_999_999_97e-35, unit: "J T^-1" }),
    ("deuteron mag. mom. to Bohr magneton ratio", Constant { value: 0.000_466_975_456_800_000_014, uncertainty: 1.199_999_999_999_999_94e-12, unit: "" }),
    ("deuteron mag. mom. to nuclear magneton ratio", Constant { value: 0.857_438_233_500_000_035, uncertainty: 2.199_999_999_999_999_85e-09, unit: "" }),
    ("deuteron mass", Constant { value: 3.343_583_776_800_000_06e-27, uncertainty: 9.999_999_999_999_999_41e-37, unit: "kg" }),
    ("deuteron mass energy equivalent", Constant { value: 3.005_063_234_909_999_96e-10, uncertainty: 9.400_000_000_000_000_3e-20, unit: "J" }),
    ("deuteron mass energy equivalent in MeV", Constant { value: 1_875.612_945_000_000_08, uncertainty: 5.799_999_999_999_999_53e-07, unit: "MeV" }),
    ("deuteron mass in u", Constant { value: 2.013_553_212_544_000_1, uncertainty: 1.499_999_999_999_999_99e-11, unit: "u" }),
    ("deuteron molar mass", Constant { value: 0.002_013_553_214_659_999_8, uncertainty: 6.300_000_000_000_000_42e-13, unit: "kg mol^-1" }),
    ("deuteron-neutron mag. mom. ratio", Constant { value: -0.448_206_519_999_999_997, uncertainty: 1.100_000_000_000_000_06e-07, unit: "" }),
    ("deuteron-proton mag. mom. ratio", Constant { value: 0.307_012_209_300_000_005, uncertainty: 7.899_999_999_999_999_6e-10, unit: "" }),
    ("deuteron-proton mass ratio", Constant { value: 1.999_007_501_269_900_01, uncertainty: 8.399_999_999_999_999_75e-12, unit: "" }),
    ("deuteron relative atomic mass", Constant { value: 2.013_553_212_544_000_1, uncertainty: 1.499_999_999_999_999_99e-11, unit: "" }),
    ("deuteron rms charge radius", Constant { value: 2.127_780_000_000_000_03e-15, uncertainty: 2.700_000_000_000_000_08e-19, unit: "m" }),
    ("electron charge to mass quotient", Constant { value: -175_882_000_838.0, uncertainty: 55.0, unit: "C kg^-1" }),
    ("electron-deuteron mag. mom. ratio", Constant { value: -2_143.923_492_100_000_2, uncertainty: 5.599_999_999_999_999_75e-06, unit: "" }),
    ("electron-deuteron mass ratio", Constant { value: 0.000_272_443_710_762_900_013, uncertainty: 4.699_999_999_999_999_85e-15, unit: "" }),
    ("electron g factor", Constant { value: -2.002_319_304_360_919_98, uncertainty: 3.599_999_999_999_999_81e-13, unit: "" }),
    ("electron gyromag. ratio", Constant { value: 176_085_962_784.0, uncertainty: 55.0, unit: "s^-1 T^-1" }),
    ("electron gyromag. ratio in MHz/T", Constant { value: 28_024.951_386_100_001_4, uncertainty: 8.699_999_999_999_999_71e-06, unit: "MHz T^-1" }),
    ("electron-helion mass ratio", Constant { value: 0.000_181_954_307_464_899_989, uncertainty: 5.300_000_000_000_000_14e-15, unit: "" }),
    ("electron mag. mom.", Constant { value: -9.284_764_691_700_000_03e-24, uncertainty: 2.900_000_000_000_000_03e-33, unit: "J T^-1" }),
    ("electron mag. mom. anomaly", Constant { value: 0.001_159_652_180_459_999_97, uncertainty: 1.799_999_999_999_999_9e-13, unit: "" }),
    ("electron mag. mom. to Bohr magneton ratio", Constant { value: -1.001_159_652_180_459_99, uncertainty: 1.799_999_999_999_999_9e-13, unit: "" }),
    ("electron mag. mom. to nuclear magneton ratio", Constant { value: -1_838.281_971_876_999_93, uncertainty: 3.200_000_000_000_000_2e-08, unit: "" }),
    ("electron mass", Constant { value: 9.109_383_713_899_999_82e-31, uncertainty: 2.799_999_999_999_999_97e-40, unit: "kg" }),
    ("electron mass energy equivalent", Constant { value: 8.187_105_788_000_000_12e-14, uncertainty: 2.599_999_999_999_999_99e-23, unit: "J" }),
    ("electron mass energy equivalent in MeV", Constant { value: 0.510_998_950_690_000_009, uncertainty: 1.599_999_999_999_999_9e-10, unit: "MeV" }),
    ("electron mass in u", Constant { value: 0.000_548_579_909_044_099_971, uncertainty: 9.700_000_000_000_000_64e-15, unit: "u" }),
    ("electron molar mass", Constant { value: 5.485_799_096_199_999_47e-07, uncertainty: 1.700_000_000_000_000_01e-16, unit: "kg mol^-1" }),
    ("electron-muon mag. mom. ratio", Constant { value: 206.766_988_099_999_992, uncertainty: 4.6e-06, unit: "" }),
    ("electron-muon mass ratio", Constant { value: 0.004_836_331_699_999_999_94, uncertainty: 1.099_999_999_999_999_95e-10, unit: "" }),
    ("electron-neutron mag. mom. ratio", Constant { value: 960.920_479_999_999_998, uncertainty: 0.000_230_000_000_000_000_007, unit: "" }),
    ("electron-neutron mass ratio", Constant { value: 0.000_543_867_344_160_000_048, uncertainty: 2.199_999_999_999_999_97e-13, unit: "" }),
    ("electron-proton mag. mom. ratio", Constant { value: -658.210_687_890_000_031, uncertainty: 1.900_000_000_000_000_07e-07, unit: "" }),
    ("electron-proton mass ratio", Constant { value: 0.000_544_617_021_488_900_022, uncertainty: 9.399_999_999_999_999_7e-15, unit: "" }),
    ("electron relative atomic mass", Constant { value: 0.000_548_579_909_044_099_971, uncertainty: 9.700_000_000_000_000_64e-15, unit: "" }),
    ("electron-tau mass ratio", Constant { value: 0.000_287_584_999_999_999_978, uncertainty: 1.900_000_000_000_000_14e-08, unit: "" }),
    ("electron to alpha particle mass ratio", Constant { value: 0.000_137_093_355_473_299_989, uncertainty: 3.199_999_999_999_999_93e-15, unit: "" }),
    ("electron to shielded helion mag. mom. ratio", Constant { value: 864.058_239_859_999_958, uncertainty: 6.999_999_999_999_999_68e-07, unit: "" }),
    ("electron to shielded proton mag. mom. ratio", Constant { value: -658.227_585_599_999_998, uncertainty: 2.699_999_999_999_999_98e-06, unit: "" }),
    ("electron-triton mass ratio", Constant { value: 0.000_181_920_006_232_700_004, uncertainty: 6.800_000_000_000_000_06e-15, unit: "" }),
    ("electron volt", Constant { value: 1.602_176_633_999_999_89e-19, uncertainty: 0.0, unit: "J" }),
    ("electron volt-atomic mass unit relationship", Constant { value: 1.073_544_100_829_999_91e-09, uncertainty: 3.299_999_999_999_999_77e-19, unit: "u" }),
    ("electron volt-hartree relationship", Constant { value: 0.036_749_322_175_664_997_4, uncertainty: 4.0e-14, unit: "E_h" }),
    ("electron volt-hertz relationship", Constant { value: 241_798_924_200_000.0, uncertainty: 0.0, unit: "Hz" }),
    ("electron volt-inverse meter relationship", Constant { value: 806_554.393_700_000_015, uncertainty: 0.0, unit: "m^-1" }),
    ("electron volt-joule relationship", Constant { value: 1.602_176_633_999_999_89e-19, uncertainty: 0.0, unit: "J" }),
    ("electron volt-kelvin relationship", Constant { value: 11_604.518_120_000_000_6, uncertainty: 0.0, unit: "K" }),
    ("electron volt-kilogram relationship", Constant { value: 1.782_661_920_999_999_95e-36, uncertainty: 0.0, unit: "kg" }),
    ("elementary charge", Constant { value: 1.602_176_633_999_999_89e-19, uncertainty: 0.0, unit: "C" }),
    ("elementary charge over h-bar", Constant { value: 1_519_267_447_000_000.0, uncertainty: 0.0, unit: "A J^-1" }),
    ("Faraday constant", Constant { value: 96_485.332_120_000_006_4, uncertainty: 0.0, unit: "C mol^-1" }),
    ("Fermi coupling constant", Constant { value: 1.166_378_699_999_999_95e-05, uncertainty: 6.000_000_000_000_000_28e-12, unit: "GeV^-2" }),
    ("fine-structure constant", Constant { value: 0.007_297_352_564_299_999_96, uncertainty: 1.099_999_999_999_999_96e-12, unit: "" }),
    ("first radiation constant", Constant { value: 3.741_771_852_000_000_16e-16, uncertainty: 0.0, unit: "W m^2" }),
    ("first radiation constant for spectral radiance", Constant { value: 1.191_042_972_000_000_12e-16, uncertainty: 0.0, unit: "W m^2 sr^-1" }),
    ("hartree-atomic mass unit relationship", Constant { value: 2.921_262_317_969_999_9e-08, uncertainty: 9.099_999_999_999_999_34e-18, unit: "u" }),
    ("hartree-electron volt relationship", Constant { value: 27.211_386_245_981_000_1, uncertainty: 2.999_999_999_999_999_98e-11, unit: "eV" }),
    ("Hartree energy", Constant { value: 4.359_744_722_205_999_89e-18, uncertainty: 4.799_999_999_999_999_7e-30, unit: "J" }),
    ("Hartree energy in eV", Constant { value: 27.211_386_245_981_000_1, uncertainty: 2.999_999_999_999_999_98e-11, unit: "eV" }),
    ("hartree-hertz relationship", Constant { value: 6_579_683_920_499_900.0, uncertainty: 7_200.0, unit: "Hz" }),
    ("hartree-inverse meter relationship", Constant { value: 21_947_463.136_314_000_9, uncertainty: 2.400_000_000_000_000_06e-05, unit: "m^-1" }),
    ("hartree-joule relationship", Constant { value: 4.359_744_722_205_999_89e-18, uncertainty: 4.799_999_999_999_999_7e-30, unit: "J" }),
    ("hartree-kelvin relationship", Constant { value: 315_775.024_803_980_021, uncertainty: 3.399_999_999_999_999_74e-07, unit: "K" }),
    ("hartree-kilogram relationship", Constant { value: 4.850_870_209_541_900_36e-35, uncertainty: 5.299_999_999_999_999_99e-47, unit: "kg" }),
    ("helion-electron mass ratio", Constant { value: 5_495.885_279_840_000_29, uncertainty: 1.600_000_000_000_000_03e-07, unit: "" }),
    ("helion g factor", Constant { value: -4.255_250_699_500_000_34, uncertainty: 3.399_999_999_999_999_84e-09, unit: "" }),
    ("helion mag. mom.", Constant { value: -1.074_617_551_979_999_98e-26, uncertainty: 9.299_999_999_999_999_55e-36, unit: "J T^-1" }),
    ("helion mag. mom. to Bohr magneton ratio", Constant { value: -0.001_158_740_980_830_000_02, uncertainty: 9.400_000_000_000_000_34e-13, unit: "" }),
    ("helion mag. mom. to nuclear magneton ratio", Constant { value: -2.127_625_349_800_000_18, uncertainty: 1.699_999_999_999_999_92e-09, unit: "" }),
    ("helion mass", Constant { value: 5.006_412_786_199_999_95e-27, uncertainty: 1.600_000_000_000_000_11e-36, unit: "kg" }),
    ("helion mass energy equivalent", Constant { value: 4.499_539_418_499_999_94e-10, uncertainty: 1.400_000_000_000_000_01e-19, unit: "J" }),
    ("helion mass energy equivalent in MeV", Constant { value: 2_808.391_611_120_000_11, uncertainty: 8.800_000_000_000_000_45e-07, unit: "MeV" }),
    ("helion mass in u", Constant { value: 3.014_932_246_932_000_13, uncertainty: 7.400_000_000_000_000_3e-11, unit: "u" }),
    ("helion molar mass", Constant { value: 0.003_014_932_250_099_999_85, uncertainty: 9.400_000_000_000_000_34e-13, unit: "kg mol^-1" }),
    ("helion-proton mass ratio", Constant { value: 2.993_152_671_552_000_04, uncertainty: 7.000_000_000_000_000_38e-11, unit: "" }),
    ("helion relative atomic mass", Constant { value: 3.014_932_246_932_000_13, uncertainty: 7.400_000_000_000_000_3e-11, unit: "" }),
    ("helion shielding shift", Constant { value: 5.996_702_900_000_000_3e-05, uncertainty: 2.300_000_000_000_000_14e-11, unit: "" }),
    ("hertz-atomic mass unit relationship", Constant { value: 4.439_821_659_000_000_06e-24, uncertainty: 1.399_999_999_999_999_94e-33, unit: "u" }),
    ("hertz-electron volt relationship", Constant { value: 4.135_667_696_000_000_34e-15, uncertainty: 0.0, unit: "eV" }),
    ("hertz-hartree relationship", Constant { value: 1.519_829_846_057_400_02e-16, uncertainty: 1.700_000_000_000_000_03e-28, unit: "E_h" }),
    ("hertz-inverse meter relationship", Constant { value: 3.335_640_950_999_999_91e-09, uncertainty: 0.0, unit: "m^-1" }),
    ("hertz-joule relationship", Constant { value: 6.626_070_149_999_999_83e-34, uncertainty: 0.0, unit: "J" }),
    ("hertz-kelvin relationship", Constant { value: 4.799_243_072_999_999_71e-11, uncertainty: 0.0, unit: "K" }),
    ("hertz-kilogram relationship", Constant { value: 7.372_497_322_999_999_41e-51, uncertainty: 0.0, unit: "kg" }),
    ("hyperfine transition frequency of Cs-133", Constant { value: 9_192_631_770.0, uncertainty: 0.0, unit: "Hz" }),
    ("inverse fine-structure constant", Constant { value: 137.035_999_177_000_008, uncertainty: 2.099_999_999_999_999_94e-08, unit: "" }),
    ("inverse meter-atomic mass unit relationship", Constant { value: 1.331_025_048_24e-15, uncertainty: 4.099_999_999_999_999_87e-25, unit: "u" }),
    ("inverse meter-electron volt relationship", Constant { value: 1.239_841_984_000_000_01e-06, uncertainty: 0.0, unit: "eV" }),
    ("inverse meter-hartree relationship", Constant { value: 4.556_335_252_913_200_15e-08, uncertainty: 4.999_999_999_999_999_88e-20, unit: "E_h" }),
    ("inverse meter-hertz relationship", Constant { value: 299_792_458.0, uncertainty: 0.0, unit: "Hz" }),
    ("inverse meter-joule relationship", Constant { value: 1.986_445_856_999_999_92e-25, uncertainty: 0.0, unit: "J" }),
    ("inverse meter-kelvin relationship", Constant { value: 0.014_387_768_77, uncertainty: 0.0, unit: "K" }),
    ("inverse meter-kilogram relationship", Constant { value: 2.210_219_094_000_000_11e-42, uncertainty: 0.0, unit: "kg" }),
    ("inverse of conductance quantum", Constant { value: 12_906.403_720_000_000_2, uncertainty: 0.0, unit: "ohm" }),
    ("Josephson constant", Constant { value: 483_597_848_400_000.0, uncertainty: 0.0, unit: "Hz V^-1" }),
    ("joule-atomic mass unit relationship", Constant { value: 6_700_535_247.100_000_38, uncertainty: 2.100_000_000_000_000_09, unit: "u" }),
    ("joule-electron volt relationship", Constant { value: 6.241_509_074e+18, uncertainty: 0.0, unit: "eV" }),
    ("joule-hartree relationship", Constant { value: 229_371_227_839_689_984.0, uncertainty: 250_000.0, unit: "E_h" }),
    ("joule-hertz relationship", Constant { value: 1.509_190_178_999_999_89e+33, uncertainty: 0.0, unit: "Hz" }),
    ("joule-inverse meter relationship", Constant { value: 5.034_116_566_999_999_95e+24, uncertainty: 0.0, unit: "m^-1" }),
    ("joule-kelvin relationship", Constant { value: 7.242_970_516_000_000_18e+22, uncertainty: 0.0, unit: "K" }),
    ("joule-kilogram relationship", Constant { value: 1.112_650_055_999_999_99e-17, uncertainty: 0.0, unit: "kg" }),
    ("kelvin-atomic mass unit relationship", Constant { value: 9.251_087_288_400_000_04e-14, uncertainty: 2.900_000_000_000_000_16e-23, unit: "u" }),
    ("kelvin-electron volt relationship", Constant { value: 8.617_333_262_000_000_06e-05, uncertainty: 0.0, unit: "eV" }),
    ("kelvin-hartree relationship", Constant { value: 3.166_811_563_456_400_01e-06, uncertainty: 3.499_999_999_999_999_87e-18, unit: "E_h" }),
    ("kelvin-hertz relationship", Constant { value: 20_836_619_120.0, uncertainty: 0.0, unit: "Hz" }),
    ("kelvin-inverse meter relationship", Constant { value: 69.503_480_039_999_999_5, uncertainty: 0.0, unit: "m^-1" }),
    ("kelvin-joule relationship", Constant { value: 1.380_649_000_000_000_09e-23, uncertainty: 0.0, unit: "J" }),
    ("kelvin-kilogram relationship", Constant { value: 1.536_179_187_000_000_09e-40, uncertainty: 0.0, unit: "kg" }),
    ("kilogram-atomic mass unit relationship", Constant { value: 6.022_140_753_699_999_96e+26, uncertainty: 190_000_000_000_000_000.0, unit: "u" }),
    ("kilogram-electron volt relationship", Constant { value: 5.609_588_602_999_999_76e+35, uncertainty: 0.0, unit: "eV" }),
    ("kilogram-hartree relationship", Constant { value: 2.061_485_788_741_500_06e+34, uncertainty: 2.2e+22, unit: "E_h" }),
    ("kilogram-hertz relationship", Constant { value: 1.356_392_488_999_999_99e+50, uncertainty: 0.0, unit: "Hz" }),
    ("kilogram-inverse meter relationship", Constant { value: 4.524_438_334_999_999_82e+41, uncertainty: 0.0, unit: "m^-1" }),
    ("kilogram-joule relationship", Constant { value: 89_875_517_870_000_000.0, uncertainty: 0.0, unit: "J" }),
    ("kilogram-kelvin relationship", Constant { value: 6.509_657_260_000_000_32e+39, uncertainty: 0.0, unit: "K" }),
    ("lattice parameter of silicon", Constant { value: 5.431_020_511_000_000_28e-10, uncertainty: 8.900_000_000_000_000_41e-18, unit: "m" }),
    ("lattice spacing of ideal Si (220)", Constant { value: 1.920_155_715_999_999_89e-10, uncertainty: 3.199_999_999_999_999_92e-18, unit: "m" }),
    ("Loschmidt constant (273.15 K, 100 kPa)", Constant { value: 2.651_645_803_999_999_87e+25, uncertainty: 0.0, unit: "m^-3" }),
    ("Loschmidt constant (273.15 K, 101.325 kPa)", Constant { value: 2.686_780_111_000_000_06e+25, uncertainty: 0.0, unit: "m^-3" }),
    ("luminous efficacy", Constant { value: 683.0, uncertainty: 0.0, unit: "lm W^-1" }),
    ("mag. flux quantum", Constant { value: 2.067_833_848_000_000_17e-15, uncertainty: 0.0, unit: "Wb" }),
    ("molar gas constant", Constant { value: 8.314_462_618_000_000_3, uncertainty: 0.0, unit: "J mol^-1 K^-1" }),
    ("molar mass constant", Constant { value: 0.001_000_000_001_049_999_95, uncertainty: 3.099_999_999_999_999_92e-13, unit: "kg mol^-1" }),
    ("molar mass of carbon-12", Constant { value: 0.012_000_000_012_599_999_4, uncertainty: 3.699_999_999_999_999_99e-12, unit: "kg mol^-1" }),
    ("molar Planck constant", Constant { value: 3.990_312_712_000_000_01e-10, uncertainty: 0.0, unit: "J Hz^-1 mol^-1" }),
    ("molar volume of ideal gas (273.15 K, 100 kPa)", Constant { value: 0.022_710_954_639_999_999_4, uncertainty: 0.0, unit: "m^3 mol^-1" }),
    ("molar volume of ideal gas (273.15 K, 101.325 kPa)", Constant { value: 0.022_413_969_539_999_998_4, uncertainty: 0.0, unit: "m^3 mol^-1" }),
    ("molar volume of silicon", Constant { value: 1.205_883_198_999_999_99e-05, uncertainty: 5.999_999_999_999_999_68e-13, unit: "m^3 mol^-1" }),
    ("Molybdenum x unit", Constant { value: 1.002_099_520_000_000_05e-13, uncertainty: 5.300_000_000_000_000_21e-20, unit: "m" }),
    ("muon Compton wavelength", Constant { value: 1.173_444_109_999_999_96e-14, uncertainty: 2.599_999_999_999_999_99e-22, unit: "m" }),
    ("muon-electron mass ratio", Constant { value: 206.768_282_699_999_986, uncertainty: 4.6e-06, unit: "" }),
    ("muon g factor", Constant { value: -2.002_331_841_230_000_19, uncertainty: 8.199_999_999_999_999_6e-10, unit: "" }),
    ("muon mag. mom.", Constant { value: -4.490_448_299_999_999_82e-26, uncertainty: 1.000_000_000_000_000_06e-33, unit: "J T^-1" }),
    ("muon mag. mom. anomaly", Constant { value: 0.001_165_920_620_000_000_1, uncertainty: 4.099_999_999_999_999_8e-10, unit: "" }),
    ("muon mag. mom. to Bohr magneton ratio", Constant { value: -0.004_841_970_479_999_999_94, uncertainty: 1.099_999_999_999_999_95e-10, unit: "" }),
    ("muon mag. mom. to nuclear magneton ratio", Constant { value: -8.890_597_039_999_999_4, uncertainty: 1.999_999_999_999_999_91e-07, unit: "" }),
    ("muon mass", Constant { value: 1.883_531_627_000_000_11e-28, uncertainty: 4.199_999_999_999_999_82e-36, unit: "kg" }),
    ("muon mass energy equivalent", Constant { value: 1.692_833_803_999_999_89e-11, uncertainty: 3.8e-19, unit: "J" }),
    ("muon mass energy equivalent in MeV", Constant { value: 105.658_375_500_000_005, uncertainty: 2.3e-06, unit: "MeV" }),
    ("muon mass in u", Constant { value: 0.113_428_925_700_000_002, uncertainty: 2.500_000_000_000_000_05e-09, unit: "u" }),
    ("muon molar mass", Constant { value: 0.000_113_428_925_800_000_002, uncertainty: 2.499_999_999_999_999_85e-12, unit: "kg mol^-1" }),
    ("muon-neutron mass ratio", Constant { value: 0.112_454_516_800_000_001, uncertainty: 2.500_000_000_000_000_05e-09, unit: "" }),
    ("muon-proton mag. mom. ratio", Constant { value: -3.183_345_146_000_000_18, uncertainty: 7.100_000_000_000_000_05e-08, unit: "" }),
    ("muon-proton mass ratio", Constant { value: 0.112_609_526_200_000_004, uncertainty: 2.500_000_000_000_000_05e-09, unit: "" }),
    ("muon-tau mass ratio", Constant { value: 0.059_463_500_000_000_002_5, uncertainty: 3.999_999_999_999_999_82e-06, unit: "" }),
    ("natural unit of action", Constant { value: 1.054_571_816_999_999_99e-34, uncertainty: 0.0, unit: "J s" }),
    ("natural unit of action in eV s", Constant { value: 6.582_119_569_000_000_31e-16, uncertainty: 0.0, unit: "eV s" }),
    ("natural unit of energy", Constant { value: 8.187_105_788_000_000_12e-14, uncertainty: 2.599_999_999_999_999_99e-23, unit: "J" }),
    ("natural unit of energy in MeV", Constant { value: 0.510_998_950_690_000_009, uncertainty: 1.599_999_999_999_999_9e-10, unit: "MeV" }),
    ("natural unit of length", Constant { value: 3.861_592_674_399_999_85e-13, uncertainty: 1.200_000_000_000_000_01e-22, unit: "m" }),
    ("natural unit of mass", Constant { value: 9.109_383_713_899_999_82e-31, uncertainty: 2.799_999_999_999_999_97e-40, unit: "kg" }),
    ("natural unit of momentum", Constant { value: 2.730_924_534_459_999_83e-22, uncertainty: 8.500_000_000_000_000_27e-32, unit: "kg m s^-1" }),
    ("natural unit of momentum in MeV/c", Constant { value: 0.510_998_950_690_000_009, uncertainty: 1.599_999_999_999_999_9e-10, unit: "MeV/c" }),
    ("natural unit of time", Constant { value: 1.288_088_666_440_000_02e-21, uncertainty: 4.000_000_000_000_000_33e-31, unit: "s" }),
    ("natural unit of velocity", Constant { value: 299_792_458.0, uncertainty: 0.0, unit: "m s^-1" }),
    ("neutron Compton wavelength", Constant { value: 1.319_590_903_820_000_01e-15, uncertainty: 6.700_000_000_000_000_32e-25, unit: "m" }),
    ("neutron-electron mag. mom. ratio", Constant { value: 0.001_040_668_839_999_999_91, uncertainty: 2.399_999_999_999_999_98e-10, unit: "" }),
    ("neutron-electron mass ratio", Constant { value: 1_838.683_661_999_999_91, uncertainty: 7.400_000_000_000_000_09e-07, unit: "" }),
    ("neutron g factor", Constant { value: -3.826_085_519_999_999_91, uncertainty: 8.999_999_999_999_999_59e-07, unit: "" }),
    ("neutron gyromag. ratio", Constant { value: 183_247_174.0, uncertainty: 43.0, unit: "s^-1 T^-1" }),
    ("neutron gyromag. ratio in MHz/T", Constant { value: 29.164_693_499_999_998_5, uncertainty: 6.900_000_000_000_000_01e-06, unit: "MHz T^-1" }),
    ("neutron mag. mom.", Constant { value: -9.662_365_3e-27, uncertainty: 2.299_999_999_999_999_86e-33, unit: "J T^-1" }),
    ("neutron mag. mom. to Bohr magneton ratio", Constant { value: -0.001_041_875_650_000_000_05, uncertainty: 2.500_000_000_000_000_16e-10, unit: "" }),
    ("neutron mag. mom. to nuclear magneton ratio", Constant { value: -1.913_042_759_999_999_95, uncertainty: 4.499_999_999_999_999_8e-07, unit: "" }),
    ("neutron mass", Constant { value: 1.674_927_500_560_000_11e-27, uncertainty: 8.500_000_000_000_000_67e-37, unit: "kg" }),
    ("neutron mass energy equivalent", Constant { value: 1.505_349_765_140_000_02e-10, uncertainty: 7.599_999_999_999_999_52e-20, unit: "J" }),
    ("neutron mass energy equivalent in MeV", Constant { value: 939.565_421_939_999_965, uncertainty: 4.799_999_999_999_999_57e-07, unit: "MeV" }),
    ("neutron mass in u", Constant { value: 1.008_664_916_060_000_08, uncertainty: 4.000_000_000_000_000_15e-10, unit: "u" }),
    ("neutron molar mass", Constant { value: 0.001_008_664_917_12, uncertainty: 5.100_000_000_000_000_48e-13, unit: "kg mol^-1" }),
    ("neutron-muon mass ratio", Constant { value: 8.892_484_079_999_999_12, uncertainty: 1.999_999_999_999_999_91e-07, unit: "" }),
    ("neutron-proton mag. mom. ratio", Constant { value: -0.684_979_349_999_999_987, uncertainty: 1.600_000_000_000_000_03e-07, unit: "" }),
    ("neutron-proton mass difference", Constant { value: 2.305_574_610_000_000_14e-30, uncertainty: 6.700_000_000_000_000_01e-37, unit: "kg" }),
    ("neutron-proton mass difference energy equivalent", Constant { value: 2.072_147_119_999_999_92e-13, uncertainty: 6.000_000_000_000_000_57e-20, unit: "J" }),
    ("neutron-proton mass difference energy equivalent in MeV", Constant { value: 1.293_332_509_999_999_94, uncertainty: 3.800_000_000_000_000_15e-07, unit: "MeV" }),
    ("neutron-proton mass difference in u", Constant { value: 0.001_388_449_479_999_999_97, uncertainty: 4.000_000_000_000_000_15e-10, unit: "u" }),
    ("neutron-proton mass ratio", Constant { value: 1.001_378_419_459_999_95, uncertainty: 4.000_000_000_000_000_15e-10, unit: "" }),
    ("neutron relative atomic mass", Constant { value: 1.008_664_916_060_000_08, uncertainty: 4.000_000_000_000_000_15e-10, unit: "" }),
    ("neutron-tau mass ratio", Constant { value: 0.528_778_999_999_999_999, uncertainty: 3.600_000_000_000_000_09e-05, unit: "" }),
    ("neutron to shielded proton mag. mom. ratio", Constant { value: -0.684_996_939_999_999_999, uncertainty: 1.600_000_000_000_000_03e-07, unit: "" }),
    ("Newtonian constant of gravitation", Constant { value: 6.674_299_999_999_999_38e-11, uncertainty: 1.499_999_999_999_999_92e-15, unit: "m^3 kg^-1 s^-2" }),
    ("Newtonian constant of gravitation over h-bar c", Constant { value: 6.708_830_000_000_000_55e-39, uncertainty: 1.500_000_000_000_000_02e-43, unit: "(GeV/c^2)^-2" }),
    ("nuclear magneton", Constant { value: 5.050_783_739_300_000_11e-27, uncertainty: 1.600_000_000_000_000_11e-36, unit: "J T^-1" }),
    ("nuclear magneton in eV/T", Constant { value: 3.152_451_254_169_999_89e-08, uncertainty: 9.800_000_000_000_000_24e-18, unit: "eV T^-1" }),
    ("nuclear magneton in inverse meter per tesla", Constant { value: 0.025_426_234_100_899_999_1, uncertainty: 7.899_999_999_999_999_86e-12, unit: "m^-1 T^-1" }),
    ("nuclear magneton in K/T", Constant { value: 0.000_365_826_777_059_999_981, uncertainty: 1.099_999_999_999_999_98e-13, unit: "K T^-1" }),
    ("nuclear magneton in MHz/T", Constant { value: 7.622_593_218_799_999_63, uncertainty: 2.399_999_999_999_999_98e-09, unit: "MHz T^-1" }),
    ("Planck constant", Constant { value: 6.626_070_149_999_999_83e-34, uncertainty: 0.0, unit: "J Hz^-1" }),
    ("Planck constant in eV/Hz", Constant { value: 4.135_667_696_000_000_34e-15, uncertainty: 0.0, unit: "eV Hz^-1" }),
    ("Planck length", Constant { value: 1.616_255_000_000_000_07e-35, uncertainty: 1.800_000_000_000_000_04e-40, unit: "m" }),
    ("Planck mass", Constant { value: 2.176_434_000_000_000_14e-08, uncertainty: 2.399_999_999_999_999_87e-13, unit: "kg" }),
    ("Planck mass energy equivalent in GeV", Constant { value: 1.220_89e+19, uncertainty: 140_000_000_000_000.0, unit: "GeV" }),
    ("Planck temperature", Constant { value: 1.416_784_000_000_000_01e+32, uncertainty: 1.600_000_000_000_000_08e+27, unit: "K" }),
    ("Planck time", Constant { value: 5.391_247_000_000_000_16e-44, uncertainty: 6.0e-49, unit: "s" }),
    ("proton charge to mass quotient", Constant { value: 95_788_331.430_000_007_2, uncertainty: 0.029_999_999_999_999_998_9, unit: "C kg^-1" }),
    ("proton Compton wavelength", Constant { value: 1.321_409_853_600_000_01e-15, uncertainty: 4.099_999_999_999_999_87e-25, unit: "m" }),
    ("proton-electron mass ratio", Constant { value: 1_836.152_673_426_000_09, uncertainty: 3.200_000_000_000_000_2e-08, unit: "" }),
    ("proton g factor", Constant { value: 5.585_694_689_300_000_35, uncertainty: 1.600_000_000_000_000_06e-09, unit: "" }),
    ("proton gyromag. ratio", Constant { value: 267_522_187.080_000_013, uncertainty: 0.110_000_000_000_000_001, unit: "s^-1 T^-1" }),
    ("proton gyromag. ratio in MHz/T", Constant { value: 42.577_478_460_999_998_3, uncertainty: 1.799_999_999_999_999_91e-08, unit: "MHz T^-1" }),
    ("proton mag. mom.", Constant { value: 1.410_606_795_450_000_13e-26, uncertainty: 6.000_000_000_000_000_31e-36, unit: "J T^-1" }),
    ("proton mag. mom. to Bohr magneton ratio", Constant { value: 0.001_521_032_202_299_999_96, uncertainty: 4.500_000_000_000_000_01e-13, unit: "" }),
    ("proton mag. mom. to nuclear magneton ratio", Constant { value: 2.792_847_344_630_000_17, uncertainty: 8.199_999_999_999_999_6e-10, unit: "" }),
    ("proton mag. shielding correction", Constant { value: 2.567_149_999_999_999_98e-05, uncertainty: 4.100_000_000_000_000_32e-09, unit: "" }),
    ("proton mass", Constant { value: 1.672_621_925_949_999_91e-27, uncertainty: 5.199_999_999_999_999_59e-37, unit: "kg" }),
    ("proton mass energy equivalent", Constant { value: 1.503_277_618_02e-10, uncertainty: 4.700_000_000_000_000_15e-20, unit: "J" }),
    ("proton mass energy equivalent in MeV", Constant { value: 938.272_089_430_000_051, uncertainty: 2.899_999_999_999_999_76e-07, unit: "MeV" }),
    ("proton mass in u", Constant { value: 1.007_276_466_578_900_02, uncertainty: 8.299_999_999_999_999_77e-12, unit: "u" }),
    ("proton molar mass", Constant { value: 0.001_007_276_467_639_999_95, uncertainty: 3.099_999_999_999_999_92e-13, unit: "kg mol^-1" }),
    ("proton-muon mass ratio", Constant { value: 8.880_243_379_999_999_58, uncertainty: 1.999_999_999_999_999_91e-07, unit: "" }),
    ("proton-neutron mag. mom. ratio", Constant { value: -1.459_898_020_000_000_02, uncertainty: 3.399_999_999_999_999_74e-07, unit: "" }),
    ("proton-neutron mass ratio", Constant { value: 0.998_623_477_969_999_951, uncertainty: 4.000_000_000_000_000_15e-10, unit: "" }),
    ("proton relative atomic mass", Constant { value: 1.007_276_466_578_900_02, uncertainty: 8.299_999_999_999_999_77e-12, unit: "" }),
    ("proton rms charge radius", Constant { value: 8.407_500_000_000_000_04e-16, uncertainty: 6.399_999_999_999_999_65e-19, unit: "m" }),
    ("proton-tau mass ratio", Constant { value: 0.528_051_000_000_000_048, uncertainty: 3.600_000_000_000_000_09e-05, unit: "" }),
    ("quantum of circulation", Constant { value: 0.000_363_694_754_670_000_012, uncertainty: 1.099_999_999_999_999_98e-13, unit: "m^2 s^-1" }),
    ("quantum of circulation times 2", Constant { value: 0.000_727_389_509_340_000_025, uncertainty: 2.299_999_999_999_999_79e-13, unit: "m^2 s^-1" }),
    ("reduced Compton wavelength", Constant { value: 3.861_592_674_399_999_85e-13, uncertainty: 1.200_000_000_000_000_01e-22, unit: "m" }),
    ("reduced muon Compton wavelength", Constant { value: 1.867_594_305_999_999_98e-15, uncertainty: 4.200_000_000_000_000_16e-23, unit: "m" }),
    ("reduced neutron Compton wavelength", Constant { value: 2.100_194_151_999_999_95e-16, uncertainty: 1.100_000_000_000_000_1e-25, unit: "m" }),
    ("reduced Planck constant", Constant { value: 1.054_571_816_999_999_99e-34, uncertainty: 0.0, unit: "J s" }),
    ("reduced Planck constant in eV s", Constant { value: 6.582_119_569_000_000_31e-16, uncertainty: 0.0, unit: "eV s" }),
    ("reduced Planck constant times c in MeV fm", Constant { value: 197.326_980_399_999_997, uncertainty: 0.0, unit: "MeV fm" }),
    ("reduced proton Compton wavelength", Constant { value: 2.103_089_100_509_999_89e-16, uncertainty: 6.599_999_999_999_999_68e-26, unit: "m" }),
    ("reduced tau Compton wavelength", Constant { value: 1.110_538_000_000_000_06e-16, uncertainty: 7.500_000_000_000_000_72e-21, unit: "m" }),
    ("Rydberg constant", Constant { value: 10_973_731.568_157_000_5, uncertainty: 1.200_000_000_000_000_03e-05, unit: "m^-1" }),
    ("Rydberg constant times c in Hz", Constant { value: 3_289_841_960_250_000.0, uncertainty: 3_600.0, unit: "Hz" }),
    ("Rydberg constant times hc in eV", Constant { value: 13.605_693_122_990_000_9, uncertainty: 1.499_999_999_999_999_99e-11, unit: "eV" }),
    ("Rydberg constant times hc in J", Constant { value: 2.179_872_361_102_999_95e-18, uncertainty: 2.399_999_999_999_999_85e-30, unit: "J" }),
    ("Sackur-Tetrode constant (1 K, 100 kPa)", Constant { value: -1.151_707_534_959_999_89, uncertainty: 4.700_000_000_000_000_31e-10, unit: "" }),
    ("Sackur-Tetrode constant (1 K, 101.325 kPa)", Constant { value: -1.164_870_521_489_999_92, uncertainty: 4.700_000_000_000_000_31e-10, unit: "" }),
    ("second radiation constant", Constant { value: 0.014_387_768_77, uncertainty: 0.0, unit: "m K" }),
    ("shielded helion gyromag. ratio", Constant { value: 203_789_460.780_000_001, uncertainty: 0.179_999_999_999_999_993, unit: "s^-1 T^-1" }),
    ("shielded helion gyromag. ratio in MHz/T", Constant { value: 32.434_100_033, uncertainty: 2.799_999_999_999_999_93e-08, unit: "MHz T^-1" }),
    ("shielded helion mag. mom.", Constant { value: -1.074_553_110_350_000_05e-26, uncertainty: 9.299_999_999_999_999_55e-36, unit: "J T^-1" }),
    ("shielded helion mag. mom. to Bohr magneton ratio", Constant { value: -0.001_158_671_494_569_999_98, uncertainty: 9.400_000_000_000_000_34e-13, unit: "" }),
    ("shielded helion mag. mom. to nuclear magneton ratio", Constant { value: -2.127_497_762_399_999_99, uncertainty: 1.699_999_999_999_999_92e-09, unit: "" }),
    ("shielded helion to proton mag. mom. ratio", Constant { value: -0.761_766_577_209_999_962, uncertainty: 6.599_999_999_999_999_96e-10, unit: "" }),
    ("shielded helion to shielded proton mag. mom. ratio", Constant { value: -0.761_786_133_400_000_01, uncertainty: 3.100_000_000_000_000_05e-09, unit: "" }),
    ("shielded proton gyromag. ratio", Constant { value: 267_515_319.400_000_006, uncertainty: 1.100_000_000_000_000_09, unit: "s^-1 T^-1" }),
    ("shielded proton gyromag. ratio in MHz/T", Constant { value: 42.576_385_430_000_002, uncertainty: 1.699_999_999_999_999_87e-07, unit: "MHz T^-1" }),
    ("shielded proton mag. mom.", Constant { value: 1.410_570_583_000_000_01e-26, uncertainty: 5.800_000_000_000_000_39e-35, unit: "J T^-1" }),
    ("shielded proton mag. mom. to Bohr magneton ratio", Constant { value: 0.001_520_993_155_099_999_9, uncertainty: 6.200_000_000_000_000_24e-12, unit: "" }),
    ("shielded proton mag. mom. to nuclear magneton ratio", Constant { value: 2.792_775_648_000_000_11, uncertainty: 1.099_999_999_999_999_92e-08, unit: "" }),
    ("shielding difference of d and p in HD", Constant { value: 1.987_699_999_999_999_99e-08, uncertainty: 9.999_999_999_999_999_8e-13, unit: "" }),
    ("shielding difference of t and p in HT", Constant { value: 2.394_500_000_000_000_03e-08, uncertainty: 1.999_999_999_999_999_96e-12, unit: "" }),
    ("speed of light in vacuum", Constant { value: 299_792_458.0, uncertainty: 0.0, unit: "m s^-1" }),
    ("standard acceleration of gravity", Constant { value: 9.806_649_999_999_999_42, uncertainty: 0.0, unit: "m s^-2" }),
    ("standard atmosphere", Constant { value: 101_325.0, uncertainty: 0.0, unit: "Pa" }),
    ("standard-state pressure", Constant { value: 100_000.0, uncertainty: 0.0, unit: "Pa" }),
    ("Stefan-Boltzmann constant", Constant { value: 5.670_374_418_999_999_91e-08, uncertainty: 0.0, unit: "W m^-2 K^-4" }),
    ("tau Compton wavelength", Constant { value: 6.977_710_000_000_000_14e-16, uncertainty: 4.700_000_000_000_000_15e-20, unit: "m" }),
    ("tau-electron mass ratio", Constant { value: 3_477.230_000_000_000_02, uncertainty: 0.230_000_000_000_000_01, unit: "" }),
    ("tau energy equivalent", Constant { value: 1_776.859_999_999_999_9, uncertainty: 0.119_999_999_999_999_996, unit: "MeV" }),
    ("tau mass", Constant { value: 3.167_539_999_999_999_91e-27, uncertainty: 2.099_999_999_999_999_91e-31, unit: "kg" }),
    ("tau mass energy equivalent", Constant { value: 2.846_839_999_999_999_76e-10, uncertainty: 1.900_000_000_000_000_11e-14, unit: "J" }),
    ("tau mass in u", Constant { value: 1.907_540_000_000_000_01, uncertainty: 0.000_129_999_999_999_999_989, unit: "u" }),
    ("tau molar mass", Constant { value: 0.001_907_540_000_000_000_1, uncertainty: 1.299_999_999_999_999_99e-07, unit: "kg mol^-1" }),
    ("tau-muon mass ratio", Constant { value: 16.817_000_000_000_000_2, uncertainty: 0.001_100_000_000_000_000_07, unit: "" }),
    ("tau-neutron mass ratio", Constant { value: 1.891_150_000_000_000_11, uncertainty: 0.000_129_999_999_999_999_989, unit: "" }),
    ("tau-proton mass ratio", Constant { value: 1.893_760_000_000_000_11, uncertainty: 0.000_129_999_999_999_999_989, unit: "" }),
    ("Thomson cross section", Constant { value: 6.652_458_705_100_000_15e-29, uncertainty: 6.199_999_999_999_999_66e-38, unit: "m^2" }),
    ("triton-electron mass ratio", Constant { value: 5_496.921_535_509_999_56, uncertainty: 2.100_000_000_000_000_01e-07, unit: "" }),
    ("triton g factor", Constant { value: 5.957_924_929_999_999_9, uncertainty: 1.199_999_999_999_999_99e-08, unit: "" }),
    ("triton mag. mom.", Constant { value: 1.504_609_517_799_999_9e-26, uncertainty: 2.999_999_999_999_999_89e-35, unit: "J T^-1" }),
    ("triton mag. mom. to Bohr magneton ratio", Constant { value: 0.001_622_393_664_799_999_96, uncertainty: 3.200_000_000_000_000_1e-12, unit: "" }),
    ("triton mag. mom. to nuclear magneton ratio", Constant { value: 2.978_962_464_999_999_95, uncertainty: 5.899_999_999_999_999_89e-09, unit: "" }),
    ("triton mass", Constant { value: 5.007_356_751_199_999_82e-27, uncertainty: 1.600_000_000_000_000_11e-36, unit: "kg" }),
    ("triton mass energy equivalent", Constant { value: 4.500_387_811_899_999_92e-10, uncertainty: 1.400_000_000_000_000_01e-19, unit: "J" }),
    ("triton mass energy equivalent in MeV", Constant { value: 2_808.921_136_680_000_02, uncertainty: 8.800_000_000_000_000_45e-07, unit: "MeV" }),
    ("triton mass in u", Constant { value: 3.015_500_715_970_000_02, uncertainty: 1.000_000_000_000_000_04e-10, unit: "u" }),
    ("triton molar mass", Constant { value: 0.003_015_500_719_129_999_86, uncertainty: 9.400_000_000_000_000_34e-13, unit: "kg mol^-1" }),
    ("triton-proton mass ratio", Constant { value: 2.993_717_034_029_999_89, uncertainty: 1.000_000_000_000_000_04e-10, unit: "" }),
    ("triton relative atomic mass", Constant { value: 3.015_500_715_970_000_02, uncertainty: 1.000_000_000_000_000_04e-10, unit: "" }),
    ("triton to proton mag. mom. ratio", Constant { value: 1.066_639_918_899_999_98, uncertainty: 2.100_000_000_000_000_19e-09, unit: "" }),
    ("unified atomic mass unit", Constant { value: 1.660_539_068_919_999_93e-27, uncertainty: 5.199_999_999_999_999_59e-37, unit: "kg" }),
    ("vacuum electric permittivity", Constant { value: 8.854_187_818_800_000_41e-12, uncertainty: 1.400_000_000_000_000_02e-21, unit: "F m^-1" }),
    ("vacuum mag. permeability", Constant { value: 1.256_637_061_270_000_05e-06, uncertainty: 1.999_999_999_999_999_96e-16, unit: "N A^-2" }),
    ("von Klitzing constant", Constant { value: 25_812.807_450_000_000_2, uncertainty: 0.0, unit: "ohm" }),
    ("weak mixing angle", Constant { value: 0.223_049_999_999_999_998, uncertainty: 0.000_230_000_000_000_000_007, unit: "" }),
    ("Wien frequency displacement law constant", Constant { value: 58_789_257_570.0, uncertainty: 0.0, unit: "Hz K^-1" }),
    ("Wien wavelength displacement law constant", Constant { value: 0.002_897_771_955_000_000_03, uncertainty: 0.0, unit: "m K" }),
    ("W to Z mass ratio", Constant { value: 0.881_449_999_999_999_956, uncertainty: 0.000_129_999_999_999_999_989, unit: "" }),
];
