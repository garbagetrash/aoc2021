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

            hex = hex % 8;
            if hex > 3 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex = hex % 4;
            if hex > 1 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex = hex % 2;
            output.push(hex as u8);
        }
    }
    output
}

#[derive(Debug, PartialEq, Eq)]
enum TypeID {
    Other = 0,
    Literal = 4,
}

impl TypeID {
    fn from_u8(bits: [u8; 3]) -> TypeID {
        if bits == [1, 0, 0] {
            TypeID::Literal
        } else {
            TypeID::Other
        }
    }
}

trait PayloadVariant {
    fn from_u8(bits: &[u8]) -> Self;
    fn bit_length(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPayload {
    value: u128,
    bitlength: usize,
}

impl PayloadVariant for LiteralPayload {
    fn from_u8(bits: &[u8]) -> LiteralPayload {
        let mut bit_cntr = 0;
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
        println!("{:?}", hex_digits);
        for hex in hex_digits {
            for b in hex {
                value <<= 1;
                if b == 1 {
                    value += 1;
                }
            }
        }
        LiteralPayload { value: value, bitlength: bit_cntr }
    }

    fn bit_length(&self) -> usize {
        self.bitlength
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LengthTypeID {
    BitLength(u16),
    PacketLength(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorPayload<T: PayloadVariant> {
    length_type_id: LengthTypeID,
    sub_packets: Vec<Packet<T>>,
    bitlength: usize,
}

impl<T:PayloadVariant> PayloadVariant for OperatorPayload<T> {
    fn from_u8(bits: &[u8]) -> OperatorPayload<T> {
        let mut bit_cntr = 0;
        let mut bit_iter = bits.iter();
        let length_type_id_bit = bit_iter.next().unwrap();
        bit_cntr += 1;
        let length = {
            let mut bitvec = vec![];
            if *length_type_id_bit == 0 {
                // Bit length
                bitvec = bit_iter.take(15).collect();
                bit_cntr += 15;
            } else {
                // Sub packet length
                bitvec = bit_iter.take(12).collect();
                bit_cntr += 12;
            }
            let mut value = 0;
            for b in bitvec {
                value <<= 1;
                if *b == 1 {
                    value += 1;
                }
            }
            if *length_type_id_bit == 0 {
                LengthTypeID::BitLength(value)
            } else {
                LengthTypeID::PacketLength(value)
            }
        };

        let mut sub_packets = vec![];
        match length {
            LengthTypeID::BitLength(bitlength) => {
                let mut payload_length = 0;
                while payload_length < bitlength {
                    let pkt = Packet::<T>::from_u8(bit_iter.as_slice());
                    for _ in 0..pkt.bit_length() {
                        bit_iter.next();
                    }
                    bit_cntr += pkt.bit_length();
                }
            },
            LengthTypeID::PacketLength(pktlength) => {
                while sub_packets.len() < pktlength as usize {
                    let pkt = Packet::<T>::from_u8(bit_iter.as_slice());
                    for _ in 0..pkt.bit_length() {
                        bit_iter.next();
                    }
                    bit_cntr += pkt.bit_length();
                }
            },
        }

        OperatorPayload {
            length_type_id: length,
            sub_packets: sub_packets,
            bitlength: bit_cntr,
        }
    }

    fn bit_length(&self) -> usize {
        self.bitlength
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet<T: PayloadVariant> {
    version: u8,
    type_id: TypeID,
    payload: T,
}

impl<T: PayloadVariant> Packet<T> {
    fn from_u8(bits: &[u8]) -> Packet<T> {
        let mut bit_iter = bits.iter();

        // Parse the packet version
        let mut version = 0;
        let version_bits: Vec<_> = bit_iter.take(3).collect();
        for b in version_bits {
            version <<= 1;
            if *b == 1 {
                version += 1;
            }
        }

        // Parse the packet TypeID
        let type_id = TypeID::from_u8(bits[3..6].try_into().unwrap());

        let payload = PayloadVariant::from_u8(&bits[6..]);

        Packet {
            version: version,
            type_id: type_id,
            payload: payload,
        }
    }

    fn bit_length(&self) -> usize {
        self.payload.bit_length() + 6
    }
}

#[aoc(day16, part1)]
pub fn part1<T: PayloadVariant>(input: &[u8]) -> usize {
    let packets = Packet::<T>::from_u8(input);
    println!("{:?}", packets);
    0
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {

        // Check our ability to parse a single literal packet
        let input = String::from("D2FE28");
        let input = load_input(&input);
        let expected = Packet {
            version: 6,
            type_id: TypeID::Literal,
            payload: LiteralPayload {value: 2021, bitlength: 18},
        };
        assert_eq!(Packet::from_u8(&input), expected);

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
        let input = read_to_string("input/2021/16.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
