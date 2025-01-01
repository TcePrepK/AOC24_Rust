//! Heavily inspired by https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/point.rs
//!
//! This module provides a `Point` struct and various constants and methods for working with points.
//! The `Point` struct is a simple struct that represents a vector in two-dimensional space.
//! `Point` implements the `Add` and `Sub` traits, allowing you to add and subtract points.
//! Additional information about the `Point` struct can be found in the module-level documentation.

use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const RIGHT: Point = Point::new(1, 0);
pub const BOTTOM: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const TOP: Point = Point::new(0, -1);
pub const DIRECTIONS: [Point; 4] = [RIGHT, BOTTOM, LEFT, TOP];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Implements the `new` method for `Point`.
/// This allows us to create a new `Point` object using the `Point::new` function.
/// For example,
/// ```
/// use utils::Point;
///
/// let point = Point::new(1, 2);
/// assert_eq!(point.x, 1);
/// assert_eq!(point.y, 2);
/// ```
///
/// Implements the `cw` and `ccw` methods for `Point`.
/// This allows us to rotate a `Point` object 90 degrees clockwise or counter-clockwise.
/// For example,
/// ```
/// use utils::Point;
///
/// let p1 = Point::new(1, 0);
/// let p2 = p1.cw();
/// assert_eq!(p2, Point::new(0, 1));
///
/// let p3 = Point::new(0, -1);
/// let p4 = p3.ccw();
/// assert_eq!(p4, Point::new(1, 0));
/// ```
impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn cw(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn ccw(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

/// Implements the `Add` and `Sub` traits for `Point`.
/// This allows us to use the `+` and `-` operators with `Point` objects.
/// For example,
/// ```
/// use utils::Point;
///
/// let p1 = Point::new(1, 2);
/// let p2 = Point::new(3, 4);
/// let p3 = p1 + p2;
/// assert_eq!(p3, Point::new(4, 6));
///
/// let mut p4 = Point::new(5, 6);
/// p4 -= p2;
/// assert_eq!(p4, Point::new(2, 2));
/// ```
impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
