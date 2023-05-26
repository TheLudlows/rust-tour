

use std::io::{BufWriter, Write};
use std::net::TcpStream;

use syn::token::Dyn;
#[derive(Debug)]
struct MyWriter<W>
where
    W: Write,
{
    writer: BufWriter<W>,
}
impl<W: Write> MyWriter<W> {
    pub fn new(stream: W) -> Self {
        Self {
            writer: BufWriter::new(stream),
        }
    }
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}
fn main() {
    let addr = "127.0.0.1:8080";
    let stream = TcpStream::connect(addr).unwrap();
    let mut writer = MyWriter::new(stream);
    writer.write("hello world!").unwrap();
}
struct Writer {
    writer: Box<dyn Write>
}

impl  Writer {

    fn new(add: &str) -> Self {
        Self {
            writer: Box::new(BufWriter::new(TcpStream::connect(add).unwrap())),
        }
    }
}