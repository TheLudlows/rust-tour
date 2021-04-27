use std::cell::Cell;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Instant;

thread_local! {
  static COUNTER: Cell<u32> = Cell::new(0);
}

const STEPS: u32 = 1_000_000_000;

fn sum_rust() -> u32 {
    for step in 0..STEPS {
        COUNTER.with(|it| {
            it.set(it.get() + 1)
        })
    }
    COUNTER.with(|it| it.get())
}

#[test]
fn main() {
    let t = Instant::now();
    let r = sum_rust();
    eprintln!("Rust:{} {}ms", r, t.elapsed().as_millis());
}

static N: AtomicU32 = AtomicU32::new(0);

#[test]
fn test_pre_thread() {
    for i in 0..10 {
        thread::spawn(|| {
            let x = COUNTER.with(|it| {
                if it.get() == 0 {
                    N.fetch_add(1, Relaxed);
                    it.set(N.load(Relaxed));
                }
                it.get()
            });
            println!("{:?}", x);
        });
    }
}

mod test {
    pub struct TestStruct {
        pub a: i32,
    }
}

mod test_mod {
    use crate::thread_local::test::TestStruct;

    #[test]
    fn test() {
        let s = TestStruct { a: 1 };
        s.a;
    }
}