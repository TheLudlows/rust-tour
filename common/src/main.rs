use compressed_vec::section::AutoEncoder;
use compressed_vec::vector::VectorAppender;
use compressed_vec::VectorU32Appender;

mod compresion;
mod async_mod;

fn main() {
    let mut encoder: VectorAppender<u32, AutoEncoder> = VectorAppender::try_new(32).unwrap();
}
