use anyhow::{anyhow, Result};
use std::path::Path;

use nom::bits::{bits, complete::tag, complete::take};
use nom::branch::alt;
use nom::combinator::{eof, flat_map, map, map_res, opt};
use nom::multi::{length_count, many0, many1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}

fn from_hex(c: char) -> Result<u8> {
    c.to_digit(16)
        .map(|b| b as u8)
        .ok_or_else(|| anyhow!("{} is not a valid hex character", c))
}

fn decode_bool(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    map(take(1usize), |b: u8| b != 0)(input)
}

#[derive(Debug, Clone, Copy)]
struct VarInt(u128);

impl VarInt {
    fn decode_bits(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let mut out = 0;
        loop {
            let (i, (has_more, num)): (_, (bool, u128)) = pair(decode_bool, take(4usize))(input)?;
            input = i;

            out <<= 4;
            out |= num;

            if !has_more {
                break;
            }
        }
        Ok((input, Self(out)))
    }
}

#[derive(Debug, Clone)]
enum PacketType {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(VarInt),
    GreaterThan(Box<(Packet, Packet)>),
    LessThan(Box<(Packet, Packet)>),
    EqualTo(Box<(Packet, Packet)>),
}

impl PacketType {
    fn decode_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        alt((
            preceded(tag(0, 3usize), map(Packet::decode_inner_packets, Self::Sum)),
            preceded(
                tag(1, 3usize),
                map(Packet::decode_inner_packets, Self::Product),
            ),
            preceded(
                tag(2, 3usize),
                map(Packet::decode_inner_packets, Self::Minimum),
            ),
            preceded(
                tag(3, 3usize),
                map(Packet::decode_inner_packets, Self::Maximum),
            ),
            preceded(tag(4, 3usize), map(VarInt::decode_bits, Self::Literal)),
            preceded(
                tag(5, 3usize),
                map_res(Packet::decode_inner_packets, |p| {
                    if p.len() != 2 {
                        return Err(anyhow!("Package type only supports 2 sub-packets"));
                    }
                    Ok(Self::GreaterThan(Box::new((p[0].clone(), p[1].clone()))))
                }),
            ),
            preceded(
                tag(6, 3usize),
                map_res(Packet::decode_inner_packets, |p| {
                    if p.len() != 2 {
                        return Err(anyhow!("Package type only supports 2 sub-packets"));
                    }
                    Ok(Self::LessThan(Box::new((p[0].clone(), p[1].clone()))))
                }),
            ),
            preceded(
                tag(7, 3usize),
                map_res(Packet::decode_inner_packets, |p| {
                    if p.len() != 2 {
                        return Err(anyhow!("Package type only supports 2 sub-packets"));
                    }
                    Ok(Self::EqualTo(Box::new((p[0].clone(), p[1].clone()))))
                }),
            ),
        ))(input)
    }
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    body: PacketType,
}

impl Packet {
    fn decode_inner_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Self>> {
        alt((
            preceded(
                tag(0, 1usize),
                map_res(
                    flat_map(take(15usize), |num_bits: u16| {
                        move |(input_bytes, offset)| {
                            let mut input = (input_bytes, offset);
                            let mut subpacket = Vec::new();

                            // Extract full bytes
                            for _ in 0..(num_bits / 8) {
                                let (i, byte) = take(8usize)(input)?;
                                subpacket.push(byte);
                                input = i;
                            }

                            // Extract last byte
                            let rem = num_bits % 8;
                            if rem > 0 {
                                let (i, byte): (_, u8) = take(rem)(input)?;
                                subpacket.push(byte << (8 - rem)); // Get rid of top level zeros
                                input = i;
                            }

                            Ok((input, subpacket))
                        }
                    }),
                    |t| -> Result<Vec<Self>> {
                        bits(terminated(
                            many1(Self::decode_bits),
                            pair(opt(many0(tag(0, 1usize))), eof),
                        ))(&t)
                        .map(|(_, packets)| packets)
                        .map_err(|_: nom::Err<nom::error::Error<&[u8]>>| {
                            anyhow!("Failed to decode subpacket")
                        })
                    },
                ),
            ),
            preceded(
                tag(1, 1usize),
                length_count(take::<_, u16, _, _>(11usize), Packet::decode_bits),
            ),
        ))(input)
    }

    fn decode_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, (version, body)) = tuple((take(3usize), PacketType::decode_bits))(input)?;
        Ok((input, Self { version, body }))
    }

    fn decode(input: &[u8]) -> Result<Packet, nom::Err<nom::error::Error<Vec<u8>>>> {
        bits(terminated(
            Self::decode_bits,
            pair(opt(many0(tag(0, 1usize))), eof),
        ))(input)
        .map(|(_, packets)| packets)
        .map_err(|e: nom::Err<nom::error::Error<&[u8]>>| e.to_owned())
    }
}

