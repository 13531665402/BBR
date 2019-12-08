use std::net::UdpSocket;
use std::env;
use std::process;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main(){
    let mut arguments = Vec::new();
    for argument in env::args(){
        arguments.push(argument);
    }
    if arguments.len() != 4 {
        println!("Usage ./p2p <BindPort> <ListenPort> <time>");
        process::exit(0);
    }
    let bindPort: i32 = i32::from_str(&arguments[1]).expect("Usage ./p2p <BindPort> <ListenPort> <time>");
    let ListenPort: i32 = i32::from_str(&arguments[2]).expect("Usage ./p2p <BindPort> <ListenPort> <time>");
    let time: u32 = u32::from_str(&arguments[3]).expect("Usage ./p2p <BindPort> <ListenPort> <time>");
    let bindAddr = format!("127.0.0.1:{}", bindPort);
    let listenAddr = format!("127.0.0.1:{}", ListenPort);
    println!("Configuration Bind:{} Listen:{} TimeInterval:{}", bindAddr, listenAddr, time);

    // Mode: Listen and forward immediately
    let ListenSocket = UdpSocket::bind(listenAddr).unwrap();
    let mut SentBuf = [0; 1024];
    let mut count = 1;
    let BindSocket = UdpSocket::bind(bindAddr).unwrap();
    let mut buf = [0; 1024];
    println!("Setup!");
    loop { 
        let bindAddr = format!("127.0.0.1:{}", bindPort);
        let listenAddr = format!("127.0.0.1:{}", ListenPort);
        let listenAddr_2 = format!("127.0.0.1:{}", ListenPort);
        let (amt, src) = BindSocket.recv_from(&mut buf).unwrap();
        println!("Received {} bytes from: {:?}", amt, src);
        println!("Received Text: ");
        for e in buf.iter() {
            print!("{}", e);
        }
        println!("===================");
        SentBuf[0] = 1;
        SentBuf[2] = 1;
        SentBuf[4] = 2;
        SentBuf[5] = 1;
        BindSocket.send_to(&SentBuf, src).unwrap();
        count = count + 1;
        println!("Sent to: {}", src);
        for e in SentBuf.iter() {
            print!("{}", e);
        }
        println!("===================");
        thread::sleep(Duration::new(0, time * 1000));
    }
}