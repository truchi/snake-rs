use super::{Distance, Point, Speed2D};
use std::time::Duration;

/// A `PathFragment` type to represent the progress along a straight motion at
/// some `Speed2D` for some `Duration`.
#[derive(Copy, Clone, Debug)]
pub struct PathFragment {
    /// The `Speed2D` of the motion.
    pub speed: Speed2D,
    /// The `Duration` of the `PathFragment`.
    duration:  Duration,
    /// The elapsed `Duration` since the beginning of the motion.
    progress:  Duration,
}

impl PathFragment {
    /// Creates a new `PathFragment` from a `Speed2D` and a `Distance`.
    pub fn from_speed_and_distance(speed: impl Into<Speed2D>, distance: Distance) -> Self {
        let speed = speed.into();

        Self {
            speed,
            duration: Duration::from_secs_f64(distance / speed.as_per_sec().length()),
            progress: Duration::from_secs(0),
        }
    }

    /// Creates a new `PathFragment` from a `Speed2D` and a `Duration`.
    pub fn from_speed_and_duration(speed: impl Into<Speed2D>, duration: Duration) -> Self {
        let speed = speed.into();

        Self {
            speed,
            duration,
            progress: Duration::from_secs(0),
        }
    }

    /// Creates a new `PathFragment` from a `Duration` and a `Point`.
    pub fn from_duration_and_point(duration: Duration, point: impl Into<Point>) -> Self {
        let point = point.into();

        Self {
            speed: Speed2D::new((point.x, duration).into(), (point.y, duration).into()),
            duration,
            progress: Duration::from_secs(0),
        }
    }

    /// Progresses the motion by a `Duration` and returns both the elapsed
    /// `Duration` effectively spent in this motion and the extra `Duration`, if
    /// any.
    ///
    /// # Guaranties
    ///
    /// `extra != 0`
    ///
    /// `duration == elapsed + extra`
    pub fn progress(&mut self, duration: Duration) -> (Duration, Option<Duration>) {
        self.progress += duration;

        let (elapsed, extra) = if let Some(extra) = self.progress.checked_sub(self.duration) {
            if extra == Duration::from_secs(0) {
                (duration, None)
            } else {
                self.reset();
                (duration - extra, Some(extra))
            }
        } else {
            (duration, None)
        };

        // extra != 0
        debug_assert!(extra.map_or(true, |extra| extra != Duration::from_secs(0)));
        // duration == elapsed + extra
        debug_assert!(duration == elapsed + extra.unwrap_or(Duration::from_secs(0)));

        (elapsed, extra)
    }

    /// Resets the progress.
    pub fn reset(&mut self) {
        self.progress = Duration::from_secs(0);
    }
}

impl<T: Into<Speed2D>> From<(T, Distance)> for PathFragment {
    fn from((speed, distance): (T, Distance)) -> Self {
        Self::from_speed_and_distance(speed, distance)
    }
}

impl<T: Into<Speed2D>> From<(T, Duration)> for PathFragment {
    fn from((speed, duration): (T, Duration)) -> Self {
        Self::from_speed_and_duration(speed, duration)
    }
}

impl<T: Into<Point>> From<(Duration, T)> for PathFragment {
    fn from((duration, point): (Duration, T)) -> Self {
        Self::from_duration_and_point(duration, point)
    }
}

impl PartialEq for PathFragment {
    fn eq(&self, rhs: &Self) -> bool {
        self.progress == rhs.progress
            && self.speed * self.duration.into() == rhs.speed * rhs.duration.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_speed_and_distance() {
        use super::super::Speed;

        // 3/4/5 right triangle
        let speed = Speed2D::new(Speed::from_per_sec(3.0), Speed::from_per_sec(4.0));
        let distance = 5.0;
        let duration = Duration::from_secs(1);

        assert_eq!(PathFragment::from((speed, distance)), PathFragment {
            speed,
            duration,
            progress: Duration::from_secs(0)
        });
        assert_eq!(
            PathFragment::from((speed, distance)),
            PathFragment::from_speed_and_distance(speed, distance)
        );
    }

    #[test]
    fn from_speed_and_duration() {
        use super::super::Speed;

        let speed = Speed2D::new(Speed::from_per_sec(10.0), Speed::from_per_sec(8.0));
        let duration = Duration::new(3, 0);

        assert_eq!(PathFragment::from((speed, duration)), PathFragment {
            speed,
            duration,
            progress: Duration::from_secs(0)
        });
        assert_eq!(
            PathFragment::from((speed, duration)),
            PathFragment::from_speed_and_duration(speed, duration)
        );
    }

    #[test]
    fn from_duration_and_point() {
        use super::super::Speed;

        let point: Point = (15.0, 9.0).into();
        let duration = Duration::new(3, 0);
        let speed = Speed2D::new(Speed::from_per_sec(5.0), Speed::from_per_sec(3.0));

        assert_eq!(PathFragment::from((duration, point)), PathFragment {
            speed,
            duration,
            progress: Duration::from_secs(0)
        });
        assert_eq!(
            PathFragment::from((duration, point)),
            PathFragment::from_duration_and_point(duration, point)
        );
    }

    #[test]
    fn progress() {
        use super::super::Speed;

        let speed = Speed2D::new(Speed::from_per_sec(10.0), Speed::from_per_sec(8.0));
        let duration = Duration::new(3, 0);
        let mut fragment = PathFragment::from((speed, duration));

        // Nothing at first when given nothing
        assert_eq!(
            fragment.progress(Duration::from_secs(0)),
            (Duration::from_secs(0), None)
        );
        // Progress by 1s
        assert_eq!(
            fragment.progress(Duration::from_secs(1)),
            (Duration::from_secs(1), None)
        );
        // Progress by 3s: until the end by 2s with 1s of extra
        assert_eq!(
            fragment.progress(Duration::from_secs(3)),
            (Duration::from_secs(2), Some(Duration::from_secs(1)))
        );
        // Make sure it is in the initial state regardless of the previous extra
        assert_eq!(fragment.progress(duration), (duration, None));
    }

    #[test]
    fn eq() {
        use super::super::Speed;

        assert_eq!(
            PathFragment::from_speed_and_duration(
                (Speed::from_per_sec(12.0), Speed::from_per_sec(6.0)),
                Duration::from_secs(1)
            ),
            PathFragment::from_speed_and_duration(
                (Speed::from_per_sec(6.0), Speed::from_per_sec(3.0)),
                Duration::from_secs(2)
            ),
        );
    }
}
