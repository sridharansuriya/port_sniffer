use clap::{command, Parser};

use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::{io, io::Write, thread, vec};

const MAX_PORT: u32 = 65535;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, default_value_t = 64)]
    job: u16,
    ip: String,
}
fn main() {
    let args = Args::parse();
    let ip = IpAddr::from_str(&args.ip).expect("Expected a valid ip address");
    let (tx, rx) = channel::<Option<u32>>();
    let mut port = 0;

    loop {
        if port > MAX_PORT {
            break;
        }
        let mut threads = vec![];
        for i in 0..args.job {
            threads.push(worker(i, ip.to_string(), port, tx.clone()));
            port += 1;
        }
        for t in threads {
            t.join().unwrap();
        }
    }
    drop(tx);
    let mut open_ports = vec![];
    for port in rx.iter().flatten() {
        open_ports.push(port);
    }
    open_ports.sort();
    println!();
    for port in open_ports {
        println!("port {} is open", port);
    }
}

fn worker(
    _id: u16,
    mut addr: String,
    port: u32,
    tx: Sender<Option<u32>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        addr.push(':');
        addr.push_str(&port.to_string());
        if TcpStream::connect(&addr).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(Some(port)).unwrap();
        }
    })
}
