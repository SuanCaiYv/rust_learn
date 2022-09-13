#![feature(generator_trait)]
#![feature(generators)]

use std::future::Future;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
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

    fn poll(self: Pin<&mut Self>, waker: Waker) -> Poll<String> {
        return if Instant::now() >= self.when {
            Poll::Ready(format!("run after {} seconds", self.delay_sec))
        } else {
            let delay_sec = self.delay_sec;
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(delay_sec));
                waker.wake();
            });
            Poll::Pending
        }
    }
}

struct Task {
    // 简单起见，不用Context，不然会引入一大堆生命周期问题
    generator: Pin<Box<dyn Generator<Waker, Yield=Poll<()>, Return=()>>>,
}

impl Task {
    fn new(delay_queue: Vec<u64>) -> Task {
        let mut gen = move |waker: Waker| {
            println!("task running...");
            for delay in delay_queue.into_iter() {
                let mut timer = Timer::new(delay);
                // 最核心的在这里，这里大概展示了 非叶子Future是怎么把await转换成yield操作的
                loop {
                    let mut timer_mut = &mut timer;
                    let timer_pin = Pin::new(timer_mut);
                    let poll_res = timer_pin.poll(waker.clone());
                    if let Poll::Ready(str) = poll_res {
                        println!("{}", str);
                        break;
                    } else {
                        yield Poll::Pending;
                    }
                }
            };
        };
        let gen_pin = Pin::new(Box::new(gen));
        Task {
            generator: gen_pin,
        }
    }

    fn poll(mut self: Pin<&mut Self>, waker: Waker) -> Poll<()> {
        return if let GeneratorState::Yielded(_) = self.generator.as_mut().resume(waker.clone()) {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |data| {
            RawWaker::new(data, &VTABLE)
        },
        |data| {
        },
        |data| {
        },
        |data| {
        },
    )
};

fn main() {
    let mut task = Task::new(vec![1, 2, 3]);
    loop {
        let task_pin = Pin::new(&mut task);
        if let Poll::Ready(_) = task_pin.poll(unsafe {Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE))}) {
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}