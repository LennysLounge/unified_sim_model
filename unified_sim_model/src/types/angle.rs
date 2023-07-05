pub const RAD_TO_DEGREE: f32 = 57.2958;

/// An angle value.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Angle {
    /// The angle in radians
    pub rad: f32,
}

impl Angle {
    /// Create a angle from radians.
    #[allow(dead_code)]
    pub fn from_rad(v: f32) -> Self {
        Self { rad: v }
    }

    /// Create a angle from degrees.
    #[allow(dead_code)]
    pub fn from_deg(v: f32) -> Self {
        Self {
            rad: v / RAD_TO_DEGREE,
        }
    }

    /// Return the angle in rad.
    #[allow(dead_code)]
    pub fn as_rad(&self) -> f32 {
        self.rad
    }

    /// Return the angle in degrees.
    #[allow(dead_code)]
    pub fn as_deg(&self) -> f32 {
        self.rad * RAD_TO_DEGREE
    }
}
