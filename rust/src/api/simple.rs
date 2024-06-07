

extern crate num_bigint;
extern crate num_traits;

use std::cmp::min;
use std::str::FromStr;
use ff::{PrimeField, to_hex};
use num_bigint::{BigInt, Sign};
use num_traits::Zero;
use poseidon_rs::{Fr, Poseidon};
use rustc_hex::{FromHex, ToHex};
use crate::api::eddsa::{decompress_point, decompress_signature, Point, PrivateKey, Signature, verify};


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}


#[flutter_rust_bridge::frb(sync)]
pub fn pack_signature(signature: String) -> String {
    let signature_str = signature.as_str();
    let signature_bytes_raw = signature_str.from_hex().unwrap();
    let mut signature_bytes: [u8; 64] = [0; 64];
    signature_bytes.copy_from_slice(&signature_bytes_raw);

    let r_b8_bytes: [u8; 32] = *array_ref!(signature_bytes[..32], 0, 32);
    let s: BigInt = BigInt::from_bytes_le(Sign::Plus, &signature_bytes[32..]);
    let x_big: BigInt = BigInt::from_bytes_le(Sign::Plus, &r_b8_bytes[0..15]);
    let y_big: BigInt = BigInt::from_bytes_le(Sign::Plus, &r_b8_bytes[15..32]);

    let r_b8: Point = Point {
        x: Fr::from_str(
            &x_big.to_string(),
        ).unwrap(),
        y: Fr::from_str(
            &y_big.to_string(),
        ).unwrap(),
    };

    let sig = Signature { r_b8 : r_b8.clone(), s };
    let res = sig.compress();

    let hex_string = to_hex_string(res.to_vec());
    hex_string
}

#[flutter_rust_bridge::frb(sync)]
pub fn unpack_signature(compressed_signature: String) -> String{
    let compressed_signature_str = compressed_signature.as_str();
    let compressed_signature_bytes_raw = compressed_signature_str.from_hex().unwrap();
    let mut compressed_signature_bytes: [u8; 64] = [0; 64];
    compressed_signature_bytes.copy_from_slice(&compressed_signature_bytes_raw);
    let decompressed_sig = decompress_signature(&compressed_signature_bytes).unwrap();

    let mut b: Vec<u8> = Vec::new();

    let x_big = BigInt::parse_bytes(to_hex(&decompressed_sig.r_b8.x).as_bytes(), 16).unwrap();
    let y_big = BigInt::parse_bytes(to_hex(&decompressed_sig.r_b8.y).as_bytes(), 16).unwrap();
    let (_, x_bytes) = x_big.to_bytes_le();
    let (_, y_bytes) = y_big.to_bytes_le();
    let mut x_16bytes: [u8; 16] = [0; 16];
    let lenx = min(x_bytes.len(), x_16bytes.len());
    x_16bytes[..lenx].copy_from_slice(&x_bytes[..lenx]);
    b.append(&mut x_16bytes.to_vec());
    let mut y_16bytes: [u8; 16] = [0; 16];
    let leny = min(y_bytes.len(), y_16bytes.len());
    y_16bytes[..leny].copy_from_slice(&y_bytes[..leny]);
    b.append(&mut y_16bytes.to_vec());
    let (_, s_bytes) = decompressed_sig.s.to_bytes_le();
    let mut s_32bytes: [u8; 32] = [0; 32];
    let lens = min(s_bytes.len(), s_32bytes.len());
    s_32bytes[..lens].copy_from_slice(&s_bytes[..lens]);
    b.append(&mut s_32bytes.to_vec());

    let mut r: [u8; 64] = [0; 64];
    let res_len = min(r.len(), b.len());
    r[..res_len].copy_from_slice(&b[..res_len]);
    let hex_string = to_hex_string(r.to_vec());
    hex_string
}


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn sign_poseidon(private_key: String, msg: String) -> String {
    let private_key_str = private_key.as_str();
    //let pk_bigint = BigInt::from_str(private_key_str).unwrap();
    let pk_bytes_raw = private_key_str.from_hex().unwrap();
    let mut pk_bytes: [u8; 32] = [0; 32];
    pk_bytes.copy_from_slice(&pk_bytes_raw);
    let pk = PrivateKey { key: pk_bytes };
    let message_str = msg.as_str();
    let message_bigint = BigInt::from_str(message_str).unwrap();
    let sig = pk.sign(message_bigint.clone()).unwrap();
    let compressed_signature = sig.compress();
    let hex_string = compressed_signature.to_hex();
    hex_string
}

