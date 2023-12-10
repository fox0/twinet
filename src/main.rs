#![feature(ip_bits)]
// https://github.com/yaa110/tokio-tun/blob/master/examples/read.rs

mod ip;

use std::mem;
use std::net::Ipv4Addr;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::convert::TryFrom;

use tokio_tun::Tun;

use crate::ip::{IpType, IPv4, MTU};


#[tokio::main]
async fn main() {
    let tun = Arc::new(
        Tun::builder()
            .name("")
            .tap(false)
            .packet_info(false)
            .mtu(MTU as i32)
            .up()
            .address(Ipv4Addr::new(10, 10, 0, 1))
            .destination(Ipv4Addr::new(10, 10, 0, 0))
            .netmask(Ipv4Addr::new(255, 255, 255, 0))
            .broadcast(Ipv4Addr::BROADCAST)
            .try_build()
            .unwrap(),
    );

    println!("-----------");
    println!("tun created");
    println!("-----------");

    println!(
        "┌ name: {}\n├ fd: {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tun.name(),
        tun.as_raw_fd(),
        tun.mtu().unwrap(),
        tun.flags().unwrap(),
        tun.address().unwrap(),
        tun.destination().unwrap(),
        tun.broadcast().unwrap(),
        tun.netmask().unwrap(),
    );

    println!("----------------------");
    println!("ping 10.10.0.2 to test");
    println!("----------------------");

    // Reader
    let mut buf = [0u8; MTU];
    loop {
        let _n = tun.recv(&mut buf).await.unwrap();
        let ttt = IpType::try_from(buf[0]).unwrap();
        match ttt {
            IpType::V4 => {
                let t: IPv4 = unsafe { mem::transmute(buf) };
                t.print();
            }
        }
        // println!("reading {} bytes: {:?}", n, &buf[..n]);
    }

    // Writer: simply clone Arced Tun.
    /*
        let tun_c = tun.clone();
        tokio::spawn(async move{
            let buf = b"data to be written";
            tun_c.send_all(buf).await.unwrap();
        });
    */
}
