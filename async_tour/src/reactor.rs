use std::collections::HashMap;
use std::os::fd::RawFd;
use std::task::Context;
use nix::libc::SYS_nanosleep;
use polling::{Event, Events, Poller};
use crate::waker::Waker;

pub struct Reactor {
    poller: Poller,
    waker_map: HashMap<u64, Waker>,
    buffer: Events
}

impl Default for Reactor {
    fn default() -> Self {
        return Self::new()
    }
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            poller: Poller::new().unwrap(),
            waker_map: Default::default(),
            buffer: Default::default(),
        }
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
            polling::Event::none(fd as usize))
            .unwrap();
        }
    }

    pub fn modify_readable(&mut self, fd: RawFd, cx: &mut Context) {
        println!("[reactor] modify_readable fd {} token {}", fd, fd * 2);

        self.push_completion(fd as u64 * 2, cx);
        let event = polling::Event::readable(fd as usize);
        self.poller.modify(fd, event).unwrap();
    }

    pub fn modify_writable(&mut self, fd: RawFd, cx: &mut Context) {
        println!("[reactor] modify_writable fd {}, token {}", fd, fd * 2 + 1);

        self.push_completion(fd as u64 * 2 + 1, cx);
        let event = polling::Event::writable(fd as usize);
        self.poller.modify(fd, event).unwrap();
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