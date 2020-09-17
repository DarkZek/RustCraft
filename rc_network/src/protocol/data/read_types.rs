// Data Types Source from https://wiki.vg/Protocol#Data_types

use crate::stream::NetworkStream;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use nbt::Blob;
use std::io::Read;

pub fn read_varint<T: Read>(read: &mut T) -> i64 {
    let mut result = 0;

    const MSB: u8 = 0b10000000;
    const MASK: u8 = !MSB;

    for i in 0..5 {
        let read = read.read_u8().unwrap();

        result |= ((read & MASK) as i64) << (7 * i);

        /* The last (5th) byte is only allowed to have the 4 LSB set */
        if i == 4 && (read & 0xf0 != 0) {
            panic!("VarInt is too long, last byte: {}", read)
        }

        if (read & MSB) == 0 {
            return result;
        }
    }

    panic!("read_varint reached end of loop, which should not be possible")
}

pub fn read_varlong<R: Read>(reader: &mut R) -> i64 {
    let mut result = 0;

    let msb: u8 = 0b10000000;
    let mask: u8 = !msb;

    for i in 0..10 {
        let read = reader.read_u8().unwrap();

        result |= ((read & mask) as i64) << (7 * i);

        /* The last (10th) byte is only allowed to have the LSB set */
        if i == 9 && ((read & (!0x1)) != 0) {
            panic!("VarLong is too long, last byte: {}", read);
        }

        if (read & msb) == 0 {
            return result;
        }
    }

    panic!("read_varlong reached end of loop, which should not be possible");
}

pub fn read_string<T: Read>(read: &mut T) -> String {
    let len = read_varint(read);

    let mut text: Vec<u8> = vec![0; len as usize];
    read.read_exact(&mut text).unwrap();

    let result = std::str::from_utf8(&text).unwrap().to_string();

    result
}

pub fn length_as_varint(mut value: i64) -> i64 {
    let mut i = 0;
    loop {
        value >>= 7;

        i += 1;

        if value == 0 {
            return i;
        }
    }
}

pub fn read_int<T: Read>(read: &mut T) -> i32 {
    read.read_i32::<BigEndian>().unwrap()
}

pub fn read_unsignedbyte<T: Read>(read: &mut T) -> u8 {
    match read.read_u8() {
        Ok(val) => val,
        Err(_) => panic!(),
    }
}

pub fn read_long<T: Read>(read: &mut T) -> i64 {
    read.read_i64::<BigEndian>().unwrap()
}

pub fn read_float<T: Read>(read: &mut T) -> f32 {
    read.read_f32::<BigEndian>().unwrap()
}

pub fn read_double<T: Read>(read: &mut T) -> f64 {
    read.read_f64::<BigEndian>().unwrap()
}

pub fn read_bool<T: Read>(read: &mut T) -> bool {
    read.read_u8().unwrap() == 1
}

pub fn read_short<T: Read>(read: &mut T) -> i16 {
    read.read_i16::<BigEndian>().unwrap()
}

pub fn read_uuid<T: Read>(read: &mut T) -> u128 {
    read.read_u128::<BigEndian>().unwrap()
}

pub fn read_position<T: Read>(read: &mut T) -> [i64; 3] {
    let val = read.read_i64::<BigEndian>().unwrap();

    [val >> 38, val & 0xFFF, val << 26 >> 38]
}

pub fn read_bytearray<T: Read>(read: &mut T, len: u16) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    read.read_exact(&mut buf);
    buf
}

pub fn read_longarray<T: Read>(read: &mut T, len: u16) -> Vec<i64> {
    let mut buf = Vec::with_capacity(len as usize);
    for _ in 0..len {
        buf.push(read.read_i64::<LittleEndian>().unwrap());
    }
    buf
}

pub fn read_intarray(read: &mut NetworkStream, len: usize) -> Vec<i32> {
    let mut result = Vec::new();

    for _ in 0..len {
        result.push(read_int(read));
    }

    result
}

pub fn read_varintarray(read: &mut NetworkStream) -> Vec<u8> {
    let len = read_varint(read);

    let byte_array = read_bytearray(read, len as u16);

    byte_array
}

pub fn read_varlongarray(read: &mut NetworkStream) -> Vec<i64> {
    let len = read_varint(read);

    let byte_array = read_longarray(read, len as u16);

    byte_array
}

pub fn read_nbtarray(read: &mut NetworkStream, _len: u16) -> Vec<Blob> {
    let mut nbt = Vec::new();
    let bytes_count = 0;

    while bytes_count < 0 {
        let t = Blob::from_reader(read).unwrap();
        nbt.push(t);
    }

    nbt
}
