#![doc = include_str!("../README.md")]

use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter};

/// Encapsulates a piece of state that can be modified and
/// we want all outside code to see the edit as a single
/// atomic change.
///
/// There are two ways to swap:
///
/// 1. [`DoubleBuffer::swap()`] - when swapping, the next value will have the previous current value.
/// 2. [`DoubleBuffer::swap_cloning()`] - when swapping, the next value will keep same and will be cloned to the current value.
///
/// You can read about the two ways [how the buffers are swapped](https://gameprogrammingpatterns.com/double-buffer.html#how-are-the-buffers-swapped)
/// in "Game Programming Patterns" by Robert Nystrom.
///
/// # Examples
///
/// ```
/// use double_buffer::DoubleBuffer;
///
/// let mut buffer: DoubleBuffer<u32> = DoubleBuffer::default();
/// *buffer = 1;
///
/// assert_eq!(buffer, 0);
/// buffer.swap();
/// assert_eq!(buffer, 1);
/// ```
pub struct DoubleBuffer<T> {
    current: T,
    next: T,
}

impl<T> DoubleBuffer<T> {
    #[inline]
    pub fn new(current: T, next: T) -> Self {
        Self { current, next }
    }

    /// Swaps the current and next values,
    /// then writes will be over the previous current value.
    #[inline]
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }
}

impl<T: Clone> DoubleBuffer<T> {
    /// Swaps buffers cloning the next value to the current value,
    /// then writes will continue over the same next value.
    #[inline]
    pub fn swap_cloning(&mut self) {
        self.current = self.next.clone();
    }
}

impl<T: Debug> Debug for DoubleBuffer<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoubleBuffer")
            .field("current", &self.current)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Default> Default for DoubleBuffer<T> {
    #[inline]
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T> Deref for DoubleBuffer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<T> DerefMut for DoubleBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.next
    }
}

impl<T> Borrow<T> for DoubleBuffer<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.current
    }
}

impl<T> BorrowMut<T> for DoubleBuffer<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.next
    }
}

impl<T> AsRef<T> for DoubleBuffer<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.current
    }
}

impl<T> AsMut<T> for DoubleBuffer<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.next
    }
}

impl<T: PartialEq> PartialEq<T> for DoubleBuffer<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.current == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for DoubleBuffer<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_format() {
        let buffer: DoubleBuffer<u32> = DoubleBuffer::default();
        assert_eq!(format!("{:?}", buffer), "DoubleBuffer { current: 0, next: 0 }");
    }

    #[test]
    fn test_modify_and_swap() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::default();
        *buffer = 1;

        assert_eq!(buffer, 0);

        buffer.swap();
        assert_eq!(buffer, 1);

        assert_eq!(buffer.current, 1);
        assert_eq!(buffer.next, 0);
    }

    #[test]
    fn test_modify_and_swap_cloning() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::default();
        *buffer = 1;

        assert_eq!(buffer, 0);

        buffer.swap_cloning();
        assert_eq!(buffer, 1);

        assert_eq!(buffer.current, 1);
        assert_eq!(buffer.next, 1);
    }
}
