use std::fmt::Display;

/// A Time value. Represented in milliseconds.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Time {
    /// The time value as milliseconds.
    pub ms: f64,
}

impl From<i32> for Time {
    /// Convert a i32 of milliseconds to Time.
    fn from(value: i32) -> Self {
        Self { ms: value as f64 }
    }
}

impl From<f32> for Time {
    /// Convert f32 of milliseconds to Time.
    fn from(value: f32) -> Self {
        Self { ms: value as f64 }
    }
}

impl From<f64> for Time {
    /// Convert f64 of milliseconds to Time.
    fn from(value: f64) -> Self {
        Self { ms: value }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Time {
    /// Create a time from seconds.
    pub fn from_secs(secs: impl Into<f64>) -> Self {
        Self {
            ms: secs.into() * 1000.0,
        }
    }

    /// Format a time as hh:mm:ss:ms.
    /// Removes leading zero.
    /// ```
    /// let time: unified_sim_model::Time = 45_296_789.into();
    /// assert_eq!(time.format(), "12:34:56.789");
    /// ```
    pub fn format(&self) -> String {
        let sign = if self.ms < 0.0 { "-" } else { "" };
        let mut remaining = self.ms.abs().round() as i64;
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

    /// Format a time as hh:mm:ss.
    /// Removes leading zero and does not display milliseconds.
    /// Milliseconds are truncated.
    /// ```
    /// let time: unified_sim_model::Time = 45_296_789.into();
    /// assert_eq!(time.format(), "12:34:56");
    /// ```
    pub fn fmt_no_ms(&self) -> String {
        let sign = if self.ms < 0.0 { "-" } else { "" };
        let mut remaining = self.ms.abs().round() as i64;
        let ms = remaining % 1000;
        remaining = (remaining - ms) / 1000;
        let s = remaining % 60;
        remaining = (remaining - s) / 60;
        let m = remaining % 60;
        let h = (remaining - m) / 60;
        match (h, m, s) {
            (0, 0, s) => format!("{}{}", sign, s),
            (0, m, s) => format!("{}{}:{:02}", sign, m, s),
            (h, m, s) => format!("{}{}:{:02}:{:02}", sign, h, m, s),
        }
    }

    /// Format a time as hh:mm
    /// Removes leading zero and does not display milliseconds.
    /// Milliseconds are truncated.
    /// Seconds are truncated.
    /// ```
    /// let time: unified_sim_model::Time = 45_296_789.into();
    /// assert_eq!(time.format(), "12:34:56");
    /// ```
    pub fn fmt_no_s_ms(&self) -> String {
        let sign = if self.ms < 0.0 { "-" } else { "" };
        let mut remaining = self.ms.abs().round() as i64;
        let ms = remaining % 1000;
        remaining = (remaining - ms) / 1000;
        let s = remaining % 60;
        remaining = (remaining - s) / 60;
        let m = remaining % 60;
        let h = (remaining - m) / 60;
        match (h, m) {
            (0, m) => format!("{}{}", sign, m),
            (h, m) => format!("{}{}:{:02}", sign, h, m),
        }
    }

    /// Format the time as either hh:mm or mm::ss
    /// and pad with zero to a width of two
    /// ```
    /// let time: unified_sim_model::Time = 45_296_789.into();
    /// assert_eq!(time.format(), "12:34");
    /// ```
    pub fn fmt_h_m_s_at_most_two_fill_with_zero(&self) -> String {
        let sign = if self.ms < 0.0 { "-" } else { "" };
        let mut remaining = self.ms.abs().round() as i64;
        let ms = remaining % 1000;
        remaining = (remaining - ms) / 1000;
        let s = remaining % 60;
        remaining = (remaining - s) / 60;
        let m = remaining % 60;
        let h = (remaining - m) / 60;
        match (h, m, s) {
            (0, m, s) => format!("{sign}{m:02}:{s:02}"),
            (h, m, _) => format!("{sign}{h:02}:{m:02}"),
        }
    }
}

mod tests {
    #[test]
    fn format_correctly() {
        let time = crate::types::Time::from(45_296_789);
        assert_eq!(time.format(), "12:34:56.789");
    }

    #[test]
    fn format_leading_zeros() {
        let time = crate::types::Time::from(3_661_001);
        assert_eq!(time.format(), "1:01:01.001");
    }

    #[test]
    fn format_negative() {
        let time = crate::types::Time::from(-3_661_001);
        assert_eq!(time.format(), "-1:01:01.001");
    }
}
