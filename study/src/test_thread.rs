use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 多线程任务，不需要获取结果
fn do_001() {
    println!("主线程启动");
    let mut thread_list = Vec::new();

    for i in 0..3 {
        let t = thread::spawn(move || {
            println!("子线程{}启动", i);
            thread::sleep(Duration::from_secs(3));
            println!("子线程{}结束", i);
        });
        thread_list.push(t)
    }
    for t in thread_list {
        t.join().unwrap();
    }
    println!("end");
}


/// 多线程任务，通信获取结果
fn do_002() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    for thread_index in 0..10 {
        let sender = tx.clone(); // 克隆多个发送者
        // 创建线程，并发送消息
        thread::spawn(move || {
            println!("子线程{thread_index} 启动");
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
                sender.send(i).unwrap();
                println!("子线程{thread_index} 成功发送:{i}");
            }
            println!("子线程{thread_index} 结束");
        });
    }

    // 手动丢弃原始发送者，确保所有克隆的发送者关闭后，通道结束
    drop(tx);

    // 在主线程中接收子线程发送的消息并输出
    // println!("receive {}", rx.recv().unwrap());
    for received in rx {
        println!("主线程收到: {}", received);
    }
    println!("end");
}


#[cfg(test)]
mod thread_test {
    use super::*;

    #[test]
    fn test_01() {
        do_001();
    }

    // 线程间通信
    #[test]
    fn test_02() {
        do_002();
    }
}
