#![doc = include_str!("../README.md")]
#![no_std]

use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Pointer};

enum State {
    First,
    Second,
}

/// Encapsulates a piece of state that can be modified and
/// we want all outside code to see the edit as a single
/// atomic change.
///
/// # Trait implementations
///
/// If trait use an immutable reference ([`AsRef<T>`], [`Deref`], [`Borrow<T>`]...) give access to the current value
/// and mutable references ([`AsMut<T>`], [`DerefMut`], [`BorrowMut<T>`]...) give access to the next value.
///
/// # Swapping
///
/// There are three ways to swap:
///
/// 1. [`DoubleBuffer::swap()`] - when swapping, the next value will have the previous current value.
/// 2. [`DoubleBuffer::swap_with_clone()`] - when swapping, the next value will keep same and will be cloned to the current value.
/// 3. [`DoubleBuffer::swap_with_default()`] - like [`DoubleBuffer::swap()`] but the next value will be set to the default value of the type.
///
/// Note that for the third way, the type must implement [`Default`].
///
/// You can read about the two ways [how the buffers are swapped](https://gameprogrammingpatterns.com/double-buffer.html#how-are-the-buffers-swapped)
/// in "Game Programming Patterns" by Robert Nystrom.
///
/// ## Swapping Benchmarks
///
/// The following are the results in a i7 10th gen with 32GB RAM for a `vec![u8; 8192]` buffer:
///
/// 1. [`DoubleBuffer::swap()`] - 1.6655 ns 1.6814 ns 1.6964 ns
/// 2. [`DoubleBuffer::swap_with_default()`] - 1.7783 ns 1.8009 ns 1.8262 ns
/// 3. [`DoubleBuffer::swap_with_clone()`] - 174.18 ns 177.86 ns 182.07 ns
///
/// If it's not important to keep the pointer address of the current value unchanged,
/// [`DoubleBuffer::swap()`] is the best option, or [`DoubleBuffer::swap_with_default()`]
/// if the type implements [`Default`] and starts with the default value is important.
///
/// Only use [`DoubleBuffer::swap_with_clone()`] if it's important to keep the pointer
/// address of the current value unchanged.
///
/// # Examples
///
/// The following example shows how the buffer is swapped with the three ways:
///
/// ```
/// # use double_buffer::DoubleBuffer;
/// let mut buffer: DoubleBuffer<[u8; 32]> = DoubleBuffer::default();
/// print!("{:?}", buffer); // DoubleBuffer { current: [0, ...], next: [0, ...] }
///
/// buffer[0] = 1;
/// print!("{:?}", buffer); // DoubleBuffer { current: [0, ...], next: [1, ...] }
///
/// buffer.swap();
/// print!("{:?}", buffer); // DoubleBuffer { current: [1, ...], next: [0, ...] }
///
/// buffer[0] = 2;
/// print!("{:?}", buffer); // DoubleBuffer { current: [1, ...], next: [2, ...] }
///
/// buffer.swap_with_clone();
/// print!("{:?}", buffer); // DoubleBuffer { current: [2, ...], next: [2, ...] }
///
/// buffer[0] = 3;
/// print!("{:?}", buffer); // DoubleBuffer { current: [2, ...], next: [3, ...] }
///
/// buffer.swap_with_default();
/// print!("{:?}", buffer); // DoubleBuffer { current: [3, ...], next: [0, ...] }
/// ```
pub struct DoubleBuffer<T> {
    state: State,
    buffers: [T; 2],
}

impl<T> DoubleBuffer<T> {
    #[inline]
    pub const fn new(current: T, next: T) -> Self {
        let state = State::First;
        let buffers = [current, next];
        Self { state, buffers }
    }

    /// Swaps the current and next values,
    /// then writes will be over the previous current value.
    ///
    /// This changes the pointer address of the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use double_buffer::DoubleBuffer;
    /// let mut buffer: DoubleBuffer<[u8; 8192]> = DoubleBuffer::new([0; 8192], [0; 8192]);
    /// let first_address = format!("{:p}", buffer);
    /// buffer.swap();
    /// let second_address = format!("{:p}", buffer);
    /// // The addresses are different.
    /// assert_ne!(first_address, second_address);
    /// ```
    #[inline]
    pub fn swap(&mut self) {
        self.state = match &self.state {
            State::First => State::Second,
            State::Second => State::First,
        };
    }

    #[inline]
    const fn current(&self) -> &T {
        match self.state {
            State::First => &self.buffers[0],
            State::Second => &self.buffers[1],
        }
    }

    #[inline]
    const fn next(&self) -> &T {
        match self.state {
            State::First => &self.buffers[1],
            State::Second => &self.buffers[0],
        }
    }

    #[inline]
    fn current_mut(&mut self) -> &mut T {
        match self.state {
            State::First => &mut self.buffers[0],
            State::Second => &mut self.buffers[1],
        }
    }

    #[inline]
    fn next_mut(&mut self) -> &mut T {
        match self.state {
            State::First => &mut self.buffers[1],
            State::Second => &mut self.buffers[0],
        }
    }
}

