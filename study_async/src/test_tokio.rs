use tokio::time::{sleep, Duration};


/// 无参异步函数
async fn func_no_args() {
    println!("func_no_args start...");
    sleep(Duration::from_secs(2)).await; // 模拟异步操作
    println!("func_no_args end.");
}


/// 有参异步函数
async fn func_hive_args(name: &str, seconds: u64) {
    println!("func_no_args start...");
    println!("Hello, name:{}! seconds:{}...", name, seconds);
    sleep(Duration::from_secs(seconds)).await; // 模拟异步操作
    println!("func_no_args end...");
}

///tokio::sync模块提供了几种状态同步的机制：
/// Mutex: 互斥锁
/// RwLock: 读写锁
/// Notify: 通知唤醒机制
/// Barrier: 屏障
/// Semaphore: 信号量
async fn func_mutex() {
    use tokio::sync::{Mutex, RwLock, Notify, Barrier, Semaphore};
    use std::sync::Arc;


    // 1. Mutex
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex);

    let handle = tokio::spawn(async move {
        let mut num = mutex_clone.lock().await;
        *num += 1;
        println!("Mutex incremented: {}", *num);
    });

    handle.await.unwrap();
    println!("Final Mutex value: {}", *mutex.lock().await);

    // 2. RwLock
    let rwlock = Arc::new(RwLock::new(0));
    let rwlock_clone = Arc::clone(&rwlock);

    let handle_read = tokio::spawn(async move {
        let num = rwlock_clone.read().await;
        println!("RwLock read: {}", *num);
    });

    let handle_write = tokio::spawn(async move {
        let mut num = rwlock.write().await;
        *num += 1;
        println!("RwLock incremented: {}", *num);
    });

    handle_read.await.unwrap();
    handle_write.await.unwrap();
    println!("Final RwLock value: {}", *rwlock.read().await);

    // 3. Notify
    let notify = Arc::new(Notify::new());
    let notify_clone = Arc::clone(&notify);

    tokio::spawn(async move {
        println!("Waiting for notification...");
        notify_clone.notified().await;
        println!("Received notification!");
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    notify.notify_one();
    println!("Sent notification!");

    // 4. Barrier
    let barrier = Arc::new(Barrier::new(3));
    let barrier_clone = Arc::clone(&barrier);

    let handle_barrier = tokio::spawn(async move {
        println!("Task 1 waiting at the barrier...");
        barrier_clone.wait().await;
        println!("Task 1 passed the barrier!");
    });

    tokio::spawn(async move {
        println!("Task 2 waiting at the barrier...");
        barrier.wait().await;
        println!("Task 2 passed the barrier!");
    });

    handle_barrier.await.unwrap();
    barrier.wait().await; // Main task also waits
    println!("Main task passed the barrier!");

    // 5. Semaphore
    let semaphore = Arc::new(Semaphore::new(2));

    let sem_clone = Arc::clone(&semaphore);
    let handle_semaphore = tokio::spawn(async move {
        let _permit = sem_clone.acquire().await.unwrap();
        println!("Acquired semaphore permit in Task 1!");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    });

    let sem_clone2 = Arc::clone(&semaphore);
    tokio::spawn(async move {
        let _permit = sem_clone2.acquire().await.unwrap();
        println!("Acquired semaphore permit in Task 2!");
    });

    handle_semaphore.await.unwrap();
    println!("Main task completed!");
}

///tokio使用通道在task之间进行通信，有四种类型通道：oneshot、mpsc、broadcast和watch。
/// 1、Oneshot 通道：
/// 用于单一发送和接收一条消息。
/// Sender 发送消息，Receiver 接收。
/// 2、MPSC 通道：
/// 多生产者单消费者通道，适合多个任务发送消息到一个接收任务。
/// Sender 用于发送消息，Receiver 用于接收。
/// 3、Broadcast 通道：
/// 多发送者多接收者通道，所有订阅者都能接收到相同的消息。
/// Sender 用于发送，Receiver 通过订阅来接收消息。
/// 4、Watch 通道：
/// 用于发送值的更新通知，接收者可以获取当前值并监听变化。
/// Sender 发送更新值，Receiver 可以获取当前值并监听变化
async fn func_channel() {
    use tokio::sync::{mpsc, oneshot, broadcast, watch};
    use tokio::task;


    // 1. oneshot通道
    let (tx, rx): (oneshot::Sender<String>, oneshot::Receiver<String>) = oneshot::channel();
    task::spawn(async move {
        tx.send("Hello from oneshot!".to_string()).unwrap();
    });

    let msg = rx.await.unwrap();
    println!("Oneshot received: {}", msg);

    // 2. MPSC通道
    let (tx, mut rx) = mpsc::channel(32);
    task::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Message {}", i)).await.unwrap();
        }
    });

    while let Some(message) = rx.recv().await {
        println!("MPSC received: {}", message);
    }

    // 3. Broadcast通道
    let (tx, _) = broadcast::channel(10);
    let mut _rx1 = tx.subscribe();
    let mut _rx2 = tx.subscribe();

    task::spawn(async move {
        tx.send("Hello from broadcast!").unwrap();
    });

    // 接收广播消息
    let msg1 = _rx1.recv().await.unwrap();
    println!("Broadcast received by rx1: {}", msg1);

    let msg2 = _rx2.recv().await.unwrap();
    println!("Broadcast received by rx2: {}", msg2);

    // 4. Watch通道
    let (tx, mut rx) = watch::channel(0);
    task::spawn(async move {
        tx.send(1).unwrap();
        tx.send(2).unwrap(); // 更新值
    });

    let value = rx.borrow(); // 获取当前值
    println!("Watch current value: {}", value);

    // 等待更新
    let new_value = rx.changed().await.unwrap();
    println!("Watch updated value: {}", *new_value);
}


#[cfg(test)]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_01() {
        println!("Calling async func_no_args start...");
        func_no_args().await; // 调用无参数异步函数
        println!("Calling async func_no_args end.");
    }

    #[tokio::test]
    async fn test_02() {
        let name = "Alice";
        let delay = 2;
        println!("Calling async func_hive_args start...");
        func_hive_args(name, delay).await; // 调用无参数异步函数
        println!("Calling async func_hive_args end.");
    }


    #[tokio::test]
    async fn test_03() {
        println!("Calling async func_mutex start...");
        func_mutex().await;
        println!("Calling async func_mutex end.");
    }
    #[tokio::test]
    async fn test_04() {
        println!("Calling async func_channel start...");
        func_channel().await;
        println!("Calling async func_channel end.");
    }
}