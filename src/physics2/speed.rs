use super::{Distance, Duration};
use std::{
    cmp::Ordering,
    fmt::{Debug, Error, Formatter},
    ops::{Div, Mul},
};

/// A `Speed<T>` type to represent some distance type `T` over a `Duration`.
#[derive(Copy, Clone, Default)]
pub struct Speed<T> {
    /// The `T` distance.
    distance: T,
    /// The `Duration`.
    duration: Duration,
}

impl<T> Speed<T> {
    /// Creates a new `Speed` with the specified `distance` and `duration`
    /// without checking if `duration` is zero.
    pub fn new_uncheked(distance: T, duration: Duration) -> Self {
        Self { distance, duration }
    }

    /// Creates a new `Speed` with the specified `distance` and `duration`.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is zero.
    pub fn new(distance: T, duration: Duration) -> Self {
        assert!(
            duration != Duration::from_secs(0),
            "Duration cannot be zero"
        );

        Self::new_uncheked(distance, duration)
    }

    /// Creates a new `Speed` from a `per_sec` speed as `T`.
    pub fn from_per_sec(per_sec: T) -> Self {
        Self::new_uncheked(per_sec, Duration::from_secs(1))
    }

    /// Returns this `Speed` expressed as `T` per second.
    pub fn as_per_sec(&self) -> T
    where
        T: Div<Distance, Output = T> + Clone,
    {
        self.distance.clone() / self.duration.as_secs_f64()
    }
}

impl<T> From<(T, Duration)> for Speed<T> {
    fn from((distance, duration): (T, Duration)) -> Self {
        Self::new(distance, duration)
    }
}

impl<T> From<Speed<T>> for (T, Duration) {
    fn from(Speed { distance, duration }: Speed<T>) -> Self {
        (distance, duration)
    }
}

/// Multiplies a `Speed` by a `Duration` to produce the traveled distance `T`.
impl<T: Mul<Distance>> Mul<Duration> for Speed<T> {
    type Output = <T as Mul<Distance>>::Output;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.distance * (rhs.as_secs_f64() / self.duration.as_secs_f64())
    }
}

impl<T: Div<Distance, Output = T> + PartialEq + Clone> PartialEq for Speed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_per_sec() == other.as_per_sec()
    }
}

impl<T: Div<Distance, Output = T> + Eq + Clone> Eq for Speed<T> {}

impl<T: Div<Distance, Output = T> + PartialOrd + Clone> PartialOrd for Speed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_per_sec().partial_cmp(&other.as_per_sec())
    }
}

impl<T: Div<Distance, Output = T> + Ord + Clone> Ord for Speed<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_per_sec().cmp(&other.as_per_sec())
    }
}

impl<T: Debug> Debug for Speed<T> {
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
    fn from_tuple() {
        assert_eq!(
            Speed::from((19.0, Duration::new(3, 0))),
            Speed::new(19.0, Duration::new(3, 0))
        );
    }

    #[test]
    fn into_tuple() {
        assert_eq!(
            <(f32, Duration)>::from(Speed::new(29.0, Duration::new(3, 0))),
            (29.0, Duration::new(3, 0)),
        );
    }

    #[test]
    fn mul_duration() {
        assert_eq!(
            Speed::new(2.253, Duration::from_millis(500)) * Duration::from_secs(1),
            4.506
        );
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
}
