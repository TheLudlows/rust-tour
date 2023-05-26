use anyhow::Ok;
use anyhow::anyhow;
use anyhow::Result;
use std::time::Duration;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, Condvar, Mutex,
    },
    thread,
};

#[test]
fn test_channel() {
    let  (s,  r) = unbounded();
    s.send("hello".to_string()).unwrap();
    let msg = r.recv().unwrap();
    assert_eq!(msg, "hello")
}

#[test]
fn test_channel_mutil() {
    let (s, r) = mpsc::channel();
    s.send("hello".to_string()).unwrap();
    let s1 = s.clone();
    let j = thread::spawn(move || {
        s1.send("workd".to_string()).unwrap();
    });
    j.join().unwrap();
    assert_eq!(r.recv().unwrap(), "hello");
    assert_eq!(r.recv().unwrap(), "workd");
}

#[test]
fn receiver_should_be_blocked_when_nothing_to_read() {
    let (s, r) = unbounded();
    let s1 = s.clone();
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
    assert_eq!(s1.size(), 0);
}

#[derive(Default)]
struct Mpmp<T> {
    queue: Mutex<VecDeque<T>>,
    senders: AtomicUsize,
    recvs: AtomicUsize,
    available: Condvar,
}
pub struct Sender<T> {
    shared: Arc<Mpmp<T>>,
}
pub struct Receiver<T> {
    shared: Arc<Mpmp<T>>,
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let mp = Mpmp {
        queue: Mutex::new(VecDeque::new()),
        senders: AtomicUsize::new(1),
        recvs: AtomicUsize::new(1),
        available: Condvar::new(),
    };
    let arc = Arc::new(mp);
    let sender = Sender {
        shared: arc.clone(),
    };
    let rec = Receiver { shared: arc };
    (sender, rec)
}
impl<T> Sender<T> {
    pub fn send(&self, data : T) -> Result<()> {
        if self.shared.recvs.load(Ordering::SeqCst) == 0 {
            return Err(anyhow!("no consumer"));
        }
        let mut queue = self.shared.queue.lock().unwrap();
        let empty = queue.is_empty();
        queue.push_back(data);
        if empty {
            self.shared.available.notify_one();
        }
        Ok(())
    }
    pub fn size(&self)  -> usize {
        self.shared.queue.lock().unwrap().len()
    }

}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T> {
        let mut queue = self.shared.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(v) => return Ok(v),
                None => {
                    if self.shared.senders.load(Ordering::Acquire) == 0 {
                        return Result::Err(anyhow!("no senders"));
                    } else {
                        queue = self.shared.available.wait(queue).map_err(|e| anyhow!("error wait"))?;
                    }
                }
            }
        }
    }
}
impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.shared.recvs.fetch_add(1, Ordering::AcqRel);

        Self {
            shared: self.shared.clone(),
        }
    }
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: self.shared.clone(),
        }
    }
}
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
       self.shared.recvs.fetch_sub(1, Ordering::AcqRel);
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let senders = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if senders <= 1 {
            self.shared.available.notify_all();
        }
    }
}
impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}