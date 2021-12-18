#[aoc_generator(day16)]
pub fn load_input(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    for line in input.lines() {
        for c in line.chars() {
            let mut hex = c.to_digit(16).unwrap();
            if hex > 7 {
                output.push(1);
            } else {
                output.push(0);
            }

            hex %= 8;
            if hex > 3 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex %= 4;
            if hex > 1 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex %= 2;
            output.push(hex as u8);
        }
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypeID {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TypeID {
    fn from_u8(bits: [u8; 3]) -> TypeID {
        if bits == [0, 0, 0] {
            TypeID::Sum
        } else if bits == [0, 0, 1] {
            TypeID::Product
        } else if bits == [0, 1, 0] {
            TypeID::Minimum
        } else if bits == [0, 1, 1] {
            TypeID::Maximum
        } else if bits == [1, 0, 0] {
            TypeID::Literal
        } else if bits == [1, 0, 1] {
            TypeID::GreaterThan
        } else if bits == [1, 1, 0] {
            TypeID::LessThan
        } else {
            TypeID::EqualTo
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum OperatorLengthTypeID {
    BitLength(u16),
    PacketLength(u16),
}

#[derive(Clone, Copy, Debug)]
enum PayloadVariant {
    Literal(u128),
    Operator(OperatorLengthTypeID),
}

impl PayloadVariant {
    fn from_u8(type_id: TypeID, bits: &[u8]) -> (usize, PayloadVariant) {
        let mut bit_cntr = 0;
        match type_id {
            TypeID::Literal => {
                let mut bits_iter = bits.iter();
                let mut hex_digits: Vec<[u8; 4]> = vec![];
                loop {
                    let more_prefix = bits_iter.next().unwrap();
                    bit_cntr += 1;
                    let mut hex = [0_u8; 4];
                    hex[0] = *bits_iter.next().unwrap();
                    hex[1] = *bits_iter.next().unwrap();
                    hex[2] = *bits_iter.next().unwrap();
                    hex[3] = *bits_iter.next().unwrap();
                    bit_cntr += 4;
                    hex_digits.push(hex);
                    if *more_prefix == 0 {
                        break;
                    }
                }

                let mut value: u128 = 0;
                for hex in hex_digits {
                    for b in hex {
                        value <<= 1;
                        if b == 1 {
                            value += 1;
                        }
                    }
                }
                (bit_cntr, PayloadVariant::Literal(value))
            }
            _ => {
                let length_type_id_bit = bits[0];
                bit_cntr += 1;
                let length = {
                    let bitvec;
                    if length_type_id_bit == 0 {
                        // Bit length
                        bitvec = bits[1..16].to_vec();
                        bit_cntr += 15;
                    } else {
                        // Sub packet length
                        bitvec = bits[1..12].to_vec();
                        bit_cntr += 11;
                    }
                    let mut value = 0;
                    for b in bitvec {
                        value <<= 1;
                        if b == 1 {
                            value += 1;
                        }
                    }
                    if length_type_id_bit == 0 {
                        OperatorLengthTypeID::BitLength(value)
                    } else {
                        OperatorLengthTypeID::PacketLength(value)
                    }
                };

                match length {
                    OperatorLengthTypeID::BitLength(bitlength) => {
                        bit_cntr += bitlength as usize;
                    }
                    OperatorLengthTypeID::PacketLength(_) => {}
                }

                (bit_cntr, PayloadVariant::Operator(length))
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: TypeID,
    payload: PayloadVariant,
    sub_packets: Vec<Packet>,
    bit_length: usize,
}

impl Packet {
    fn from_u8(bits: &[u8]) -> Packet {
        // Parse the packet version
        let mut version = 0;
        let version_bits: Vec<_> = bits[..3].to_vec();
        for b in version_bits {
            version <<= 1;
            if b == 1 {
                version += 1;
            }
        }

        // Parse the packet TypeID
        let type_id = TypeID::from_u8(bits[3..6].try_into().unwrap());

        // Common header bit length is 6
        let mut bit_length = 6;

        let (bit_count, payload) = PayloadVariant::from_u8(type_id, &bits[6..]);

        // Increment by payload bit size
        bit_length += bit_count;

        let mut sub_packets: Vec<Packet> = vec![];
        match type_id {
            TypeID::Literal => (),
            _ => {
                let mut header_bit_length = 6 + 1;
                if let PayloadVariant::Operator(length) = payload {
                    match length {
                        OperatorLengthTypeID::BitLength(numbits) => {
                            header_bit_length += 15;
                            let mut subpackets_bitlength = 0;
                            while subpackets_bitlength < numbits as usize {
                                let subpacket = Packet::from_u8(
                                    &bits[header_bit_length + subpackets_bitlength..],
                                );
                                subpackets_bitlength += subpacket.bit_length;
                                sub_packets.push(subpacket);
                            }
                        }
                        OperatorLengthTypeID::PacketLength(numpackets) => {
                            header_bit_length += 11;
                            let mut subpackets_bitlength = 0;
                            while sub_packets.len() < numpackets as usize {
                                let subpacket = Packet::from_u8(
                                    &bits[header_bit_length + subpackets_bitlength..],
                                );
                                subpackets_bitlength += subpacket.bit_length;
                                sub_packets.push(subpacket);
                            }
                            bit_length += subpackets_bitlength;
                        }
                    }
                }
            }
        }

        Packet {
            version,
            type_id,
            payload,
            sub_packets,
            bit_length,
        }
    }

    fn version_count(&self) -> usize {
        self.sub_packets
            .iter()
            .map(|p| p.version_count())
            .sum::<usize>()
            + self.version as usize
    }

    fn value(&self) -> usize {
        match self.type_id {
            TypeID::Sum => {
                return self.sub_packets.iter().map(|p| p.value()).sum::<usize>();
            }
            TypeID::Product => {
                return self
                    .sub_packets
                    .iter()
                    .map(|p| p.value())
                    .product::<usize>();
            }
            TypeID::Minimum => {
                return self.sub_packets.iter().map(|p| p.value()).min().unwrap();
            }
            TypeID::Maximum => {
                return self.sub_packets.iter().map(|p| p.value()).max().unwrap();
            }
            TypeID::Literal => {
                if let PayloadVariant::Literal(value) = self.payload {
                    value as usize
                } else {
                    panic!("Shouldn't get here");
                }
            }
            TypeID::GreaterThan => {
                if self.sub_packets[0].value() > self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            TypeID::LessThan => {
                if self.sub_packets[0].value() < self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            TypeID::EqualTo => {
                if self.sub_packets[0].value() == self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> usize {
    let top_level_packet = Packet::from_u8(input);
    top_level_packet.version_count()
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> usize {
    let top_level_packet = Packet::from_u8(input);
    top_level_packet.value()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = load_input(&String::from("D2FE28"));
        let literal_packet = Packet::from_u8(&input);
        println!("literal_packet value: {:?}", literal_packet);

        let input = load_input(&String::from("38006F45291200"));
        let operator_packet = Packet::from_u8(&input);
        println!("operator_packet: {:?}", operator_packet);

        let input = load_input(&String::from("EE00D40C823060"));
        let operator_packet = Packet::from_u8(&input);
        println!("operator_packet: {:?}", operator_packet);

        let input = read_to_string("input/2021/16a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 16);

        let input = read_to_string("input/2021/16b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 12);

        let input = read_to_string("input/2021/16c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 23);

        let input = read_to_string("input/2021/16d.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = load_input(&String::from("C200B40A82"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 3);

        let input = load_input(&String::from("04005AC33890"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 54);

        let input = load_input(&String::from("880086C3E88112"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 7);

        let input = load_input(&String::from("CE00C43D881120"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 9);

        let input = load_input(&String::from("D8005AC2A8F0"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 1);

        let input = load_input(&String::from("F600BC2D8F"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 0);

        let input = load_input(&String::from("9C005AC2F8F0"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 0);

        let input = load_input(&String::from("9C0141080250320F1802104A08"));
        let packet = Packet::from_u8(&input);
        println!("packet: {:?}", packet);
        assert_eq!(packet.value(), 1);
    }
}
