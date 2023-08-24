#![feature(generators)]
#![feature(generator_trait)]

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use std::ops::Generator;
use std::time::Duration;

#[derive(Clone)]
struct SimpleWaker {
    wake_fn: Arc<Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}

impl SimpleWaker {
    fn wake(&self) {
        let mut wake_fn = self.wake_fn.lock().unwrap();
        wake_fn();
    }

    fn empty() -> Self {
        SimpleWaker {
            wake_fn: Arc::new(Mutex::new(Box::new(|| {}))),
        }
    }

    fn set_wake_fn(&self, wake_fn0: Box<dyn FnMut() -> () + Send + 'static>) {
        let mut wake_fn = self.wake_fn.lock().unwrap();
        *wake_fn = wake_fn0;
    }
}

struct SimpleTimeout {
    duration: Duration,
    timeout: bool,
}

impl SimpleTimeout {
    fn poll(mut self: Pin<&mut Self>, waker: &SimpleWaker) -> Poll<()> {
        if self.timeout {
            return Poll::Ready(());
        }
        let waker_clone = waker.clone();
        let duration = self.duration;
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            waker_clone.wake();
        });
        self.timeout = true;
        Poll::Pending
    }
}

fn main() {
    let waker0 = SimpleWaker::empty();
    let waker = waker0.clone();
    let mut generator = move || {
        println!("test start");
        let mut vec = vec![1, 2, 3];
        let mut timer = SimpleTimeout {
            duration: Duration::from_secs(1),
            timeout: false,
        };
        loop {
            match Pin::new(&mut timer).poll(&waker) {
                Poll::Ready(()) => {
                    break;
                }
                Poll::Pending => {
                    yield ();
                }
            }
        }
        vec.push(4);
        println!("{:?}", vec);
        let display = format!("{:?}", vec);
        let mut timer = SimpleTimeout {
            duration: Duration::from_secs(1),
            timeout: false,
        };
        loop {
            match Pin::new(&mut timer).poll(&waker) {
                Poll::Ready(()) => {
                    break;
                }
                Poll::Pending => {
                    yield ();
                }
            }
        }
        println!("{}", display);
        println!("test end");
        return ();
    };
    waker0.set_wake_fn(Box::new(move || {
        Pin::new(&mut generator).resume(());
    }));
    waker0.wake();
    let (_tx, rx) = std::sync::mpsc::channel::<()>();
    _ = rx.recv();
}