#![doc = include_str!("../README.md")]
#![no_std]

use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter};

/// Encapsulates a piece of state that can be modified and
/// we want all outside code to see the edit as a single
/// atomic change.
///
/// # Trait implementations
///
/// If trait use an immutable reference ([`AsRef<T>`], [`Deref<T>`], [`Borrow<T>`]...) give access to the current value
/// and mutable references ([`AsMut<T>`], [`DerefMut<T>`], [`BorrowMut<T>`]...) give access to the next value.
///
/// # Swapping
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
        core::mem::swap(&mut self.current, &mut self.next);
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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
        self.current.eq(other)
    }
}

impl<T: PartialEq> PartialEq for DoubleBuffer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.current.eq(&other.current)
    }
}

impl<T: Eq> Eq for DoubleBuffer<T> {}

impl<T: PartialOrd> PartialOrd<T> for DoubleBuffer<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.current.partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialOrd for DoubleBuffer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.current.partial_cmp(&other.current)
    }
}

impl<T: Ord> Ord for DoubleBuffer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.current.cmp(&other.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_and_modify() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(buffer, 1);

        *buffer = 3;
        assert_eq!(buffer, 1);

        buffer.swap();
        assert_eq!(buffer, 3);
    }

    #[test]
    fn test_swap() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(buffer.current, 1);
        assert_eq!(buffer.next, 2);

        buffer.swap();
        assert_eq!(buffer.current, 2);
        assert_eq!(buffer.next, 1);
    }



        assert_eq!(buffer.current, 1);
        assert_eq!(buffer.next, 1);
    }

    #[test]
    fn test_greater_and_less_than() {
        let mut buffer: DoubleBuffer<i32> = DoubleBuffer::default();
        *buffer = 1;

        assert!(buffer > -1);
        assert!(buffer < 1);

        buffer.swap();

        assert!(buffer > 0);
        assert!(buffer < 2);
    }

    #[test]
    fn test_modify_bytes_array() {
        let mut buffer: DoubleBuffer<[u8; 3]> = DoubleBuffer::default();
        buffer[1] = 2;

        assert_eq!(buffer[1], 0);
        assert_eq!(buffer, [0, 0, 0]);

        buffer.swap();

        assert_eq!(buffer[1], 2);
        assert_eq!(buffer, [0, 2, 0]);

        assert_eq!(buffer.current, [0, 2, 0]);
        assert_eq!(buffer.next, [0, 0, 0]);
    }

    #[test]
    fn test_for_iter_mut_bytes_array() {
        let mut buffer: DoubleBuffer<[u8; 3]> = DoubleBuffer::default();
        buffer[1] = 2;

        for byte in buffer.iter_mut() {
            *byte += 1;
        }

        assert_eq!(buffer, [0, 0, 0]);

        buffer.swap();

        assert_eq!(buffer, [1, 3, 1]);
    }
}
