
//并发：concurrent 程序的不同部分之间独立运行
//并行：parallel 程序的不同部分同时运行

fn main() {
    arc();

}

//多线程可导致的问题： 1、竞争状态 ：线程以不一致的顺序访问数据 2、死锁：两个线程彼此等待对方所持有的资源，无法继续

//实现线程的方式：
//-通过OS的api来创建线程 1:1 模型 -需要较小的运行时
//-语言自己实现的线程 M:N模型 -需要更大运行时
//rust需要权衡运行时的支持
//rust 标准库只提供1:1模型的线程

//通过Thread::spawn创建线程: 参数：一个闭包（在新线程里运行的代码）

use std::time::Duration;
fn new_thread(){
    std::thread::spawn(|| {//闭包里传递了空的参数
        for i in 1..10{
                println!("number {} from the spawned thread",i);
                std::thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("--hi number {} from the main thread",i); //主线程结束后，不管其他线程是否结束，都结束
        std::thread::sleep(Duration::from_millis(1));
    }
}

// 通过join handle来等待所有线程的完成 
//-thread::spawn 函数的返回类型是joinhandle
//-joinhandle 持有值的所有权：调用其join方法可以等待对应的其他线程的完成
//-join方法：调用handle的join方法会阻止当前运行线程的执行，直到handle所表示的这些线程的终结

fn new_thread2(){
    let handle = std::thread::spawn(|| {
        for i in 1..10{
                println!("number {} from the spawned thread",i);
                std::thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();//如果在这里，就等分线程都执行完了，才去执行主线程

    for i in 1..5{
        println!("--hi number {} from the main thread",i); //主线程结束后，不管其他线程是否结束，都结束
        std::thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();//主线程要结束，被阻止，等到handle所有线程完成
}

//使用move闭包：通常和thread::spawn函数一起使用，它允许你使用其他线程的数据。创建线程时，把值的所有权从一个线程转移到另一个线程
fn new_thread3(){

    let v= vec![1,2,3];
    let handle = std::thread::spawn(move || {  //thread::spawn 分线程中的v 还没执行呢，主线程中的v可能已经被丢弃了。所以通过move取得主线程v的所有权
        println!("here is a vector:{:?}",v);
      
    });
    //drop(v);
    handle.join().unwrap();//主线程要结束，被阻止，等到handle所有线程完成
}

/**
 * 使用消息传递来跨线程传递数据：一种很流行并且能保证安全并发的技术就是：消息传递
 * 线程通过彼此发送消息来通信：要用通信来共享内存
 * rust标准库提供 channel  :channel 包含发送端和接收端，如果发送端，接收端中任意端被丢弃了，那么channel就关闭了（消息通道）
 */
//创建channel: 使用mpsc::channel来创建函数channel . mpsc表示多个生产者，一个消费者
//-返回一个tuple元组：里面元素分别是发送端，接收端

use std::sync::mpsc;
fn channel_test(){
    let (tx,rx) =  mpsc::channel();
    std::thread::spawn(move || {
        let val =String::from("hi");
        tx.send(val).unwrap();   //必须拥有通道发送端的所有权，才能往通道里面发消息。发送端的send方法，返回Result<T>,如果有错误（例如接收端已被丢弃）返回错误
    });

    let received =rx.recv().unwrap(); //recv会一直阻塞线程，直到有消息传入为止，传入的消息被包裹到Result
    println!("got {}",received);
}

//接收端的方法：
// recv方法:阻止当前线程，直到channel中有值被传进来，一旦收到值，返回Result<T,E>,当发送端关闭，就会收到一个错误
//try_recv方法：不会阻塞，立即返回Result<T,E>,有数据到达返回Ok,否则返回错误。通常会使用循环调用来检查try_recv的结果，有消息来就处理，如果还没来，就执行其他指令


//channel 和所有权转移;所有权在消息传递中非常重要：能帮你编写安全，并发的代码

fn channel_test2(){
    let (tx,rx) =  mpsc::channel();
    std::thread::spawn(move || {
        let val =String::from("hi");
        tx.send(val).unwrap();   //必须拥有通道发送端的所有权，才能往通道里面发消息。发送端的send方法，返回Result<T>,如果有错误（例如接收端已被丢弃）返回错误
        // println!("{}",val); //此处报错，借用已被移动所有权的值
    });

    

    let received =rx.recv().unwrap(); //recv会一直阻塞线程，直到有消息传入为止，传入的消息被包裹到Result
    println!("got {}",received);
}

//发送多个值，看到接受者等待
fn recv_wait(){
    let (tx,rx) = mpsc::channel();
    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            std::thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx{  //这里将接收端当做迭代器使用，就不用显示调用recv函数了
        println!("Got {}",received);
    }
}

//通过克隆创建多个发送者    
fn recv_wait_clone(){
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    std::thread::spawn(move || {
        let vals = vec![
            String::from("1:hi"),
            String::from("1:from"),
            String::from("1:the"),
            String::from("1:thread"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            std::thread::sleep(Duration::from_millis(200));
        }
    });

    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            std::thread::sleep(Duration::from_millis(200));
        }
    });


    for received in rx{  //将交替接收
        println!("Got {}",received);
    }
} 


/**
 * 下面使用共享内存的方式实现并发
 * 使用共享内存的方式实现并发：就是go语言不建议的那种方式-不要用共享内存来通信
 * rust支持共享状态来实现并发
 * channel 类似单所有权：一旦将值的所有权转移至channel,就无法使用它了。
 * 共享内存类似多所有权：多个线程可以同时访问同一块内存
 */

//使用mutex 来每次只允许一个线程来访问数据   mutex 是mutual exclusion(互斥锁)的简写-在同一时刻，mutex只允许一个线程来访问某些数据
//想要访问数据：
// -线程必须首先获取互斥锁（lock）:lock数据结构是mutex的一部分，他能跟踪谁对数据拥有独占访问权
// -mutex通常被描述为：通过锁定系统来保护它所持有的数据

// mutex两条规则：在使用数据之前，必须尝试获取锁（lock）
// 使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁

//Mutex<T>的API: 通过Mutex::new(数据)来创建Mutex<T>
//-Mutex<T>是一个智能指针
//访问数据前，通过lock方法来获取锁。
//-会阻塞当前线程 -lock可能会失败 -返回的是MutexGuard(智能指针，实现了Deref,和Drop trait )

use std::sync::Mutex;
fn mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); //这里加锁锁定，获取内部数据的引用
        *num = 6;                        //修改里面的值
    } //走完作用域的时候自动解锁，实现释放，因为lock 返回的的MutexGuard实现了Drop trait

    println!("m = {:?}",m);

}

//多线程共享Mutex<T>

// fn mutex0(){
//     let counter  = Mutex::new(0);
//     let mut handles  = vec![];

//     for _ in 0..10{
//         let handle = std::thread::spawn(move || {  //报错，上次循环counter所有权已经被移动，第二次循环时无所有权
//             let mut num  = counter.lock().unwrap();
//             *num + =1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles{
//         handle.join().unwrap();
//     }

//     println!("result :{}",*counter.lock().unwrap());
// }

//多线程多重所有权：只有实现了Send trait 的类型，才能在线程之间安全的传递（注：Rc 只适合单线程的场景，没有实现Send trait，不能用Rc）
//使用Arc<T>  原子引用计数：Arc<T> 与Rc<T>类似，Arc<T>可以用于并发情景    原子的Rc（atomic Rc） 

//为什么所有基础类型都不是原子的，为什么标准库类型不默认使用Arc<T>?  因为需要性能作为代价
//Arc<T>和Rc<T>的API是相同的
use std::sync::{Arc};
fn arc(){
    let counter  = Arc::new(Mutex::new(0));//原子引用计数 
    let mut handles = vec![];

    for _ in 0..10{
        let counter  = Arc::clone(&counter); //10个线程，创建一个线程，计数+1 执行完一次循环后计数-1
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles{ //等待每一个线程执行完毕
        handle.join().unwrap();
    }

    println!("result :{}",*counter.lock().unwrap());
}

/**
 * RefCell<T> / Rc<T> vs Mutex<T> /Arc<T>
 * 
 * Mutex<T>提供了内部可变性，和Cell家族一样
 * 我们使用RefCell<T> 来改变Rc<T>里面的内容
 * 我们使用Mutex<T> 来改变Arc<T>里面的内容
 * 注意：Mutex<T>有死锁风险：某个操作同时锁住两个资源，两个线程分别持有其中一个锁，并相互请求另外一个锁。陷入无穷无尽的等待
 */

 //通过Send 和 Sync 这两个trait来扩展并发
 //rust语言的并发特性比较少，目前讲的并发特性，都来自标准库，而不是语言本身。
 //无需局限于标准库的并发，可以自己实现并发
 //但在rust语言中有两个并发概念： std::marker:Sync 和std::marker::Send 这两个trait

 //实现 Send trait 就可以允许线程间转移所有权，rust里面几乎所有的类型都实现了Send 但Rc<T>没有实现send,它只用于单线程情景
 //任何完全由Send类型组成的类型也被标记为Send,除了原始指针之外，几乎所有的基础类型都实现了Send

 //Sync:允许从多线程访问：实现Sync trait 的类型可以安全的被多个线程使用 ，也就是说 如果T 是实现了 Sync, 那么&T 就是相当于实现了Send
 //-也就是引用可以被安全的送往另一个线程
 //-基础类型都是Sync,完全由Sync类型组成的类型也是Sync  
 //-但Rc<T>不是Sync的,RefCell<T>和Cell<T>家族也不是Sync的.而Mutex<T>是Sync的
 //手动来实现Send 和 Sync是不安全的，因为要谨慎使用不安全的代码
fn zoo(){
    let a = 6;

}

