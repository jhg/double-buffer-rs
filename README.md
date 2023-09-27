# Generic and simple double buffer

This only provides [`DoubleBuffer<T>`], for implementing a double buffer pattern.

[`DoubleBuffer<T>`] is not limited to bytes arrays or similar buffers, it can be used with any type
that requires modify while reading current state and all the changes look as one atomic operation.

[`DoubleBuffer<T>`]: https://docs.rs/double-buffer/latest/double_buffer/struct.DoubleBuffer.html
