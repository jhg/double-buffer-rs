use double_buffer::DoubleBuffer;

#[test]
fn test_debug_format() {
    let buffer: DoubleBuffer<u32> = DoubleBuffer::default();
    assert_eq!(format!("{:?}", buffer), "DoubleBuffer { current: 0, next: 0 }");
}

#[test]
fn test_pointer_format() {
    let buffer: DoubleBuffer<u32> = DoubleBuffer::default();
    assert!(format!("{:p}", buffer).starts_with("0x"));
}
