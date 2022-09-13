#![feature(generator_trait)]
#![feature(generators)]

use std::future::Future;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::task::Poll::Pending;
use std::time::Duration;
use tokio::time::Instant;

struct Timer {
    delay_sec: u64,
    when: Instant,
}

impl Timer {
    fn new(delay_sec: u64) -> Self {
        Timer {
            delay_sec,
            when: Instant::now() + Duration::from_secs(delay_sec),
        }
    }
}

impl Future for Timer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        return if Instant::now() <= self.when {
            Poll::Ready(format!("run after {} seconds", self.delay_sec))
        } else {
            let waker = cx.waker().clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(self.delay_sec));
                waker.wake();
            });
            Poll::Pending
        }
    }
}

struct Task {
    generator: Option<dyn Generator<Yield=Pending<String>, Return=String>>
}

impl Task {
    fn new() -> Task {
        Task {
            generator: None,
        }
    }
}

impl Future for Task {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn main() {
}