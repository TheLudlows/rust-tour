use std::cell::RefCell;
use std::collections::HashMap;
use std::os::fd::{AsRawFd, BorrowedFd, RawFd};
use std::rc::Rc;
use std::task::{Context, Waker};
use polling::{AsRawSource, Event, Events, Poller};
use crate::executor;

pub struct Reactor {
    poller: Poller,
    waker_map: HashMap<u64, Waker>,
    buffer: Events
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            poller: Poller::new().unwrap(),
            waker_map: Default::default(),
            buffer: Default::default(),
        }
    }
    pub fn reactor() -> Rc<RefCell<Self>> {
        executor::EX.with(|e| e.reactor.clone())
    }
    pub fn add(&mut self, fd: RawFd) {
        println!("[reactor]add fd {} to reactor", fd);
        let flags = nix::fcntl::OFlag::from_bits(nix::fcntl::fcntl(fd, nix::fcntl::F_GETFL).unwrap())
                .unwrap();
        let flags_nonblocking = flags | nix::fcntl::OFlag::O_NONBLOCK;
        nix::fcntl::fcntl(fd, nix::fcntl::F_SETFL(flags_nonblocking)).unwrap();
        unsafe  {
            self.poller
            .add(fd,
             Event::none(fd as usize))
            .unwrap();
        }
    }

    pub fn delete(&mut self, fd: RawFd) {
        println!("now delete fd {}", fd);
        self.waker_map.remove(&(fd as u64 * 2));
        self.waker_map.remove(&(fd as u64 * 2 + 1));
        println!(
            "[reactor token] fd {} wakers token {}, {} removed",
            fd,
            fd * 2,
            fd * 2 + 1
        );
    }
    pub fn modify_readable(&mut self, fd: BorrowedFd<'_>, cx: &mut Context) {
        let raw = fd.as_raw_fd();
        println!("[reactor] modify_readable fd {} token {}", raw, raw * 2);

        self.push_completion(raw as u64 * 2, cx);
        let event = Event::readable(raw as usize);
        self.poller.modify(fd, event).unwrap();
    }

    pub fn modify_writable(&mut self, fd: BorrowedFd<'_>, cx: &mut Context) {
        let raw = fd.as_raw_fd();

        println!("[reactor] modify_writable fd {}, token {}", raw, raw * 2 + 1);

        self.push_completion(raw as u64 * 2 + 1, cx);
        let event = polling::Event::writable(raw as usize);
        self.poller.modify(fd, event).unwrap();
    }
    fn push_completion(&mut self, token: u64, cx: &mut Context) {
        println!("[reactor token] token {} waker saved", token);

        self.waker_map.insert(token, cx.waker().clone());
    }
    pub fn wait(&mut self) {
        println!("[reactor] waiting");
        self.poller.wait(&mut self.buffer, None).unwrap();
        println!("[reactor] wait done");

        for event in self.buffer.iter() {
            if event.readable {
                if let Some(waker) = self.waker_map.remove(&(event.key as u64 * 2)) {
                    println!(
                        "[reactor token] fd {} read waker token {} removed and woken",
                        event.key,
                        event.key * 2
                    );
                    waker.wake();
                }
            }
            if event.writable {
                if let Some(waker) = self.waker_map.remove(&(event.key as u64 * 2 + 1)) {
                    println!(
                        "[reactor token] fd {} write waker token {} removed and woken",
                        event.key,
                        event.key * 2 + 1
                    );
                    waker.wake();
                }
            }
        }
    }
}