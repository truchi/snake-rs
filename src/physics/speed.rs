use super::{Distance, Duration};
use std::{
    fmt::{Debug, Error, Formatter},
    ops::Mul,
};

/// A `Speed` type to represent a `Distance` over a `Duration`.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Speed {
    /// The `Distance`.
    distance: Distance,
    /// The `Duration`.
    duration: Duration,
}

impl Speed {
    /// Creates a new `Speed` with the specified `distance` and `duration`.
    pub fn new(distance: Distance, duration: Duration) -> Self {
        // TODO: panic if duration == 0
        Self { distance, duration }
    }

    /// Creates a new `Speed` from an `units_per_sec` speed as `u16`.
    pub fn from_units_per_sec(units_per_sec: u16) -> Self {
        Self {
            distance: Distance::from_units(units_per_sec),
            duration: Duration::from_secs(1),
        }
    }

    /// Multiplies a `Speed` by a `Duration` to produce the traveled `Distance`.
    pub fn mul_duration(self, rhs: Duration) -> Distance {
        self.distance * (rhs.as_secs_f64() / self.duration.as_secs_f64())
    }
}

impl From<(Distance, Duration)> for Speed {
    fn from((distance, duration): (Distance, Duration)) -> Self {
        Self::new(distance, duration)
    }
}

impl Into<(Distance, Duration)> for Speed {
    fn into(self) -> (Distance, Duration) {
        (self.distance, self.duration)
    }
}

impl From<u16> for Speed {
    fn from(units_per_sec: u16) -> Self {
        Self::from_units_per_sec(units_per_sec)
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
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(
            Speed::new(Distance::new(1, 200), Duration::new(2, 321)),
            Speed {
                distance: Distance::new(1, 200),
                duration: Duration::new(2, 321)
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
    fn mul_duration() {
        assert_eq!(
            Speed::new(Distance::new(2, 253), Duration::from_millis(500))
                .mul_duration(Duration::from_secs(1)),
            Distance::new(4, 506)
        );
    }
}
