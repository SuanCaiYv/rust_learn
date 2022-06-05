use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;

struct Delay {
    when: Instant,
    // 因为Future可能在多个线程之间转移，为了模拟真实情况，我们会判断当前线程提供的waker是否和我们保存的一致
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Delay {
    fn new(duration: Instant) -> Self {
        Delay {
            when: duration,
            waker: None,
        }
    }
}

impl Future for Delay {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 如果当前的存在waker
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            // 不存在就简单多了，直接创建
            let when = self.when;
            // 使用传入的waker作为当前waker保存在Delay中
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            // waker后面会被移动到闭包，所以这里clone
            self.waker = Some(waker.clone());
            thread::spawn(move || {
                let curr = Instant::now();
                if curr < when {
                    thread::sleep(when - curr);
                }
                let waker = waker.lock().unwrap();
                // 唤醒waker
                waker.wake_by_ref();
            });
        }
        if Instant::now() >= self.when {
            println!("done");
            Poll::Ready("ok".to_string())
        } else {
            Poll::Pending
        }
    }
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = String> + Send>>>,
    tail: channel::Sender<Arc<Task>>,
}

impl Task {
    fn execute(self: &Arc<Task>) {
        // 唤醒waker的最终实现，就是把它添加到任务队列中等待推进
        self.tail.send(self.clone());
    }

    fn poll(self: Arc<Task>) -> String {
        // 根据ArcWaker创建一个waker
        let waker = task::waker(self.clone());
        // 创建对应的上下文，或者可以理解成一个waker包装器
        let mut context = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        // 触发Future的poll
        let ans = future.as_mut().poll(&mut context);
        if let Poll::Ready(str) = ans {
            str
        } else {
            "waiting".to_string()
        }
    }

    fn spawn<F>(future: F, tail: &channel::Sender<Arc<Task>>) where F: Future<Output = String> + Send + 'static {
        let task = Arc::new(Task{
            future: Mutex::new(Box::pin(future)),
            tail: tail.clone(),
        });
        task.execute();
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Task>) {
        arc_self.execute()
    }
}

struct MiniTokio {
    head: channel::Receiver<Arc<Task>>,
    tail: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn run(&self) {
        // 从任务队列拉取任务并执行
        while let Ok(task) = self.head.recv() {
            let ans = task.poll();
            println!("{}", ans);
        }
    }

    fn new() -> MiniTokio {
        let (sender, receiver) = channel::unbounded();
        MiniTokio {
            head: receiver,
            tail: sender,
        }
    }

    fn spawn<F>(&self, future: F) where F: Future<Output = String> + Send + 'static {
        Task::spawn(future, &self.tail)
    }
}

/// 大致执行如下：
/// mini_tokio::spawn ->
///                      task::spawn ->
///                                     new Delay -> tail.send(delay)
/// mini_tokio::run -> head.recv() -> delay.poll()
///                    Poll::Pending
///                    ... ....
///                    waker.wake()
///                    head.recv() -> delay.poll()
///                    Poll::Ready
///
///
fn main() {
    let mini_tokio = MiniTokio::new();
    let timer = Delay::new(Instant::now() + Duration::from_secs(5));
    mini_tokio.spawn(timer);
    mini_tokio.run();
}
