
// 引用类库 io、net 用来完成TCP监听读取，
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
// 引入 thread 类库用来多线程处理
use std::{thread};
// 引入 str 库，用来转换输入的 buf 到 str 类型。

fn main(){
    // 创建一个监听，定义一个ip，代码执行后可以通过命令“telnet localhost 8886”进行验证端口.
    let listener = TcpListener::bind("127.0.0.1:8886").unwrap();
    // 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)
    for stream in listener.incoming() {
        //可以用telnet接入，接入后就会输出打印新连接接入
        println!("有新连接接入");
        // rust官方文档：https://doc.rust-lang.org/1.0.0/std/net/struct.AddrParseError.html
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            //连接失败会输出打印失败原因
            Err(e) => { 
                panic!("错误 {:?}", e)
            }
        }
    }

    // 关闭连接
    drop(listener);
    //上面调用的函数正确则ok，错误则返回
}

// 线程调用的处理函数
fn handle_client(mut stream: TcpStream) {
    // 定义一个存储数组，可变的
    let mut buf = [0; 512];
    // 建立循环，读取接入信息
    loop {
        // 使用read方法
        let bytes_read = stream.read(&mut buf).expect("出现错误，中断程序");
        // 输出调试信息
        println!("调试信息: {}", bytes_read);
        // 若输入流的字符长度是0
        if bytes_read == 0 {
            // 直接退出循环
            break;
        }
        //否则则直接写回buf
        stream.write(&buf[..bytes_read]).err();
    }
}