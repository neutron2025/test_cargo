
///构建多线程web服务器
/// -在socket上监听TCP链接
/// -解析少量的HTTP请求
/// -创建一个合适的HTTP响应
/// -使用线程池改进服务器的吞吐量
/// 注：不是最佳实践，只是复习知识和开发思路


use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use multithread20::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2){
        let stream =stream.unwrap();

        // std::thread::spawn(move || {
        //     handle_connection(stream);
        // })


        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("shutting down");

}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line,contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}