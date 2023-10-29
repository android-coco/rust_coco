use std::{sync::{mpsc, Mutex,Arc}, thread, time::Duration};
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    send();

    send2();

    send3();

    arc();
}

fn send() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        tx.send(val).unwrap(); //给主线程发生消息
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

//生产者-->消费者  多观察
fn send2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); //给主线程发生消息
            thread::sleep(Duration::from_secs(1)); //休眠1秒
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//多个生产者
fn send3(){
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); //给主线程发生消息
            thread::sleep(Duration::from_secs(1)); //休眠1秒
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap(); //给主线程发生消息
            thread::sleep(Duration::from_secs(1)); //休眠1秒
        }
    });

    for received in rx {
        println!("Got: {}", received);
    } 
}

//内存共享数据   锁  arc原子引用计数
fn arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();

            *num +=1;
        });
        //加入线程
        handles.push(handle);
    }

    for hander in handles {
        hander.join().unwrap();
    }

    println!("Result: {}",counter.lock().unwrap());
}