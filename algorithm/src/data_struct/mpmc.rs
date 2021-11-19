use anyhow::{anyhow, Result};
use std::sync::atomic::Ordering::*;
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicU32, Arc, Condvar, Mutex},
};

struct Channel<T> {
    condvar: Condvar,
    recvers: AtomicU32,
    senders: AtomicU32,
    vec: Mutex<VecDeque<T>>,
}
pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}
pub struct Recv<T> {
    channel: Arc<Channel<T>>,
}
impl<T> Channel<T> {
    fn total_sender(&self) -> u32 {
        self.senders.load(Acquire)
    }
    fn total_recv(&self) -> u32 {
        self.recvers.load(Acquire)
    }
}

pub fn unbounded<T>() -> (Sender<T>, Recv<T>) {
    let c = Arc::new(Channel {
        vec: Mutex::new(VecDeque::new()),
        condvar: Condvar::new(),
        senders: AtomicU32::new(1),
        recvers: AtomicU32::new(1),
    });
    let sender = Sender { channel: c.clone() };

    let recv = Recv { channel: c };
    (sender, recv)
}
impl<T> Recv<T> {
    pub fn recv(&self) -> Result<T> {
        let mut guard = self.channel.vec.lock().unwrap();
        loop {
            match guard.pop_front() {
                Some(t) => {
                    return Ok(t);
                }
                None => {
                    if self.channel.total_sender() == 0 {
                        return Err(anyhow!("no sender"));
                    }
                    guard = self
                        .channel
                        .condvar
                        .wait(guard)
                        .map_err(|_| anyhow!("lock err"))?;
                }
            }
        }
    }
}
impl<T> Iterator for Recv<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}
impl<T> Drop for Recv<T> {
    fn drop(&mut self) {
        self.channel.recvers.fetch_sub(1, AcqRel);
    }
}

impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<()> {
        if self.channel.total_recv() == 0 {
            return Err(anyhow!("no recv"));
        }
        let mut c = self.channel.vec.lock().unwrap();
        let empty = c.is_empty();
        c.push_back(t);
        if empty {
            self.channel.condvar.notify_one();
        }
        Ok(())
    }

    pub fn clone(&self) -> Self {
        self.channel.senders.fetch_add(1, AcqRel);
        Self {
            channel: self.channel.clone(),
        }
    }
    pub fn total_queued_items(&self) -> usize {
        self.channel.vec.lock().unwrap().len()
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.channel.senders.fetch_sub(1, AcqRel);
    }
}

#[cfg(test)]
mod test {
    use super::unbounded;
    use std::{thread, time::Duration};
    #[test]
    fn channel_should_work() {
        let (s, r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        // 在这个测试里，数据到达的顺序是不确定的，所以我们排个序再 assert
        result.sort();

        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据，确保它和发送的数据一致
                assert_eq!(idx, i);
            }
            // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 再次发送数据，唤醒消费者
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 留点时间让 receiver 处理
        thread::sleep(Duration::from_millis(1));

        // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        // sender 即用即抛
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // sender 在此被丢弃
            })
            .join()
            .unwrap();
        }

        // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而，读取更多数据时会出错
        assert!(r.recv().is_err());
    }
    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }
}
