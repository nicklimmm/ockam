use crate::TimestampInSeconds;
use ockam_core::compat::time::Duration;

impl core::ops::Deref for TimestampInSeconds {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for TimestampInSeconds {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl core::ops::Add<TimestampInSeconds> for TimestampInSeconds {
    type Output = TimestampInSeconds;

    fn add(self, rhs: TimestampInSeconds) -> Self::Output {
        TimestampInSeconds(self.0 + rhs.0)
    }
}

impl core::ops::Add<Duration> for TimestampInSeconds {
    type Output = TimestampInSeconds;

    fn add(self, rhs: Duration) -> Self::Output {
        self + rhs.as_secs()
    }
}

impl core::ops::Add<u64> for TimestampInSeconds {
    type Output = TimestampInSeconds;

    fn add(self, rhs: u64) -> Self::Output {
        TimestampInSeconds(self.0 + rhs)
    }
}

impl core::ops::Sub<TimestampInSeconds> for TimestampInSeconds {
    type Output = TimestampInSeconds;

    fn sub(self, rhs: TimestampInSeconds) -> Self::Output {
        TimestampInSeconds(self.0 - rhs.0)
    }
}

impl TimestampInSeconds {
    /// Calculate absolute difference between the two values.
    pub fn abs_diff(self, other: Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }
}
