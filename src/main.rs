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
    uint24_DEC: [u8; 3],
    uint24_OCT: [u8; 3],
    uint24_HEX: [u8; 3],
    uint24_DEC_HEX: [u8; 3],
    uint24_DEC_DEC: [u8; 3],
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
    int24_DEC: [i8; 3],
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
    answer_to_universe: u8,
    temperature: i32
}

fn main() -> std::io::Result<()> {
    let socket_src = UdpSocket::bind("127.0.0.1:60001")?;
    let socket_des = SocketAddr::from(([127, 0, 0, 1], 60002));

    // ----- SPECIFIC BEGIN -----
    let time_as_sec = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut ipv4_le = Ipv4Addr::new(127, 0, 0, 1).octets();
    ipv4_le.reverse();

    let ipv6_le = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).octets();

    let mac_address = get_mac_address()
        .unwrap()
        .expect("Failed to get MAC address")
        .bytes();

    let uint8_sample: u8 = rand::random();
    let uint16_sample: u16 = rand::random();
    let uint24_sample: [u8; 3] = [rand::random(); 3];
    let uint32_sample: u32 = rand::random();
    let uint64_sample: u64 = rand::random();

    let int8_sample: i8 = rand::random();
    let int16_sample: i16 = rand::random();
    let int24_sample: [i8; 3] = [rand::random(); 3];
    let int32_sample: i32 = rand::random();
    let int64_sample: i64 = rand::random();

    let float_sample: f32 = rand::random();
    let double_sample: f64 = rand::random();

    let bytes_samples: [i8; 10] = rand::random();

    let payload = Data {
        boolean_NONE: true,
        character_OCT: 'a',
        character_HEX: 'Z',
        uint8_DEC: uint8_sample,
        uint8_OCT: uint8_sample,
        uint8_HEX: uint8_sample,
        uint8_DEC_HEX: uint8_sample,
        uint8_HEX_DEC: uint8_sample,
        uint16_DEC: uint16_sample,
        uint16_OCT: uint16_sample,
        uint16_HEX: uint16_sample,
        uint16_DEC_HEX: uint16_sample,
        uint16_HEX_DEC: uint16_sample,
        uint24_DEC: uint24_sample,
        uint24_OCT: uint24_sample,
        uint24_HEX: uint24_sample,
        uint24_DEC_HEX: uint24_sample,
        uint24_DEC_DEC: uint24_sample,
        uint32_DEC: uint32_sample,
        uint32_OCT: uint32_sample,
        uint32_HEX: uint32_sample,
        uint32_DEC_HEX: uint32_sample,
        uint32_HEX_DEC: uint32_sample,
        uint64_DEC: uint64_sample,
        uint64_OCT: uint64_sample,
        uint64_HEX: uint64_sample,
        uint64_DEC_HEX: uint64_sample,
        uint64_HEX_DEC: uint64_sample,
        int8_DEC: int8_sample,
        int16_DEC: int16_sample,
        int24_DEC: int24_sample,
        int32_DEC: int32_sample,
        int64_DEC: int64_sample,
        float_NONE: float_sample,
        double_NONE: double_sample,
        absolute_time_LOCAL: time_as_sec,
        absolute_time_UTC: time_as_sec,
        absolute_time_DOY_UTC: time_as_sec,
        relative_time_NONE: time_as_sec,
        bytes_NONE:  bytes_samples,
        bytes_DOT:   bytes_samples,
        bytes_DASH:  bytes_samples,
        bytes_COLON: bytes_samples,
        bytes_SPACE: bytes_samples,
        ipv4_NONE: ipv4_le,
        ipv6_NONE: ipv6_le,
        ether_NONE: mac_address,
        guid_NONE: *Uuid::new_v4().as_bytes(),
        oid_NONE: rand::random(),
        none_NONE: rand::random(),
        answer_to_universe: 42,
        temperature: -273
    };

    // ----- SPECIFIC END -----

    let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();

    socket_src.send_to(&encoded, &socket_des)?;

    Ok(())
}
