use openssl::sha::sha1;

use super::encode::encode;

pub fn get_digest_value(xml: &str) -> String {
    let digest = sha1(xml.as_bytes());
    encode(digest)
}
