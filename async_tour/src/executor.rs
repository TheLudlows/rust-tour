use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use futures::future::LocalBoxFuture;
use futures::{FutureExt, pin_mut};
use scoped_tls::scoped_thread_local;
use crate::reactor::Reactor;

scoped_thread_local!(pub(crate) static EX: Executor);

pub struct Task {
    future: RefCell<LocalBoxFuture<'static, ()>>,
}

impl Task {
    fn wake_(self: Rc<Self>) {
        Self::wake_by_ref_(&self)
    }

    fn wake_by_ref_(self: &Rc<Self>) {
        EX.with(|ex| ex.push(self.clone()));
    }
}

pub struct Executor {
    local_queue: RefCell<VecDeque<Rc<Task>>>,
    pub(crate) reactor: Rc<RefCell<Reactor>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            local_queue: RefCell::new(Default::default()),
            reactor: Rc::new(RefCell::new(Reactor::new())),
        }
    }
    pub fn spawn(fut: impl Future<Output=()> + 'static) {
        let t = Rc::new(Task {
            future: RefCell::new(fut.boxed_local()),
        });
        EX.with(|ex| ex.push(t));
    }

    pub fn block_on<F, O, T>(&self, f: F) -> O where F: Fn() -> T, T: Future<Output=O> + 'static {
        let _waker = waker_fn::waker_fn(|| {});
        let cx = &mut Context::from_waker(&_waker);

        EX.set(self, || {
            let fut = f();
            pin_mut!(fut);
            loop {
                if let Poll::Ready(t) = fut.as_mut().poll(cx) {
                    return t;
                }
                // do all task
                while let Some(task) = self.pop() {
                    let w = waker(task.clone());
                    let mut cx = Context::from_waker(&w);
                    let future = task.future.borrow_mut();
                    let _ = Pin::new(future).as_mut().poll(&mut cx);
                }
                if let Poll::Ready(t) = fut.as_mut().poll(cx) {
                    return t;
                }
                self.reactor.borrow_mut().wait();
            }
        })
    }
    fn push(&self, task: Rc<Task>) {
        self.local_queue.borrow_mut().push_back(task);
    }
    fn pop(&self) -> Option<Rc<Task>> {
        self.local_queue.borrow_mut().pop_front()
    }
}

fn waker(wake: Rc<Task>) -> Waker {
    let ptr = Rc::into_raw(wake) as *const ();
    let vtable = &Helper::VTABLE;
    unsafe { Waker::from_raw(RawWaker::new(ptr, vtable)) }
}

struct Helper;

impl Helper {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone_waker(data: *const ()) -> RawWaker {
        //increase_refcount(data);
        let vtable = &Self::VTABLE;
        RawWaker::new(data, vtable)
    }

    unsafe fn wake(ptr: *const ()) {
        let rc = Rc::from_raw(ptr as *const Task);
        rc.wake_();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let rc = mem::ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
        rc.wake_by_ref_();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Rc::from_raw(ptr as *const Task));
    }
}