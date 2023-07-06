pub const KPA_TO_INCHES_HG_AT_ZERO_C: f32 = 0.2953005;

/// A pressure value
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Pressure {
    /// The pressure in kilo pacal
    pub kpa: f32,
}

impl Pressure {
    /// Create a pressure from kilo pascal.
    pub fn from_kpa(v: f32) -> Self {
        Self { kpa: v }
    }

    /// Create a pressure from inches of mercury.
    /// Internaly this is stored in kilo pascal and the conversion is dependant
    /// on temperature. This conversion asumes a mercury temperature of 0 degrees celcius.
    pub fn from_inches_hg(v: f32) -> Self {
        Self {
            kpa: v / KPA_TO_INCHES_HG_AT_ZERO_C,
        }
    }

    /// Return the pressure in kilo pascal.
    pub fn as_kpa(&self) -> f32 {
        self.kpa
    }

    /// Return the pressure in inches of mercury.
    /// Internaly this is stored in kilo pascal and the conversion is dependant
    /// on temperature. This conversion asumes a mercury temperature of 0 degrees celcius.
    pub fn as_inches_hg(&self) -> f32 {
        self.kpa * KPA_TO_INCHES_HG_AT_ZERO_C
    }
}
