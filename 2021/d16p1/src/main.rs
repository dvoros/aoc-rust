
#![feature(array_chunks)]

struct bits<'a> {
    value: &'a[u8],
    pos: usize,
}

impl<'a> bits<'a> {
    fn from_bytes(bytes: &'a[u8]) -> Self {
        Self { value: bytes, pos: 0 }
    }

    fn take(&mut self, mut size: usize) -> usize {
        let mut res = 0;
        while size > 0 {
            let curr = self.pos / 8;
            let remaining = 8 - (self.pos % 8);
            let target = size.min(remaining);
            
            res <<= target;
            res |= ((self.value[curr] & (u8::MAX >> (self.pos % 8))) >> (remaining - target)) as usize;
            size -= target;
            self.pos += target;
        }
        res
    }

    fn take_literal(&mut self) -> usize {
        let mut res = 0;
        loop {
            let next5 = self.take(5);
            res = res << 4 | next5 & (usize::MAX >> usize::BITS - 4);
            if next5 & (1 << 4) == 0 {
                return res
            }
        }
    }
}

#[derive(Debug)]
struct Packet<'a> {
    version: usize,
    packet_type: PacketType<'a>,
    length: usize,
}

#[derive(Debug)]
struct OperatorValue<'a> {
    type_id: usize,
    sub_packets: &'a [Packet<'a>]
}

#[derive(Debug)]
enum PacketType<'a> {
    Literal(usize),
    Operator(OperatorValue<'a>)
}

fn parse_packet<'a>(b: &'a mut bits) -> Packet<'a> {
    let start_pos = b.pos;
    let version = b.take(3);
    let type_id = b.take(3);
    if type_id == 4 {
        return Packet {
            version,
            packet_type: PacketType::Literal(b.take_literal()),
            length: b.pos - start_pos,
        }
    } else {
        let length_type_id = b.take(1);
        let mut sub_packets = Vec::new();
        if length_type_id == 0 {
            let mut total_length = b.take(15);
            while total_length > 0 {
                let sub_packet = parse_packet(b);
                total_length -= sub_packet.length;
                sub_packets.push(sub_packet);
            }
        } else {
            panic!("unimplemented")
        }
        let length = b.pos - start_pos;
        return Packet {
            version,
            packet_type: PacketType::Operator(
                OperatorValue {
                    type_id: type_id,
                    sub_packets: &sub_packets,
                }),
            length,
        }
    }
}

fn main() {
    let bytes: Vec<_> = include_str!("../test_input1")
        .trim()
        .as_bytes()
        .array_chunks()
        .map(|&[a, b]| {
            (((a as char).to_digit(16).unwrap() as u8) << 4)
            | (b as char).to_digit(16).unwrap() as u8
        }).collect();
    println!("{:?}", bytes);

    let mut b = bits::from_bytes(&bytes);
    let p = parse_packet(&mut b);
    println!("{:?}", p);
}