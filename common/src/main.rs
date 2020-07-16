mod async_mod;
mod compresion;
mod json;

use compressed_vec::VectorU32Appender;
use compressed_vec::vector::VectorAppender;
use compressed_vec::section::AutoEncoder;

fn main() {
    let mut encoder:VectorAppender<u32,AutoEncoder> = VectorAppender::try_new(32).unwrap();
}
