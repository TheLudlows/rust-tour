use std::cell::RefCell;
use std::io;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::os::fd::{AsFd, AsRawFd};
use std::pin::Pin;
use std::rc::{Rc};
use std::task::{Context, Poll};
use futures::{Stream};
use socket2::{Domain, Protocol, Socket, Type};
use tokio::io::{AsyncRead, ReadBuf, AsyncWrite};
use crate::reactor::Reactor;

pub struct MyTcpListener {
    reactor: Rc<RefCell<Reactor>>,
    listener: TcpListener,
}

impl MyTcpListener {
    // impl < A:
    pub fn bind(add: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let addr = add.to_socket_addrs()?.next().unwrap();
        let sk = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        let addr = socket2::SockAddr::from(addr);
        sk.set_reuse_address(true)?;
        sk.bind(&addr)?;
        sk.listen(1024)?;

        let reactor = Reactor::reactor();
        reactor.borrow_mut().add(sk.as_raw_fd());
        println!("tcp bind with fd {}", sk.as_raw_fd());
        Ok( Self{
            reactor,
            listener: sk.into(),
        })
    }
}
impl Stream for MyTcpListener {
    type Item = io::Result<(MyTcpStream, SocketAddr)>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.listener.accept() {
            Ok((stream, addr)) => Poll::Ready(Some(Ok((stream.into(), addr)))),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("poll next in listener {}", self.listener.as_raw_fd());
                // modify reactor to register interest
                self.reactor.borrow_mut().modify_readable(self.listener.as_raw_fd(), cx);
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

pub struct MyTcpStream {
    stream: TcpStream,
}

impl From<TcpStream> for MyTcpStream {
    fn from(stream: TcpStream) -> Self {
        let reactor = Reactor::reactor();
        reactor.borrow_mut().add(stream.as_raw_fd());
        Self { stream }
    }
}

impl Drop for MyTcpStream {
    fn drop(&mut self) {
        println!("drop tcp stream");
        let reactor = Reactor::reactor();
        reactor.borrow_mut().delete(self.stream.as_raw_fd());
    }
}

impl AsyncRead for MyTcpStream {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let  fd = self.stream.as_raw_fd();

        unsafe {
            let read_buf = &mut *(buf.unfilled_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
            println!("read for fd {}", fd);
            match self.stream.read(read_buf) {
                Ok(n) => {
                    buf.assume_init(n);
                    println!("read for fd {} done,  buf {}", fd, String::from_utf8_lossy(&read_buf[..n]));
                    buf.advance(n);
                    Poll::Ready(Ok(()))
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock=> {
                    println!("read for fd {} done WouldBlock", fd);
                    Reactor::reactor().borrow_mut().modify_readable(self.stream.as_raw_fd(), cx);
                    Poll::Pending
                },
                Err(e) => {
                    Poll::Ready(Err(e))
                }
            }
        }
    }
}

impl AsyncWrite for MyTcpStream {


    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
       Poll::Ready(Ok(()))
    }

    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        let fd = self.stream.as_raw_fd();
        match self.stream.write(buf) {
            Ok(n) => {
                println!("write for fd {:?} ready", fd);

                Poll::Ready(Ok(n))
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                println!("write for fd {:?} pending", fd);

                Reactor::reactor().borrow_mut().modify_writable(fd, cx);
                Poll::Pending
            }
            Err(e) => {
                Poll::Ready(Err(e))
            }
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.stream.shutdown(Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}