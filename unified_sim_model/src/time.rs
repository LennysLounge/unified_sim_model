use std::fmt::Display;

/// A Time value. Represented in milliseconds.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Time {
    /// The time value as milliseconds.
    pub ms: i32,
}

impl From<i32> for Time {
    /// Convert a i32 of milliseconds to Time.
    fn from(value: i32) -> Self {
        Self { ms: value }
    }
}

impl From<f32> for Time {
    /// Convert f32 of milliseconds to Time.
    fn from(value: f32) -> Self {
        Self { ms: value as i32 }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Time {
    /// Format a time as hh:mm:ss:ms.
    /// Removes leading zero.
    /// ```
    /// let time: Time = 45_296_789.into();
    /// assert_eq!(time.format(), "12:34:56.789");
    /// ```
    pub fn format(&self) -> String {
        let sign = if self.ms < 0 { "-" } else { "" };
        let mut remaining = self.ms.abs();
        let ms = remaining % 1000;
        remaining = (remaining - ms) / 1000;
        let s = remaining % 60;
        remaining = (remaining - s) / 60;
        let m = remaining % 60;
        let h = (remaining - m) / 60;
        match (h, m, s, ms) {
            (0, 0, 0, ms) => format!("{}0.{:03}", sign, ms),
            (0, 0, s, ms) => format!("{}{}.{:03}", sign, s, ms),
            (0, m, s, ms) => format!("{}{}:{:02}.{:03}", sign, m, s, ms),
            (h, m, s, ms) => format!("{}{}:{:02}:{:02}.{:03}", sign, h, m, s, ms),
        }
    }
}

mod tests {
    #[test]
    fn format_correctly() {
        let time = crate::time::Time::from(45_296_789);
        assert_eq!(time.format(), "12:34:56.789");
    }

    #[test]
    fn format_leading_zeros() {
        let time = crate::time::Time::from(3_661_001);
        assert_eq!(time.format(), "1:01:01.001");
    }

    #[test]
    fn format_negative() {
        let time = crate::time::Time::from(-3_661_001);
        assert_eq!(time.format(), "-1:01:01.001");
    }
}
