use std::fmt::Display;

/// A temperature value.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Temperature {
    /// The temperature in celcius.
    pub c: f32,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} °C", self.as_celcius())
    }
}

impl Temperature {
    /// Create a temperature from degrees celcius.
    #[allow(dead_code)]
    pub fn from_celcius(v: f32) -> Self {
        Self { c: v }
    }

    /// Create a temperature from degrees celcius.
    #[allow(dead_code)]
    pub fn from_fahrenheit(f: f32) -> Self {
        Self { c: to_celcius(f) }
    }

    /// Return the temperature in c.
    #[allow(dead_code)]
    pub fn as_celcius(&self) -> f32 {
        self.c
    }

    /// Return the temperature in fahrenheit.
    #[allow(dead_code)]
    pub fn as_fahrenheit(&self) -> f32 {
        to_fahrenheit(self.c)
    }
}

/// Convert a temperature in celcius to a temperature in fahrenheit.
pub fn to_fahrenheit(c: f32) -> f32 {
    c * 1.8 + 32.0
}

/// Convert a temperature in fahrenheit to a temeratpure in celcius.
pub fn to_celcius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