#[flutter_rust_bridge::frb(sync)]
pub fn prv2pub(private_key: String) -> String {
    /*let private_key_bytes: [u8; 32] = *array_ref!(private_key[..32], 0, 32);
    let private_key = PrivateKey::import(private_key_bytes.to_vec()).unwrap();*/
    let private_key_str = private_key.as_str();
    //let pk_bigint = BigInt::from_str(private_key_str).unwrap();
    let pk_bytes_raw = private_key_str.from_hex().unwrap();
    let mut pk_bytes: [u8; 32] = [0; 32];
    pk_bytes.copy_from_slice(&pk_bytes_raw);
    let pk = PrivateKey { key: pk_bytes };
    let public_key = pk.public();
    let mut result_string: String = "".to_owned();
    result_string.push_str(&public_key.x.to_string());
    result_string.push_str(",");
    result_string.push_str(&public_key.y.to_string());
    result_string
}


#[flutter_rust_bridge::frb(sync)]
pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}

#[flutter_rust_bridge::frb(sync)]
pub fn pack_point(point_x: String, point_y: String) -> String {
    let point_x_str = point_x.as_str();
    let point_y_str = point_y.as_str();
    let p: Point = Point {
        x: Fr::from_str(point_x_str).unwrap(),
        y: Fr::from_str(point_y_str).unwrap(),
    };

    let compressed_point = p.compress();
    let hex_string = to_hex_string(compressed_point.to_vec());
    hex_string
}

#[flutter_rust_bridge::frb(sync)]
pub fn unpack_point(compressed_point: String) -> String {
    let compressed_point_str = compressed_point.as_str();
    let y_bytes_raw = compressed_point_str.from_hex().unwrap();
    let mut y_bytes: [u8; 32] = [0; 32];
    y_bytes.copy_from_slice(&y_bytes_raw);
    let p = decompress_point(y_bytes).unwrap();
    let x_big = BigInt::parse_bytes(to_hex(&p.x).as_bytes(), 16).unwrap();
    let y_big = BigInt::parse_bytes(to_hex(&p.y).as_bytes(), 16).unwrap();
    let mut result_string: String = "".to_owned();
    result_string.push_str(&x_big.to_string());
    result_string.push_str(",");
    result_string.push_str(&y_big.to_string());
    result_string
}

#[flutter_rust_bridge::frb(sync)]
pub fn hash_poseidon(tx_compressed_data: String) -> String {
    let tx_compressed_data_str = &tx_compressed_data.as_str();
    let b0: Fr = Fr::from_str(tx_compressed_data_str).unwrap();
    let hm_input = vec![b0.clone()];
    let poseidon = Poseidon::new();
    let hm = poseidon.hash(hm_input).unwrap();
    to_hex(&hm)
}

#[flutter_rust_bridge::frb(sync)]
pub fn verify_poseidon(private_key: String, compressed_signature: String, message: String) -> String {
    let private_key_str = private_key.as_str();
    // let pk_bigint = BigInt::from_str(private_key_str).unwrap();
    let pk_bytes_raw = private_key_str.from_hex().unwrap();
    let mut pk_bytes: [u8; 32] = [0; 32];
    pk_bytes.copy_from_slice(&pk_bytes_raw);
    let pk = PrivateKey { key: pk_bytes };
    let compressed_signature_str = compressed_signature.as_str();
    let signature_bytes_raw = compressed_signature_str.from_hex().unwrap();
    let mut signature_bytes: [u8; 64] = [0; 64];
    signature_bytes.copy_from_slice(&signature_bytes_raw);
    let sig = decompress_signature(&signature_bytes).unwrap();
    let message_str = message.as_str();
    let message_bigint = match message_str.parse::<i32>() {
        Ok(n) => BigInt::from(n),
        Err(e) => BigInt::zero(),
    };
    if verify(pk.public(), sig.clone(), message_bigint.clone()) {
        String::from("1")
    } else {
        String::from("0")
    }
}


mod test {
    use crate::api::simple::pack_point;

    #[test]
    fn test1() {
        let res = pack_point(String::from("5067058882184289685879291240436517726527787201084588250492822232261202434720"), String::from("17039996928425847512124231608079985051815305514092976608073695914635437634363"));
        println!("res: {}", res);
    }
}