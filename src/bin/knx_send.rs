// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use knx_rs::address::Address;

use serial;

use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use serial::prelude::*;

use std::time::Duration;
use std::str::FromStr;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud19200,
    char_size: serial::Bits8,
    parity: serial::ParityEven,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// ------------------------------------------------------------------------------
fn main() {
    let parms = App::new("knx_send")
        .version(crate_version!())
        .about("send KNX over serial/multicast")
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
        .arg(Arg::with_name("knx_address")
                .required(true)
                .index(1)
                .help("knx group address"),
        )
        .arg(Arg::with_name("knx_data")
                .required(true)
                .index(2)
                .help("knx data"),
        )
        .get_matches();

    let knx_address = parms.value_of("knx_address").unwrap();
    let knx_address = Address::from_str(knx_address).unwrap();
    let knx_data = parms.value_of("knx_data").unwrap();

    if parms.is_present("serial") {
        let serial_port = parms.value_of("serialport").unwrap();
        println!("Sending {} to {} on serial: {}", knx_data, &knx_address, serial_port);
        knx_send_serial(serial_port, &knx_address, knx_data);
    }
    if parms.is_present("multicast") {
        let multicast_address = parms.value_of("multicast_address").unwrap();
        println!("Sending {} to {} on multicast: {}", knx_data, &knx_address, multicast_address);
        knx_send_multicast(multicast_address, &knx_address, knx_data);
    }
}

// ------------------------------------------------------------------------------
fn knx_send_serial(serial_port: &str, _knx_address: &Address, _knx_data: &str) {
    let mut port = serial::open(serial_port).expect("couldn't open serial port");
    port.configure(&SETTINGS)
        .expect("couldn't set serial settings");
    port.set_timeout(Duration::from_secs(10))
        .expect("couldn't set timeout");
}

// ------------------------------------------------------------------------------
fn knx_send_multicast(multicast_address: &str, _knx_address: &Address, _knx_data: &str) {
    let knx_addr: SocketAddrV4 = multicast_address.parse().unwrap();
    let ip_any = Ipv4Addr::new(0, 0, 0, 0);

    let socket = UdpSocket::bind(&knx_addr).expect("couldn't bind to address");
    socket
        .join_multicast_v4(&knx_addr.ip(), &ip_any)
        .expect("couldn't join multicast address");
}
