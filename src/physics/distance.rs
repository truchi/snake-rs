use std::{
    fmt::{Debug, Error, Formatter},
    ops::{Add, AddAssign, Mul, MulAssign},
};

/// A `Distance` type to represent a span of space.
///
/// Each `Distance` is composed of a whole number of units and a fractional part represented in
/// milliunits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Distance {
    /// The number of units.
    units: u16,
    /// The number of milliunits.
    millis: u16,
}

impl Distance {
    /// Creates a new `Distance` from `units` and `millis`.
    ///
    /// # Panics
    ///
    /// Panics if `millis >= 1_000`.
    pub fn new(units: u16, millis: u16) -> Self {
        debug_assert!(millis < 1_000);
        Self { units, millis }
    }

    /// Creates a new `Distance` from `units`.
    pub fn from_units(units: u16) -> Self {
        Self { units, millis: 0 }
    }

    /// Creates a new `Distance` from `units` as `f64`.
    pub fn from_units_f64(units: f64) -> Self {
        Self {
            units: units as u16,
            millis: ((units * 1_000.0) as u128 % 1_000) as u16,
        }
    }

    /// Returns the number of *whole* units in this `Distance`.
    ///
    /// Does not include the fractional (millis) part.
    pub fn as_units(self) -> u16 {
        self.units
    }

    /// Returns the number of units as `f64` in this `Distance`.
    ///
    /// Includes the fractional (millis) part.
    pub fn as_units_f64(self) -> f64 {
        self.units as f64 + self.millis as f64 / 1_000.0
    }

    /// Adds two `Distance` together.
    pub fn add(self, rhs: Self) -> Self {
        let mut units = self.units + rhs.units;
        let mut millis = self.millis + rhs.millis;

        if millis >= 1_000 {
            units += 1;
            millis -= 1_000;
        }

        debug_assert!(millis < 1_000);
        Self { units, millis }
    }

    /// Multiplies a `Distance` by `f64`.
    pub fn mul_f64(self, rhs: f64) -> Self {
        Self::from_units_f64(rhs * self.as_units_f64())
    }
}

impl From<(u16, u16)> for Distance {
    fn from((units, millis): (u16, u16)) -> Self {
        Self::new(units, millis)
    }
}

impl Into<(u16, u16)> for Distance {
    fn into(self) -> (u16, u16) {
        (self.units, self.millis)
    }
}

impl From<u16> for Distance {
    fn from(units: u16) -> Self {
        Self::from_units(units)
    }
}

impl Into<u16> for Distance {
    fn into(self) -> u16 {
        self.as_units()
    }
}

impl From<f64> for Distance {
    fn from(units: f64) -> Self {
        Self::from_units_f64(units)
    }
}

impl Into<f64> for Distance {
    fn into(self) -> f64 {
        self.as_units_f64()
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Mul<f64> for Distance {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.mul_f64(rhs)
    }
}

impl MulAssign<f64> for Distance {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.mul_f64(rhs);
    }
}

impl Debug for Distance {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.units == 0 {
            if self.millis == 0 {
                write!(f, "0u")
            } else {
                write!(f, "{}mu", self.millis)
            }
        } else {
            if self.millis == 0 {
                write!(f, "{}u", self.units)
            } else {
                write!(f, "{}.{}u", self.units, self.millis)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(
            Distance::new(12, 99),
            Distance {
                units: 12,
                millis: 99
            }
        );
    }

    #[test]
    #[should_panic]
    fn new_1_000_millis_panic() {
        Distance::new(0, 1_000);
    }

    #[test]
    fn from_units() {
        assert_eq!(Distance::from_units(3), Distance::new(3, 0));
    }

    #[test]
    fn from_units_f64() {
        assert_eq!(Distance::from_units_f64(4.56), Distance::new(4, 560));
    }

    #[test]
    fn as_units() {
        assert_eq!(Distance::new(1, 289).as_units(), 1);
    }

    #[test]
    fn as_units_f64() {
        assert_eq!(Distance::new(8, 374).as_units_f64(), 8.374);
    }

    #[test]
    fn add() {
        assert_eq!(
            Distance::new(5, 33).add(Distance::new(1, 290)),
            Distance::new(6, 323)
        );
    }

    #[test]
    fn mul_f64() {
        assert_eq!(Distance::new(2, 500).mul(1.5), Distance::new(3, 750));
    }
}
