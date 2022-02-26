use argparse::{ArgumentParser, Store};
use std::net::{TcpListener, TcpStream};
fn main() {
    /* 监听地址 */
    let mut c_listen = String::from("127.0.0.1:1080");

    /* 获取命令行参数 */
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A simple SOCKS5 proxy server");
        ap.refer(&mut c_listen)
            .add_option(&["-l", "--listen"], Store, "Listen address");
        ap.parse_args_or_exit();
    }
    println!("Listening on {}", c_listen);

    /* 创建监听器 */
    let listener = TcpListener::bind(c_listen).unwrap();

    /* 循环接收 stream */
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    if let Err(err) = handle(&stream) {
                        println!("Error {:?}", err);
                    }
                });
            }
            Err(e) => {
                println!("Listener Error: {:?}", e);
            }
        }
    }
}

/* 处理 stream */
fn handle(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("src {}", stream.peer_addr()?);
    Ok(())
}
