use byteorder::{BigEndian, WriteBytesExt};

pub fn write_varint(mut value: i32, data: &mut Vec<u8>) {
    loop {
        let mut temp: u8 = (value & 0b01111111) as u8;

        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }

        data.push(temp);

        if value == 0 {
            return;
        }
    }
}

pub fn write_varlong(mut value: i128, data: &mut Vec<u8>) {
    if value == 0 {
        data.push(0);
    }
    while value != 0 {
        let mut temp: u8 = (value & 0b01111111) as u8;
        // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        data.push(temp);
    }
}

pub fn write_string(value: &str, data: &mut Vec<u8>) {
    let bytes = value.as_bytes();
    write_varint(bytes.len() as i32, data);
    data.append(&mut bytes.to_vec());
}

pub fn write_ushort(value: u16, data: &mut Vec<u8>) {
    data.write_u16::<BigEndian>(value).unwrap();
}

pub fn write_float(value: f32, data: &mut Vec<u8>) {
    data.write_f32::<BigEndian>(value).unwrap();
}
pub fn write_bool(value: bool, data: &mut Vec<u8>) {
    data.write_u8(if value { 1 } else { 0 }).unwrap();
}
