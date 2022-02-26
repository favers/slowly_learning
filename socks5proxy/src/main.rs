use argparse::{ArgumentParser, Store};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
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
    let mut src_reader = stream.try_clone()?;
    let mut src_writer = stream.try_clone()?;
    let mut buf: Vec<u8> = vec![0x00; 256];
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x05 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    let nauth = buf[0] as usize;
    src_reader.read_exact(&mut buf[0..nauth])?;
    // buf[0..nauth] must contains 0x00
    // src_writer.write_u8(0x05)?;
    // src_writer.write_u8(0x00)?;
    src_writer.write([0x05].as_ref())?;
    src_writer.write([0x00].as_ref())?;

    println!("greeting done");

    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x05 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x01 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x00 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    let host = match buf[0] {
        0x01 => {
            src_reader.read_exact(&mut buf[0..4])?;
            Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]).to_string()
        }
        0x03 => {
            src_reader.read_exact(&mut buf[0..1])?;
            let len = buf[0] as usize;
            src_reader.read_exact(&mut buf[0..len])?;
            String::from_utf8_lossy(&buf[0..len]).to_string()
        }
        0x04 => {
            src_reader.read_exact(&mut buf[0..16])?;
            Ipv6Addr::new(
                ((buf[0x00] as u16) << 8) | (buf[0x01] as u16),
                ((buf[0x02] as u16) << 8) | (buf[0x03] as u16),
                ((buf[0x04] as u16) << 8) | (buf[0x05] as u16),
                ((buf[0x06] as u16) << 8) | (buf[0x07] as u16),
                ((buf[0x08] as u16) << 8) | (buf[0x09] as u16),
                ((buf[0x0a] as u16) << 8) | (buf[0x0b] as u16),
                ((buf[0x0c] as u16) << 8) | (buf[0x0d] as u16),
                ((buf[0x0e] as u16) << 8) | (buf[0x0f] as u16),
            )
            .to_string()
        }
        _ => panic!("unreachable"),
    };
    src_reader.read_exact(&mut buf[0..2])?;
    let port = (buf[0] as u16) << 8 | (buf[1] as u16);
    let dst = format!("{}:{}", host, port);

    let dst_stream = TcpStream::connect(&dst)?;
    let mut dst_reader = dst_stream.try_clone()?;
    let mut dst_writer = dst_stream.try_clone()?;

    src_writer.write([0x05].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x01].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;
    src_writer.write([0x00].as_ref())?;

    std::thread::spawn(move || {
        std::io::copy(&mut src_reader, &mut dst_writer).ok();
    });
    std::io::copy(&mut dst_reader, &mut src_writer).ok();

    Ok(())
}
