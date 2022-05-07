#![allow(non_snake_case)]

use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket},
    time::{SystemTime, UNIX_EPOCH},
};

use mac_address::get_mac_address;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    boolean_NONE: bool,
    character_OCT: char,
    character_HEX: char,
    uint8_DEC: u8,
    uint8_OCT: u8,
    uint8_HEX: u8,
    uint8_DEC_HEX: u8,
    uint8_HEX_DEC: u8,
    uint16_DEC: u16,
    uint16_OCT: u16,
    uint16_HEX: u16,
    uint16_DEC_HEX: u16,
    uint16_HEX_DEC: u16,
    uint32_DEC: u32,
    uint32_OCT: u32,
    uint32_HEX: u32,
    uint32_DEC_HEX: u32,
    uint32_HEX_DEC: u32,
    uint64_DEC: u64,
    uint64_OCT: u64,
    uint64_HEX: u64,
    uint64_DEC_HEX: u64,
    uint64_HEX_DEC: u64,
    int8_DEC: i8,
    int16_DEC: i16,
    int32_DEC: i32,
    int64_DEC: i64,
    float_NONE: f32,
    double_NONE: f64,
    absolute_time_LOCAL: u64,
    absolute_time_UTC: u64,
    absolute_time_DOY_UTC: u64,
    relative_time_NONE: u64,
    bytes_NONE: [i8; 10],
    bytes_DOT: [i8; 10],
    bytes_DASH: [i8; 10],
    bytes_COLON: [i8; 10],
    bytes_SPACE: [i8; 10],
    none_NONE: [u8; 3],
    ipv4_NONE: [u8; 4],
    ipv6_NONE: [u8; 16],
    ether_NONE: [u8; 6],
    guid_NONE: [u8; 16],
    oid_NONE: [u8; 16],
}

fn main() -> std::io::Result<()> {
    let socket_src = UdpSocket::bind("127.0.0.1:60001")?;
    let socket_des = SocketAddr::from(([127, 0, 0, 1], 60002));

    // ----- SPECIFIC BEGIN -----
    let time_as_sec = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut ipv4_le = Ipv4Addr::LOCALHOST.octets();
    ipv4_le.reverse();

    let ipv6_le = Ipv6Addr::LOCALHOST.octets();

    let mac_address = get_mac_address()
        .unwrap()
        .expect("Failed to get MAC address")
        .bytes();

    let payload = Data {
        boolean_NONE: true,
        character_OCT: 'a',
        character_HEX: 'Z',
        uint8_DEC: rand::random(),
        uint8_OCT: rand::random(),
        uint8_HEX: rand::random(),
        uint8_DEC_HEX: rand::random(),
        uint8_HEX_DEC: rand::random(),
        uint16_DEC: rand::random(),
        uint16_OCT: rand::random(),
        uint16_HEX: rand::random(),
        uint16_DEC_HEX: rand::random(),
        uint16_HEX_DEC: rand::random(),
        uint32_DEC: rand::random(),
        uint32_OCT: rand::random(),
        uint32_HEX: rand::random(),
        uint32_DEC_HEX: rand::random(),
        uint32_HEX_DEC: rand::random(),
        uint64_DEC: rand::random(),
        uint64_OCT: rand::random(),
        uint64_HEX: rand::random(),
        uint64_DEC_HEX: rand::random(),
        uint64_HEX_DEC: rand::random(),
        int8_DEC: rand::random(),
        int16_DEC: rand::random(),
        int32_DEC: rand::random(),
        int64_DEC: rand::random(),
        float_NONE: rand::random(),
        double_NONE: rand::random(),
        absolute_time_LOCAL: time_as_sec,
        absolute_time_UTC: time_as_sec,
        absolute_time_DOY_UTC: time_as_sec,
        relative_time_NONE: time_as_sec,
        bytes_NONE:  rand::random(),
        bytes_DOT:   rand::random(),
        bytes_DASH:  rand::random(),
        bytes_COLON: rand::random(),
        bytes_SPACE: rand::random(),
        ipv4_NONE: ipv4_le,
        ipv6_NONE: ipv6_le,
        ether_NONE: mac_address,
        guid_NONE: *Uuid::new_v4().as_bytes(),
        oid_NONE: rand::random(),
        none_NONE: rand::random(),
    };

    // ----- SPECIFIC END -----

    let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();

    socket_src.send_to(&encoded, &socket_des)?;

    Ok(())
}
