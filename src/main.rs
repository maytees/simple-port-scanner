use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::str::FromStr;

const MAX_PORT: u16 = 65535;

fn scan_ports(ip: String) {
    let ipaddr = match IpAddr::from_str(ip.as_str()) {
        Ok(val) => val,
        Err(e) => panic!("Error parsing Ip Address: {}", e),
    };

    let mut curport = 0;

    while curport != MAX_PORT {
        let addr = SocketAddr::new(ipaddr, curport);

        match TcpStream::connect(addr) {
            Ok(_) => {
                println!("Port - {} is open on Ip Addr - {}", curport, ip);
            }
            Err(_) => (),
        }

        curport += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide arg: IpAdress")
    }

    match args.get(1) {
        Some(addr) => scan_ports(addr.clone()),
        None => (),
    }
}
