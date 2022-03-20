use crate::protocol::data::reader::PacketReader;
use crate::protocol::data::write_types::{
    write_string, write_string_len, write_ushort, write_varint,
};
use crate::protocol::data::writer::PacketBuilder;
use crate::protocol::types::{PVarType, PVarTypeTemplate};
use crate::stream::NetworkStream;
//use openssl::sha::Sha1;

pub struct LoginRequest {
    pub(crate) connection_host: String,
    pub(crate) connection_port: u32,
    pub(crate) username: String,
}

macro_rules! inner_enum {
    ( $var:expr , $enum_type:path ) => {{
        match $var {
            $enum_type(val) => val,
            _ => panic!("Incorrect token"),
        }
    }};
}

impl LoginRequest {
    pub fn send(&self, stream: &mut NetworkStream) {
        {
            // Send handshake
            let mut handshake_packet = PacketBuilder::new(0x00);

            // Protocol Version
            write_varint(578, &mut handshake_packet.data);
            // Server Address
            write_string(self.connection_host.as_str(), &mut handshake_packet.data);
            // Server Port
            write_ushort(self.connection_port as u16, &mut handshake_packet.data);
            // Tell it we want to login
            write_varint(2, &mut handshake_packet.data);

            handshake_packet.send(stream);
        }

        {
            // Identify ourselves
            let mut login_packet = PacketBuilder::new(0x00);

            // Username
            write_string_len(&self.username, 16, &mut login_packet.data);

            login_packet.send(stream);
        }

        // region: Encryption
        // Save for online mode later, for now we'll only support offline mode
        //         let encryption_request = PacketReader::new()
        //             .add_token(PVarType::String(String::new()))
        //             .add_token(PVarType::VarIntByteArray(Vec::new()))
        //             .add_token(PVarType::VarIntByteArray(Vec::new()))
        //             .read(stream);
        //
        //         let mut encryption_code = inner_enum!(encryption_request.tokens.get(1).unwrap(), PVarType::VarIntByteArray);
        //
        //         let encryption_key =  match openssl::rsa::Rsa::public_key_from_der(
        //             encryption_code.as_slice()) {
        //             Ok(key) => key,
        //             Err(err) => {
        //                 println!("Error parsing public encryption key: {}", err);
        //                 panic!();
        //             },
        //         };
        //
        //         let secret: [u8; 16] = [0,3,5,6,67,7,2,7,7,2,4,7,23,3,7,63];
        //         let mut encrypted_secret = vec![0; 128];
        //         let mut encrypted_verify = vec![0; 128];
        //
        //         let verify_token = inner_enum!(encryption_request.tokens.get(2).unwrap(), PVarType::VarIntByteArray);
        //
        //         match encryption_key.public_encrypt(&secret, encrypted_secret.as_mut_slice(), Padding::PKCS1) {
        //             Ok(bytes) => {},
        //             Err(e) => println!("Error: {}", e)
        //         }
        //
        //         match encryption_key.public_encrypt(&verify_token, encrypted_verify.as_mut_slice(), Padding::PKCS1) {
        //             Ok(bytes) => { },
        //             Err(e) => println!("Error: {}", e)
        //         }
        //
        //         let mut server_id = Vec::new();
        //         server_id.append(&mut secret.to_vec());
        //         server_id.append(&mut encryption_code.clone());
        //
        //         let id = calc_hash(&mut server_id);
        //
        //         let client = reqwest::Client::new();
        //         let res = block_on(client.post("https://sessionserver.mojang.com/session/minecraft/join")
        //             .body(format!(r#"
        // {{
        //     "accessToken": "{}",
        //     "selectedProfile": "{}",
        //     "serverId": "{}"
        // }}
        //     "#, "test", "uuid", id)).send());
        //
        //
        //
        //endregion
    }
}

/// Custom hashing function
/// https://wiki.vg/Protocol_Encryption
// pub fn calc_hash(input: &mut Vec<u8>) -> String {
//     let mut hasher = Sha1::new();
//
//     hasher.update(input);
//
//     let mut hash = hasher.finish().to_vec();
//     // Option 1
//     let negative = (hash.get(0).unwrap() & 0x80) == 0x80;
//
//     if negative {
//         two_complement(&mut hash);
//     }
//
//     let mut out = String::with_capacity(20);
//
//     for t in hash.iter() {
//         out.push_str(&format!("{:02x}", t));
//     }
//
//     if out.starts_with('0') {
//         out = out.split_at(1).1.to_string();
//     }
//
//     if negative {
//         out = String::from(" - ") + &out;
//     }
//
//     return out;
// }

fn two_complement(bytes: &mut Vec<u8>) {
    let mut carry = true;
    for i in (0..bytes.len()).rev() {
        bytes[i] = !bytes[i] & 0xff;
        if carry {
            carry = bytes[i] == 0xff;
            bytes[i] = bytes[i] + 1;
        }
    }
}
