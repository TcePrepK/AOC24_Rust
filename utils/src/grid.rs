//! Heavily inspired by https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/grid.rs
//!
//! This module provides a `Grid` struct and various methods for working with grids.
//! The `Grid` struct is a simple struct that represents a two-dimensional grid of values.
//! The `Grid` struct implements the `Index` and `IndexMut` traits, allowing you to access and modify values in the grid.
//! Additional information about the `Grid` struct can be found in the module-level documentation.

use crate::point::Point;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid<T> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<T>,
}

/// Implements several methods for creating a new `Grid` object.
/// `Grid::new` creates a new `Grid` object with the specified width, height, and fill value.
/// `Grid::clone_with` creates a new `Grid` object with the same width and height as the original `Grid`, but with the specified fill value and type.
/// `Grid::parse` creates a new `Grid` object from a string input [AdventOfCode Special].
/// For example,
/// ```
/// use utils::Grid;
///
/// let grid = Grid::new(3, 3, 0);
/// assert_eq!(grid.width, 3);
/// assert_eq!(grid.height, 3);
/// assert_eq!(grid.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
///
/// let grid2 = grid.clone_with::<u8>(b'a');
/// assert_eq!(grid2.width, 3);
/// assert_eq!(grid2.height, 3);
/// assert_eq!(grid2.data, vec![b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a']);
///
/// let grid3 = Grid::parse("123\n456\n789");
/// assert_eq!(grid3.width, 3);
/// assert_eq!(grid3.height, 3);    
/// assert_eq!(grid3.data, vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']);
/// ```
impl<T: Copy> Grid<T> {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32, fill_value: T) -> Self {
        Grid {
            width,
            height,
            data: vec![fill_value; (width * height) as usize],
        }
    }

    #[inline]
    #[must_use]
    pub fn clone_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: vec![value; (self.width * self.height) as usize],
        }
    }
}

impl Grid<u8> {
    #[inline]
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let iter = input.par_lines().map(str::as_bytes).collect::<Vec<&[u8]>>();

        let height = iter.len() as u32;
        let width = iter[0].len() as u32;
        let mut data = Vec::with_capacity((height * width) as usize);
        iter.iter().for_each(|&line| data.extend(line));

        Grid {
            width,
            height,
            data,
        }
    }
}

/// Implements the `Index` and `IndexMut` traits for `Grid`.
/// The `Index` trait allows you to access values in the grid using points just like indices.
/// The `IndexMut` trait allows you to modify values in the grid using points just like indices.
/// For example,
/// ```
/// use utils::{Grid, Point};
///
/// let grid = Grid::new(3, 3, 0);
/// assert_eq!(grid[Point::new(0, 0)], 0);
/// assert_eq!(grid[Point::new(1, 1)], 0);
///
/// grid[Point::new(0, 0)] = 1;
/// grid[Point::new(1, 1)] = 2;
/// assert_eq!(grid[Point::new(0, 0)], 1);
/// assert_eq!(grid[Point::new(1, 1)], 2);
/// ```
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[(index.y * self.width as i32 + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[(index.y * self.width as i32 + index.x) as usize]
    }
}

/// Implements the `find_all` and `find` methods for `Grid`.
/// The `find_all` method returns a vector of all points in the grid that have the specified value.
/// The `find` method returns the first point in the grid that has the specified value, or `None` if no points have the specified value.
/// For example,
/// ```
/// use utils::{Grid, Point};
///
/// let grid = Grid::parse("....\n.S..\n....\n..S.");
/// assert_eq!(grid.find_all(b'S'), vec![Point::new(1, 1), Point::new(3, 3)]);
/// assert_eq!(grid.find(b'S'), Some(Point::new(1, 1)));
/// assert_eq!(grid.find(b'E'), None);
/// ```
impl<T: Copy + PartialEq + Sync> Grid<T> {
    #[inline]
    #[must_use]
    pub fn find_all(&self, value: T) -> Vec<Point> {
        self.data
            .par_iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if v == value {
                    Some(Point::new(
                        i as i32 % self.width as i32,
                        i as i32 / self.width as i32,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find(&self, value: T) -> Option<Point> {
        self.find_all(value).pop()
    }
}
