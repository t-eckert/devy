use super::{Error, Result};
use jsonwebtoken::DecodingKey;
use openssl::rsa::Rsa;

#[derive(Clone)]
pub struct Decoder {
    decoding_key: DecodingKey,
}
