// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use knx_rs::helper::hex_to_string;
use knx_rs::parser::parse_cemi;

use knx_ets_rs::ets::Ets;

use serial;

use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::io::prelude::*;
use serial::prelude::*;

use std::time::Duration;
use std::io::ErrorKind;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud19200,
    char_size: serial::Bits8,
    parity: serial::ParityEven,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// ------------------------------------------------------------------------------
fn main() {
    let parms = App::new("knx_listen")
        .version(crate_version!())
        .about("listen and log KNX messages serial/multicast")
        .author(crate_authors!())
        .arg(
            Arg::with_name("serial")
                .required(true)
                .conflicts_with("multicast")
                .short("s")
                .long("serial")
                .help("use serial port"),
        )
        .arg(
            Arg::with_name("multicast")
                .required(true)
                .conflicts_with("serial")
                .short("m")
                .long("multicast")
                .help("use multicast"),
        )
        .arg(
            Arg::with_name("serialport")
                .required(false)
                .default_value("/dev/cu.usb_to_knx")
                .short("p")
                .long("serialport")
                .help("serial port device"),
        )
        .arg(
            Arg::with_name("multicast_address")
                .required(false)
                .default_value("224.0.23.12:3671")
                .short("a")
                .long("multicast_address")
                .help("multicast address for knx"),
        )
        .arg(
            Arg::with_name("opcfile")
                .required(false)
                .takes_value(true)
                .short("o")
                .long("opcfile")
                .help("OPC file exported from ETS"),
        )
        .get_matches();

    let mut ets: Option<Ets> = None;

    if parms.is_present("opcfile") {
        let opc_file = parms.value_of("opcfile").unwrap();
        ets = Some(Ets::new(opc_file));
        // ets.unwrap().print();
    } 
    if parms.is_present("serial") {
        let serial_port = parms.value_of("serialport").unwrap();
        println!("Listening on serial port: {}", serial_port);
        knx_listen_serial(serial_port);
    }
    if parms.is_present("multicast") {
        let multicast_address = parms.value_of("multicast_address").unwrap();
        println!("Listening on multicast address: {}", multicast_address);
        knx_listen_multicast(multicast_address);
    }
}


// ------------------------------------------------------------------------------
fn knx_listen_serial(serial_port: &str) {
    let mut port = serial::open(serial_port).expect("couldn't open serial port");
    port.configure(&SETTINGS)
        .expect("couldn't set serial settings");
    port.set_timeout(Duration::from_secs(10))
        .expect("couldn't set timeout");

    println!("start reading bytes");
    loop {
        let mut buf = [0; 24];
        match port.read(&mut buf) {
            Ok(nr) => {
                println!("{} -> {}", hex_to_string(&buf[0..nr]), nr);
            }
            Err(err) => {
                if err.kind() != ErrorKind::TimedOut {
                    println!(" result : {:?}", err)
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------
fn knx_listen_multicast(multicast_address: &str) {
    let knx_addr: SocketAddrV4 = multicast_address.parse().unwrap();
    let ip_any = Ipv4Addr::new(0, 0, 0, 0);

    let socket = UdpSocket::bind(&knx_addr).expect("couldn't bind to address");
    socket
        .join_multicast_v4(&knx_addr.ip(), &ip_any)
        .expect("couldn't join multicast address");

    loop {
        let mut buf = [0; 24];
        let (nr_bytes, _from) = socket.recv_from(&mut buf).expect("Didn't receive data");

        println!(
            "{} -> {}",
            hex_to_string(&buf[0..nr_bytes]),
            nr_bytes
        );
        let cemi = parse_cemi(&buf[0..nr_bytes]);
        println!("{:?}",cemi);
    }
}
