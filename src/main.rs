use std::{io::{BufReader, BufRead}, fs::File};

fn from_hex(input: &char) -> Result<Vec<bool>, ()> {

    match input {
        '0' => Ok(vec![false, false, false, false]),
        '1' => Ok(vec![false, false, false, true]),
        '2' => Ok(vec![false, false, true, false]),
        '3' => Ok(vec![false, false, true, true]),
        '4' => Ok(vec![false, true, false, false]),
        '5' => Ok(vec![false, true, false, true]),
        '6' => Ok(vec![false, true, true, false]),
        '7' => Ok(vec![false, true, true, true]),
        '8' => Ok(vec![true, false, false, false]),
        '9' => Ok(vec![true, false, false, true]),
        'A' => Ok(vec![true, false, true, false]),
        'B' => Ok(vec![true, false, true, true]),
        'C' => Ok(vec![true, true, false, false]),
        'D' => Ok(vec![true, true, false, true]),
        'E' => Ok(vec![true, true, true, false]),
        'F' => Ok(vec![true, true, true, true]),

        _ => Err(())

    }

}

#[derive(Clone, Debug)]
enum Packet {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Gt(Vec<Packet>),
    Lt(Vec<Packet>),
    Eq(Vec<Packet>)
}

enum LengthType {
    Bytes,
    Packets
}

fn parse_packet(input: &[bool]) -> (usize, Packet) {

    let type_part = to_number(&input[3..6]);

    if type_part == 4 {
        let mut bin = Vec::new();
        let mut content_length = 0;

        for chunk in input[6..].chunks(5) {
            content_length += chunk.len();
            bin.extend_from_slice(&chunk[1..]);

            if !chunk[0] {
                break;
            }
        }

        content_length += 6;

        return (content_length, Packet::Literal(to_number(&bin)));
    }

    let length_type = match input[6] {
        false => LengthType::Bytes,
        true => LengthType::Packets
    };

    let (content_length, content): (usize, Vec<Packet>) = match length_type {
        LengthType::Bytes => {
            let length = to_number(&input[7..22]);

            let mut running_length = 0;
            let mut packets: Vec<Packet> = Vec::new();

            while running_length < length {
                let (out_length, p) = parse_packet(&input[22 + running_length..]);
                running_length += out_length;
                packets.push(p);
            }

            (22 + length, packets)
            },
        LengthType::Packets => {
            let length = to_number(&input[7..18]);

            let mut running_index = 18;
            let mut packets = Vec::new();

            while packets.len() < length {
                let (out_length, p) = parse_packet(&input[running_index..]);
                running_index += out_length;
                packets.push(p);
            }

            (running_index, packets)
        }
    };

    return match type_part {
        0 => (content_length, Packet::Sum(content)),
        1 => (content_length, Packet::Product(content)),
        2 => (content_length, Packet::Min(content)),
        3 => (content_length, Packet::Max(content)),
        5 => (content_length, Packet::Gt(content)),
        6 => (content_length, Packet::Lt(content)),
        7 => (content_length, Packet::Eq(content)),
        _ => panic!("invalid packet type")
    };
    
}

fn to_number(bin: &[bool]) -> usize {
    bin.iter().rev().enumerate()
        .map(|(base, digit)| if *digit {return 1 << base;} else {return 0;}).sum()
}

fn eval_packet(packet: &Packet) -> usize {

    return match packet {
        Packet::Sum(children) => children.iter().fold(0, |res, child| res + eval_packet(child)),
        Packet::Product(children) => children.iter().fold(1, |res, child| res * eval_packet(child)),
        Packet::Min(children) => children.iter().fold(std::usize::MAX, |res, child| std::cmp::min(res, eval_packet(child))),
        Packet::Max(children) => children.iter().fold(0, |res, child| std::cmp::max(res, eval_packet(child))),
        Packet::Literal(content) => *content,
        Packet::Gt(children) => (eval_packet(&children[0]) > eval_packet(&children[1])) as usize,
        Packet::Lt(children) => (eval_packet(&children[0]) < eval_packet(&children[1])) as usize,
        Packet::Eq(children) => (eval_packet(&children[0]) == eval_packet(&children[1])) as usize
    }
}


fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
    .lines().map(|l| l.unwrap()).collect();

    let bin_number = lines[0].chars().fold(Vec::new(),|mut res, c| {res.extend(from_hex(&c).unwrap()); return res;});

    let (_, packet) = parse_packet(&bin_number);

    let res = eval_packet(&packet);

    println!("res {}", res);
}