impl<T: Clone> DoubleBuffer<T> {
    /// Clone the next value to the current value,
    /// then writes will continue over the same next value.
    ///
    /// This let the pointer address of the current value unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # use double_buffer::DoubleBuffer;
    /// let mut buffer: DoubleBuffer<[u8; 8192]> = DoubleBuffer::new([0; 8192], [0; 8192]);
    /// let first_address = format!("{:p}", buffer);
    /// buffer.swap_with_clone();
    /// let second_address = format!("{:p}", buffer);
    /// // The addresses are different.
    /// assert_eq!(first_address, second_address);
    /// ```
    #[inline]
    pub fn swap_with_clone(&mut self) {
        let next = self.next().clone();
        let current = self.current_mut();
        *current = next;
    }
}

impl<T: Default> DoubleBuffer<T> {
    /// Swaps buffers like [`DoubleBuffer::swap()`] and sets the next
    /// value to the default value of the type, then writes will be
    /// over the default value.
    #[inline]
    pub fn swap_with_default(&mut self) {
        self.swap();
        let next = self.next_mut();
        *next = T::default();
    }
}

impl<T: Debug> Debug for DoubleBuffer<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DoubleBuffer")
            .field("current", self.current())
            .field("next", self.next())
            .finish()
    }
}

impl<T> Pointer for DoubleBuffer<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:p}", self.current())
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
        self.current()
    }
}

impl<T> DerefMut for DoubleBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.next_mut()
    }
}

impl<T> Borrow<T> for DoubleBuffer<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.current()
    }
}

impl<T> BorrowMut<T> for DoubleBuffer<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.next_mut()
    }
}

impl<T> AsRef<T> for DoubleBuffer<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.current()
    }
}

impl<T> AsMut<T> for DoubleBuffer<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.next_mut()
    }
}

impl<T: PartialEq> PartialEq<T> for DoubleBuffer<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.current().eq(other)
    }
}

impl<T: PartialEq> PartialEq for DoubleBuffer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.current().eq(other.current())
    }
}

impl<T: Eq> Eq for DoubleBuffer<T> {}

impl<T: PartialOrd> PartialOrd<T> for DoubleBuffer<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.current().partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialOrd for DoubleBuffer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.current().partial_cmp(other.current())
    }
}

impl<T: Ord> Ord for DoubleBuffer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.current().cmp(other.current())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_and_modify_with_swap() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(buffer, 1);

        *buffer = 3;
        assert_eq!(buffer, 1);

        buffer.swap();
        assert_eq!(buffer, 3);
    }

    #[test]
    fn test_access_and_modify_with_swap_with_clone() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(buffer, 1);

        *buffer = 3;
        assert_eq!(buffer, 1);

        buffer.swap_with_clone();
        assert_eq!(buffer, 3);
    }

    #[test]
    fn test_access_and_modify_with_swap_with_default() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(buffer, 1);

        *buffer = 3;
        assert_eq!(buffer, 1);

        buffer.swap_with_default();
        assert_eq!(buffer, 3);
    }

    #[test]
    fn test_swap() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(*buffer.current(), 1);
        assert_eq!(*buffer.next(), 2);

        buffer.swap();
        assert_eq!(*buffer.current(), 2);
        assert_eq!(*buffer.next(), 1);
    }

    #[test]
    fn test_swap_with_clone() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(*buffer.current(), 1);
        assert_eq!(*buffer.next(), 2);

        buffer.swap_with_clone();
        assert_eq!(*buffer.current(), 2);
        assert_eq!(*buffer.next(), 2);
    }

    #[test]
    fn test_swap_with_default() {
        let mut buffer: DoubleBuffer<u32> = DoubleBuffer::new(1, 2);
        assert_eq!(*buffer.current(), 1);
        assert_eq!(*buffer.next(), 2);

        buffer.swap_with_default();
        assert_eq!(*buffer.current(), 2);
        assert_eq!(*buffer.next(), 0);
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

        assert_eq!(*buffer.current(), [0, 0, 0]);
        assert_eq!(*buffer.next(), [0, 2, 0]);

        buffer.swap();
        assert_eq!(buffer[1], 2);
        assert_eq!(buffer, [0, 2, 0]);

        assert_eq!(*buffer.current(), [0, 2, 0]);
        assert_eq!(*buffer.next(), [0, 0, 0]);
    }

    #[test]
    fn test_for_iter_mut_bytes_array() {
        let mut buffer: DoubleBuffer<[u8; 3]> = DoubleBuffer::default();
        buffer[1] = 2;

        assert_eq!(*buffer.current(), [0, 0, 0]);
        assert_eq!(*buffer.next(), [0, 2, 0]);

        for byte in buffer.iter_mut() {
            *byte += 1;
        }

        assert_eq!(*buffer.current(), [0, 0, 0]);
        assert_eq!(*buffer.next(), [1, 3, 1]);
    }
}
