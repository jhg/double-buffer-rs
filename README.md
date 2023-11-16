# Generic and simple double buffer

This only provides [`DoubleBuffer<T>`], for implementing a double buffer pattern.

[`DoubleBuffer<T>`] is not limited to bytes arrays or similar buffers, it can be used with any type
that requires modify while reading current state and all the changes look as one atomic operation.

## Swapping Benchmarks

The following are the results in a i7 10th gen with 32GB RAM for a `vec![0u8; 16777216]` buffer:

1. [`DoubleBuffer::swap()`] - 1.6655 ns 1.6814 ns 1.6964 ns
2. [`DoubleBuffer::swap_with_default()`] - 1.7547 ns 1.8009 ns 1.8262 ns
3. [`DoubleBuffer::swap_with_clone()`] - 4.4526 ms 4.5241 ms 4.5989 ms

[`DoubleBuffer<T>`]: https://docs.rs/double-buffer/latest/double_buffer/struct.DoubleBuffer.html
[`DoubleBuffer::swap()`]: https://docs.rs/double-buffer/latest/double_buffer/struct.DoubleBuffer.html#method.swap
[`DoubleBuffer::swap_with_default()`]: https://docs.rs/double-buffer/latest/double_buffer/struct.DoubleBuffer.html#method.swap_with_default
[`DoubleBuffer::swap_with_clone()`]: https://docs.rs/double-buffer/latest/double_buffer/struct.DoubleBuffer.html#method.swap_with_clone
