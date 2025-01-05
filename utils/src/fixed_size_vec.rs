use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

pub struct FixedSizeVec<T, const N: usize>
where
    T: Copy,
{
    length: usize,
    data: [Option<T>; N],
}

impl<T, const N: usize> Default for FixedSizeVec<T, N>
where
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> FixedSizeVec<T, N>
where
    T: Copy,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            length: 0,
            data: [None; N],
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        if self.length == N {
            panic!("Cannot push more than {} elements", N);
        }
        self.data[self.length] = Some(value);
        self.length += 1;
    }

    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        self.data[self.length]
    }

    #[inline]
    #[must_use]
    pub fn shift(&mut self) -> Option<T> {
        let temp = self.data[0];
        for i in 0..self.length - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.data[self.length - 1] = None;
        self.length -= 1;

        temp
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.data[index].as_ref()
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        self.data[index].as_mut()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T, const N: usize> FixedSizeVec<T, N>
where
    T: Copy,
{
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|x| x.as_ref())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter_map(|x| x.as_mut())
    }
}

impl<T, const N: usize> Index<usize> for FixedSizeVec<T, N>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for FixedSizeVec<T, N>
where
    T: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T, const N: usize> Debug for FixedSizeVec<T, N>
where
    T: Debug + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.data.iter().filter(|x| x.is_some()))
            .finish()
    }
}
