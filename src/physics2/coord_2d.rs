use std::fmt::{Debug, Error, Formatter};

/// A `Coord2D<T>` type to represent a position in cartesian space.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Coord2D<T> {
    /// The X axis coordinate.
    pub x: T,
    /// The Y axis coordinate.
    pub y: T,
}

impl<T> Coord2D<T> {
    /// Creates a new `Coord2D` from its `x`/`y` coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Coord2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Clone> From<T> for Coord2D<T> {
    fn from(t: T) -> Self {
        Self::new(t.clone(), t)
    }
}

macro_rules! ops {
    ($Trait:ident, $method:ident, $TraitAssign:ident, $method_assign:ident) => {
        impl<T: ::std::ops::$Trait<U>, U> ::std::ops::$Trait<Coord2D<U>> for Coord2D<T> {
            type Output = Coord2D<<T as ::std::ops::$Trait<U>>::Output>;

            fn $method(self, rhs: Coord2D<U>) -> Self::Output {
                Coord2D::new(self.x.$method(rhs.x), self.y.$method(rhs.y))
            }
        }

        impl<T: ::std::ops::$TraitAssign<U>, U> ::std::ops::$TraitAssign<Coord2D<U>>
            for Coord2D<T>
        {
            fn $method_assign(&mut self, rhs: Coord2D<U>) {
                self.x.$method_assign(rhs.x);
                self.y.$method_assign(rhs.y);
            }
        }
    };
    (unary $Trait:ident, $method:ident) => {
        impl<T: ::std::ops::$Trait> ::std::ops::$Trait for Coord2D<T> {
            type Output = Coord2D<<T as ::std::ops::$Trait>::Output>;

            fn $method(self) -> Self::Output {
                Coord2D::new(self.x.$method(), self.y.$method())
            }
        }
    };
}

ops!(Add, add, AddAssign, add_assign);
ops!(Sub, sub, SubAssign, sub_assign);
ops!(Mul, mul, MulAssign, mul_assign);
ops!(Div, div, DivAssign, div_assign);
ops!(Rem, rem, RemAssign, rem_assign);
ops!(unary Neg, neg);
ops!(unary Not, not);

impl<T: Debug> Debug for Coord2D<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(Coord2D::new(1, 4), Coord2D { x: 1, y: 4 });
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Coord2D::from((1.0, 2.0)), Coord2D::new(1.0, 2.0));
    }

    #[test]
    fn from_t() {
        assert_eq!(Coord2D::from(12), Coord2D::new(12, 12));
    }

    macro_rules! test_ops {
        (
            ($p1x:expr, $p1y:expr) [$method:ident $op:tt $method_assign:ident $op_ass:tt]
            ($p2x:expr, $p2y:expr) => ($p3x:expr, $p3y:expr)
        ) => {
            #[test]
            fn $method() {
                let point1 = Coord2D::new($p1x, $p1y);
                let point2 = Coord2D::new($p2x, $p2y);
                let point3 = Coord2D::new($p3x, $p3y);

                assert_eq!(point1 $op point2, point3);
            }

            #[test]
            fn $method_assign() {
                let point1 = Coord2D::new($p1x, $p1y);
                let point2 = Coord2D::new($p2x, $p2y);
                let point3 = Coord2D::new($p3x, $p3y);

                {
                    let mut new_point = point1;
                    new_point $op_ass point2;
                    assert_eq!(new_point, point3);
                }
            }
        };
        (unary ($p1x:expr, $p1y:expr) [$method:ident $op:tt] => ($p2x:expr, $p2y:expr)) => {
            #[test]
            fn $method() {
                let point1 = Coord2D::new($p1x, $p1y);
                let point2 = Coord2D::new($p2x, $p2y);

                assert_eq!($op point1, point2);
            }
        };
    }

    test_ops!((1, 3) [add + add_assign +=] (2, 1) => (3,  4));
    test_ops!((8, 9) [sub - sub_assign -=] (5, 3) => (3,  6));
    test_ops!((2, 3) [mul * mul_assign *=] (3, 4) => (6, 12));
    test_ops!((6, 3) [div / div_assign /=] (2, 3) => (3,  1));
    test_ops!((4, 5) [rem % rem_assign %=] (8, 2) => (4,  1));
    test_ops!(unary (18, 32) [neg -] => (-18, -32));
    test_ops!(unary (true, false) [not !] => (false, true));
}
