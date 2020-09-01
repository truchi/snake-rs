use super::{Distance, Duration};
use std::{
    cmp::Ordering,
    fmt::{Debug, Error, Formatter},
    ops::{Mul, Neg},
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
    /// Creates a new `Speed` with the specified `distance` and `duration`.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is zero.
    pub fn new(distance: impl Into<Distance>, duration: impl Into<Duration>) -> Self {
        let duration = duration.into();
        if duration == Duration::from_secs(0) {
            panic!("Duration cannot be zero");
        }

        Self {
            distance: distance.into(),
            duration,
        }
    }

    /// Creates a new `Speed` from an `units_per_sec` speed as `u16`.
    pub fn from_units_per_sec(units_per_sec: i16) -> Self {
        Self {
            distance: Distance::from_units(units_per_sec),
            duration: Duration::from_secs(1),
        }
    }

    /// Returns a `Speed` as units per second.
    pub fn as_units_per_sec(&self) -> f64 {
        self.distance.as_units_f64() / self.duration.as_secs_f64()
    }

    /// Changes the sign of the `Speed`
    pub fn neg(&self) -> Self {
        Self {
            distance: -self.distance,
            duration: self.duration,
        }
    }

    /// Multiplies a `Speed` by a `Duration` to produce the traveled `Distance`.
    pub fn mul_duration(&self, rhs: impl Into<Duration>) -> Distance {
        self.distance * (rhs.into().as_secs_f64() / self.duration.as_secs_f64())
    }
}

impl PartialEq for Speed {
    fn eq(&self, other: &Self) -> bool {
        self.as_units_per_sec() == other.as_units_per_sec()
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
        let a = self.as_units_per_sec();
        let b = other.as_units_per_sec();

        if a < b {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<T: Into<Distance>, U: Into<Duration>> From<(T, U)> for Speed {
    fn from((distance, duration): (T, U)) -> Self {
        Self::new(distance.into(), duration.into())
    }
}

impl<T: From<Distance>, U: From<Duration>> From<Speed> for (T, U) {
    fn from(speed: Speed) -> Self {
        (speed.distance.into(), speed.duration.into())
    }
}

impl From<i16> for Speed {
    fn from(units_per_sec: i16) -> Self {
        Self::from_units_per_sec(units_per_sec)
    }
}

impl From<Speed> for f64 {
    fn from(speed: Speed) -> Self {
        speed.as_units_per_sec()
    }
}

impl Neg for Speed {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
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
        Speed::new(Distance::new(1, 200), Duration::new(0, 0));
    }

    #[test]
    fn new() {
        assert_eq!(
            Speed::new(Distance::new(1, 200), Duration::new(2, 321)),
            Speed {
                distance: Distance::new(1, 200),
                duration: Duration::new(2, 321),
            }
        );
    }

    #[test]
    fn from_units_per_sec() {
        assert_eq!(
            Speed::from_units_per_sec(14),
            Speed::new(Distance::new(14, 0), Duration::new(1, 0))
        );
    }

    #[test]
    fn as_units_per_sec() {
        assert_eq!(
            Speed::new(Distance::new(9, 200), Duration::new(2, 0)).as_units_per_sec(),
            4.6
        );
    }

    #[test]
    fn eq() {
        assert_eq!(
            Speed::new(Distance::new(1, 0), Duration::new(1, 0)),
            Speed::new(Distance::new(60, 0), Duration::new(60, 0)),
        );
    }

    #[test]
    fn cmp() {
        assert_eq!(
            Speed::new(Distance::new(1, 0), Duration::new(1, 0))
                > Speed::new(Distance::new(2, 0), Duration::new(60, 0)),
            true
        );
    }

    #[test]
    fn neg() {
        assert_eq!(
            Speed::new(Distance::new(7, 2), Duration::from_millis(15)).neg(),
            Speed::new(Distance::new(-7, 2), Duration::from_millis(15)),
        );
    }

    #[test]
    fn mul_duration() {
        assert_eq!(
            Speed::new(Distance::new(2, 253), Duration::from_millis(500))
                .mul_duration(Duration::from_secs(1)),
            Distance::new(4, 506)
        );
    }
}