fn part_a(packet: &Packet) -> usize {
    usize::from(packet.version)
        + match &packet.body {
            PacketType::Sum(sp)
            | PacketType::Product(sp)
            | PacketType::Minimum(sp)
            | PacketType::Maximum(sp) => sp.iter().map(part_a).sum(),
            PacketType::Literal(_) => 0,
            PacketType::GreaterThan(op) | PacketType::LessThan(op) | PacketType::EqualTo(op) => {
                part_a(&op.0) + part_a(&op.1)
            }
        }
}

fn part_b(packet: &Packet) -> u128 {
    match &packet.body {
        PacketType::Sum(sp) => sp.iter().map(part_b).sum(),
        PacketType::Product(sp) => sp.iter().map(part_b).product(),
        PacketType::Minimum(sp) => sp.iter().map(part_b).min().unwrap(),
        PacketType::Maximum(sp) => sp.iter().map(part_b).max().unwrap(),
        PacketType::Literal(VarInt(v)) => *v,
        PacketType::GreaterThan(op) => {
            if part_b(&op.0) > part_b(&op.1) {
                1
            } else {
                0
            }
        }
        PacketType::LessThan(op) => {
            if part_b(&op.0) < part_b(&op.1) {
                1
            } else {
                0
            }
        }
        PacketType::EqualTo(op) => {
            if part_b(&op.0) == part_b(&op.1) {
                1
            } else {
                0
            }
        }
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<u128>)> {
    let hex_string = std::fs::read_to_string(path)?;
    let bytes = hex_string
        .chars()
        .step_by(2)
        .zip(hex_string.chars().skip(1).step_by(2))
        .map(|(high, low)| Ok((from_hex(high)? << 4) | from_hex(low)?))
        .collect::<Result<Vec<_>>>()?;

    let packet = Packet::decode(&bytes)?;
    Ok((part_a(&packet), Some(part_b(&packet))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(
            part_a(&Packet::decode(&[
                0x8a, 0x00, 0x4a, 0x80, 0x1a, 0x80, 0x02, 0xf4, 0x78
            ])?),
            16,
        );
        assert_eq!(
            part_a(&Packet::decode(&[
                0x62, 0x00, 0x80, 0x00, 0x16, 0x11, 0x56, 0x2c, 0x88, 0x02, 0x11, 0x8e, 0x34,
            ])?),
            12,
        );
        assert_eq!(
            part_a(&Packet::decode(&[
                0xc0, 0x01, 0x50, 0x00, 0x01, 0x61, 0x15, 0xa2, 0xe0, 0x80, 0x2f, 0x18, 0x23, 0x40,
            ])?),
            23,
        );
        assert_eq!(
            part_a(&Packet::decode(&[
                0xa0, 0x01, 0x6c, 0x88, 0x01, 0x62, 0x01, 0x7c, 0x36, 0x86, 0xb1, 0x8a, 0x3d, 0x47,
                0x80,
            ])?),
            31,
        );
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&Packet::decode(&[0xc2, 0x00, 0xb4, 0x0a, 0x82])?), 3);
        assert_eq!(
            part_b(&Packet::decode(&[0x04, 0x00, 0x5a, 0xc3, 0x38, 0x90])?),
            54
        );
        assert_eq!(
            part_b(&Packet::decode(&[
                0x88, 0x00, 0x86, 0xc3, 0xe8, 0x81, 0x12
            ])?),
            7
        );
        assert_eq!(
            part_b(&Packet::decode(&[
                0xce, 0x00, 0xc4, 0x3d, 0x88, 0x11, 0x20
            ])?),
            9
        );
        assert_eq!(
            part_b(&Packet::decode(&[0xd8, 0x00, 0x5a, 0xc2, 0xa8, 0xf0])?),
            1
        );
        assert_eq!(part_b(&Packet::decode(&[0xf6, 0x00, 0xbc, 0x2d, 0x8f])?), 0);
        assert_eq!(
            part_b(&Packet::decode(&[0x9c, 0x00, 0x5a, 0xc2, 0xf8, 0xf0])?),
            0
        );
        assert_eq!(
            part_b(&Packet::decode(&[
                0x9c, 0x01, 0x41, 0x08, 0x02, 0x50, 0x32, 0x0f, 0x18, 0x02, 0x10, 0x4a, 0x08
            ])?),
            1
        );
        Ok(())
    }
}
