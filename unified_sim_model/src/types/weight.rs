pub const KG_TO_LBS: f32 = 2.20462;

/// A weight value.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Weight {
    /// The weight in kg.
    pub kg: f32,
}

impl Weight {
    /// Create a weight from kg.
    #[allow(dead_code)]
    pub fn from_kg(v: f32) -> Self {
        Self { kg: v }
    }
    /// Create a weight from lbs.
    #[allow(dead_code)]
    pub fn from_lbs(v: f32) -> Self {
        Self { kg: v / KG_TO_LBS }
    }

    /// Return the weight in kilograms
    #[allow(dead_code)]
    pub fn as_kg(&self) -> f32 {
        self.kg
    }

    /// Return the weight in lbs.
    #[allow(dead_code)]
    pub fn as_lbs(&self) -> f32 {
        self.kg * KG_TO_LBS
    }
}
