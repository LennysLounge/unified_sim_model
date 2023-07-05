use std::fmt::Display;

pub const METER_TO_KILOMETER: f32 = 0.001;
pub const METER_TO_MILE: f32 = 0.000621371;
pub const METER_TO_FEET: f32 = 3.28084;

/// A distance value
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Distance {
    /// The distance in meter.
    pub meter: f32,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.as_meters())
    }
}

impl Distance {
    /// Create a distance from meters.
    #[allow(dead_code)]
    pub fn from_meters(v: f32) -> Self {
        Self { meter: v }
    }

    /// Create a distance from kilometers.
    #[allow(dead_code)]
    pub fn from_kilometers(v: f32) -> Self {
        Self {
            meter: v / METER_TO_KILOMETER,
        }
    }
    /// Create a distance from miles.
    #[allow(dead_code)]
    pub fn from_miles(v: f32) -> Self {
        Self {
            meter: v / METER_TO_MILE,
        }
    }
    /// Create a distance from feet.
    #[allow(dead_code)]
    pub fn from_feet(v: f32) -> Self {
        Self {
            meter: v / METER_TO_FEET,
        }
    }

    /// Return the distance in meter.
    #[allow(dead_code)]
    pub fn as_meters(&self) -> f32 {
        self.meter
    }

    /// Return the distance in kilometer.
    #[allow(dead_code)]
    pub fn as_kilometers(&self) -> f32 {
        self.meter * METER_TO_KILOMETER
    }

    /// Return the distance in miles.
    #[allow(dead_code)]
    pub fn as_miles(&self) -> f32 {
        self.meter * METER_TO_MILE
    }

    /// Return the distance in feet.
    #[allow(dead_code)]
    pub fn as_feet(&self) -> f32 {
        self.meter * METER_TO_FEET
    }
}
