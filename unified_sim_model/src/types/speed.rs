use std::fmt::Display;

pub const MS_TO_KMH: f32 = 3.6;
pub const MS_TO_MPH: f32 = 2.23694;

/// A speed value.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Speed {
    /// The speed in meter per second.
    pub ms: f32,
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m/s", self.as_ms())
    }
}

impl Speed {
    /// Creates a speed from the velocity in meter per second.
    #[allow(dead_code)]
    pub fn from_ms(v: f32) -> Self {
        Self { ms: v }
    }

    /// Creates a speed from the velocity in kilometers per hour.
    #[allow(dead_code)]
    pub fn from_kmh(v: f32) -> Self {
        Self { ms: v / MS_TO_KMH }
    }

    /// Creates a speed from the velocity in miles per hour.
    #[allow(dead_code)]
    pub fn from_mph(v: f32) -> Self {
        Self { ms: v / MS_TO_MPH }
    }

    /// Returns the speed as meter per second.
    #[allow(dead_code)]
    pub fn as_ms(&self) -> f32 {
        self.ms
    }

    /// Returns the speed in kilometer per hour.
    #[allow(dead_code)]
    pub fn as_kmh(&self) -> f32 {
        self.ms * MS_TO_KMH
    }

    /// Returns the speed in miles per hour.
    #[allow(dead_code)]
    pub fn as_mph(&self) -> f32 {
        self.ms * MS_TO_MPH
    }
}
