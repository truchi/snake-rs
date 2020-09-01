use super::{Distance, Duration};
use std::{
    cmp::Ordering,
    fmt::{Debug, Error, Formatter},
    ops::Mul,
};

/// A `Speed` type to represent a `Distance` over a `Duration`.
#[derive(Copy, Clone, Default)]
pub struct Speed {
    /// The `Distance`.
    distance: Distance,
    /// The `Duration`.
    duration: Duration,
}

impl Speed {
    /// Creates a new `Speed` with the specified `distance` and `duration`
    /// without checking if `duration` is zero.
    pub fn new_uncheked(distance: impl Into<Distance>, duration: Duration) -> Self {
        Self {
            distance: distance.into(),
            duration,
        }
    }

    /// Creates a new `Speed` with the specified `distance` and `duration`.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is zero.
    pub fn new(distance: impl Into<Distance>, duration: Duration) -> Self {
        assert!(
            duration != Duration::from_secs(0),
            "Duration cannot be zero"
        );

        Self::new_uncheked(distance.into(), duration)
    }

    /// Creates a new `Speed` from a `per_sec` speed as `Distance`.
    pub fn from_per_sec(per_sec: Distance) -> Self {
        Self::new_uncheked(per_sec, Duration::from_secs(1))
    }

    /// Returns a `Speed` as `Distance` per second.
    pub fn as_per_sec(&self) -> Distance {
        self.distance / self.duration.as_secs_f64()
    }

    /// Multiplies a `Speed` by a `Duration` to produce the traveled `Distance`.
    pub fn mul_duration(&self, rhs: Duration) -> Distance {
        self.distance * (rhs.as_secs_f64() / self.duration.as_secs_f64())
    }
}

impl PartialEq for Speed {
    fn eq(&self, other: &Self) -> bool {
        self.as_per_sec() == other.as_per_sec()
    }
}

impl Eq for Speed {}

impl PartialOrd for Speed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Speed {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.as_per_sec();
        let b = other.as_per_sec();

        if a < b {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<T: Into<Distance>> From<(T, Duration)> for Speed {
    fn from((distance, duration): (T, Duration)) -> Self {
        Self::new(distance.into(), duration)
    }
}

impl<T: From<Distance>> From<Speed> for (T, Duration) {
    fn from(speed: Speed) -> Self {
        (speed.distance.into(), speed.duration)
    }
}

impl From<Distance> for Speed {
    fn from(per_sec: Distance) -> Self {
        Self::from_per_sec(per_sec)
    }
}

impl From<Speed> for Distance {
    fn from(speed: Speed) -> Self {
        speed.as_per_sec()
    }
}

impl Mul<Duration> for Speed {
    type Output = Distance;

    fn mul(self, rhs: Duration) -> Distance {
        self.mul_duration(rhs)
    }
}

impl Debug for Speed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}/{:?}", self.distance, self.duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn new_duration_0_panic() {
        Speed::new(1.200, Duration::new(0, 0));
    }

    #[test]
    fn new() {
        assert_eq!(Speed::new(1.200, Duration::new(2, 321)), Speed {
            distance: 1.200,
            duration: Duration::new(2, 321),
        });
    }

    #[test]
    fn from_per_sec() {
        assert_eq!(
            Speed::from_per_sec(14.0),
            Speed::new(14.0, Duration::new(1, 0))
        );
    }

    #[test]
    fn as_per_sec() {
        assert_eq!(Speed::new(9.200, Duration::new(2, 0)).as_per_sec(), 4.6);
    }

    #[test]
    fn eq() {
        assert_eq!(
            Speed::new(1.0, Duration::new(1, 0)),
            Speed::new(60.0, Duration::new(60, 0)),
        );
    }

    #[test]
    fn cmp() {
        assert_eq!(
            Speed::new(1.0, Duration::new(1, 0)) > Speed::new(2.0, Duration::new(60, 0)),
            true
        );
    }

    #[test]
    fn mul_duration() {
        assert_eq!(
            Speed::new(2.253, Duration::from_millis(500)).mul_duration(Duration::from_secs(1)),
            4.506
        );
    }
}
