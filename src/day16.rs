use nom::bits::complete::{tag, take};
use nom::branch::alt;
use nom::combinator::flat_map;
use nom::multi::{count, many_till};
use nom::sequence::preceded;
use nom::{bits, combinator::map, sequence::tuple, IResult};

pub fn solve() -> (usize, usize) {
    let packet = Packet::from_str(include_str!("../resources/day16.txt"));
    (solve1(&packet), solve2(&packet))
}

fn solve1(pkt: &Packet) -> usize {
    pkt.count_versions()
}

fn solve2(pkt: &Packet) -> usize {
    pkt.interpret()
}

#[derive(Debug, PartialEq, Eq)]
enum OpType {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal(usize),
    Operator(OpType, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    content: PacketType,
}

impl Packet {
    fn from_str(raw: &str) -> Self {
        let bytes = hex::decode(raw.trim_end()).unwrap();
        parse_packet_nom(&bytes).unwrap().1
    }

    fn count_versions(&self) -> usize {
        self.version as usize
            + match &self.content {
                PacketType::Literal(_) => 0,
                PacketType::Operator(_, sub_pkts) => {
                    sub_pkts.iter().map(|p| p.count_versions()).sum()
                }
            }
    }

    fn interpret(&self) -> usize {
        match &self.content {
            PacketType::Literal(l) => *l,
            PacketType::Operator(code, subs) => match code {
                OpType::Sum => subs.iter().map(|p| p.interpret()).sum(),
                OpType::Product => subs.iter().map(|p| p.interpret()).product(),
                OpType::Min => subs.iter().map(|p| p.interpret()).min().unwrap(),
                OpType::Max => subs.iter().map(|p| p.interpret()).max().unwrap(),
                OpType::Gt => {
                    if subs[0].interpret() > subs[1].interpret() {
                        1
                    } else {
                        0
                    }
                }
                OpType::Lt => {
                    if subs[0].interpret() < subs[1].interpret() {
                        1
                    } else {
                        0
                    }
                }
                OpType::Eq => {
                    if subs[0].interpret() == subs[1].interpret() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

//having this fn helps with the type inference
fn parse_packet_nom(raw: &[u8]) -> IResult<&[u8], Packet> {
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(parse_bits)(raw)
}

fn parse_bits(raw: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let parse_literal = map(
        preceded(
            tag(0b100, 3usize),
            many_till(
                preceded(tag(0b1, 1usize), take(4usize)),
                preceded(tag(0b0, 1usize), take(4usize)),
            ),
        ),
        |(xs, x): (Vec<usize>, usize)| {
            let mut n = xs.into_iter().fold(0usize, |acc, x| (acc << 4) + x);
            n = (n << 4) + x;
            PacketType::Literal(n)
        },
    );

    let parse_operator = map(
        tuple((
            parse_optype,
            flat_map(take(1usize), |mode: u8| {
                if mode == 0 {
                    Box::new(flat_map(take(15usize), parse_packet_len))
                        as Box<dyn nom::Parser<_, _, _>>
                } else {
                    Box::new(flat_map(take(11usize), |n: usize| count(parse_bits, n)))
                        as Box<dyn nom::Parser<_, _, _>>
                }
            }),
        )),
        |(opcode, content)| PacketType::Operator(opcode, content),
    );

    map(
        tuple((take(3usize), alt((parse_literal, parse_operator)))),
        |(version, content)| Packet { version, content },
    )(raw)
}

// parse some number of packets, consuming `l` bits
fn parse_packet_len(l: usize) -> impl Fn((&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    move |sub_bits| {
        // unfortunately, nom::combinator::consumed doesn't work
        // for bits, so need to handrol the length counting
        let mut sub_packets = vec![];
        let mut input = sub_bits;
        while input_len(sub_bits) - input_len(input) < l {
            let (rest, pkt) = parse_bits(input)?;
            sub_packets.push(pkt);
            input = rest;
        }

        Ok((input, sub_packets))
    }
}

fn parse_optype(raw: (&[u8], usize)) -> IResult<(&[u8], usize), OpType> {
    use OpType::*;
    map(take(3usize), |c: u8| match c {
        0 => Sum,
        1 => Product,
        2 => Min,
        3 => Max,
        5 => Gt,
        6 => Lt,
        7 => Eq,
        _ => unreachable!("{}", c),
    })(raw)
}

fn input_len(raw: (&[u8], usize)) -> usize {
    raw.0.len() * 8 - raw.1
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_packet_literal() {
        let expected = Packet {
            version: 6,
            content: PacketType::Literal(2021),
        };
        assert_eq!(Packet::from_str("D2FE28"), expected);
    }

    #[test]
    fn test_parse_packet_op_0() {
        let expected = Packet {
            version: 1,
            content: PacketType::Operator(
                OpType::Lt,
                vec![
                    Packet {
                        version: 6,
                        content: PacketType::Literal(10),
                    },
                    Packet {
                        version: 2,
                        content: PacketType::Literal(20),
                    },
                ],
            ),
        };
        assert_eq!(Packet::from_str("38006F45291200"), expected);
    }

    #[test]
    fn test_parse_packet_op_1() {
        let expected = Packet {
            version: 7,
            content: PacketType::Operator(
                OpType::Max,
                vec![
                    Packet {
                        version: 2,
                        content: PacketType::Literal(1),
                    },
                    Packet {
                        version: 4,
                        content: PacketType::Literal(2),
                    },
                    Packet {
                        version: 1,
                        content: PacketType::Literal(3),
                    },
                ],
            ),
        };
        assert_eq!(Packet::from_str("EE00D40C823060"), expected);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(16, solve1(&Packet::from_str("8A004A801A8002F478")));
        assert_eq!(12, solve1(&Packet::from_str("620080001611562C8802118E34")));
        assert_eq!(
            23,
            solve1(&Packet::from_str("C0015000016115A2E0802F182340"))
        );
        assert_eq!(
            31,
            solve1(&Packet::from_str("A0016C880162017C3686B18A3D4780"))
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(3, solve2(&Packet::from_str("C200B40A82")));
        assert_eq!(54, solve2(&Packet::from_str("04005AC33890")));
        assert_eq!(7, solve2(&Packet::from_str("880086C3E88112")));
        assert_eq!(9, solve2(&Packet::from_str("CE00C43D881120")));
        assert_eq!(1, solve2(&Packet::from_str("D8005AC2A8F0")));
        assert_eq!(0, solve2(&Packet::from_str("F600BC2D8F")));
        assert_eq!(0, solve2(&Packet::from_str("9C005AC2F8F0")));
        assert_eq!(1, solve2(&Packet::from_str("9C0141080250320F1802104A08")));
    }
}
