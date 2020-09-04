use super::{Moving, PathFragment, Point};
use std::time::Duration;

/// A `Path` type to represent the progress along a sequence of `PathFragment`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Path<T: Iterator<Item = PathFragment>> {
    /// The current position.
    pub position: Point,
    /// The `PathFragment` iterator.
    fragments:    T,
    /// The current `PathFragment`.
    current:      Option<PathFragment>,
    /// The elapsed `Duration` since the beginning of the current fragment.
    progress:     Duration,
}

impl<T: Iterator<Item = PathFragment>> Path<T> {
    /// Creates a new `Path` at the specified position with the given
    /// `PathFragment` iterator.
    pub fn new(mut fragments: T) -> Self {
        let position = Point::new(0.0, 0.0);
        let current = fragments.next();
        let progress = Duration::from_secs(0);

        Self {
            position,
            fragments,
            current,
            progress,
        }
    }
}

impl<T: Iterator<Item = PathFragment>> Moving for Path<T> {
    fn r#move(&mut self, mut duration: Duration) {
        loop {
            if let Some(fragment) = &mut self.current {
                // Progress on the fragment
                self.progress += duration;
                let extra = self.progress.checked_sub(fragment.duration);
                let (dur, extra) = if let Some(extra) = extra {
                    (duration - extra, extra)
                } else {
                    (duration, Duration::from_secs(0))
                };

                // Update position
                (&mut self.position, fragment.speed).r#move(dur);

                if extra > Duration::from_secs(0) {
                    // Restart with the next fragment and extra duration
                    self.current = self.fragments.next();
                    self.progress = Duration::from_secs(0);
                    duration = extra;
                } else {
                    // No extra duration, we are done
                    break;
                }
            } else {
                // No more fragments, we are done
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn fragments_data() -> [PathFragment; 4] {
        [
            PathFragment::from((Duration::from_secs(3), (3.0, 0.0))),
            PathFragment::from((Duration::from_secs(3), (0.0, 3.0))),
            PathFragment::from((Duration::from_secs(3), (-3.0, 0.0))),
            PathFragment::from((Duration::from_secs(3), (0.0, -3.0))),
        ]
    }

    #[test]
    fn new() {
        let fragments = fragments_data();
        let path = Path::new(fragments.iter().cloned());

        assert_eq!(path.position, Point::new(0.0, 0.0));
        assert_eq!(path.current, Some(fragments[0]));
        // NOTE: std::iter::Cloned does not impl PartialEq, ...
    }

    #[test]
    fn r#move() {
        let fragments = fragments_data();
        let mut path = Path::new(fragments.iter().cloned());

        path.r#move(Duration::from_secs(1)); // 1s, going left by 1
        assert_eq!(path.position, Point::new(1.0, 0.0));
        path.r#move(Duration::from_secs(1)); // 2s, going left by 1
        assert_eq!(path.position, Point::new(2.0, 0.0));
        path.r#move(Duration::from_secs(1)); // 3s, going left by 1
        assert_eq!(path.position, Point::new(3.0, 0.0));
        path.r#move(Duration::from_secs(1)); // 4s, going down by 1
        assert_eq!(path.position, Point::new(3.0, 1.0));
        path.r#move(Duration::from_secs(3)); // 7s, going down by 2 then right by 1
        assert_eq!(path.position, Point::new(2.0, 3.0));
        path.r#move(Duration::from_secs(3)); // 10s, going right by 2 then up by 1
        assert_eq!(path.position, Point::new(0.0, 2.0));
        path.r#move(Duration::from_secs(1)); // 11s, going up by 1
        assert_eq!(path.position, Point::new(0.0, 1.0));
        path.r#move(Duration::from_secs(1)); // 12s, going up by 1
        assert_eq!(path.position, Point::new(0.0, 0.0));
        path.r#move(Duration::from_secs(1)); // 13s, done
        assert_eq!(path.position, Point::new(0.0, 0.0));
    }
}
