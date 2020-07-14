use compressed_vec::{VectorU32Appender, VectorF32XorAppender, VectorReader};
use compressed_vec::vector::VectorAppender;
use compressed_vec::section::AutoEncoder;

#[test]
fn test_int() {
    let mut encoder:VectorAppender<u32,AutoEncoder> = VectorAppender::try_new(32).unwrap();
}