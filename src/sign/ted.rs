use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Signer};

use crate::utils::{encode::encode, latin::to_latin1};

pub fn sign_ted(rsask: &str, dd: &str) -> Result<String, &'static str> {
    let dd_one_line = dd.replace('\n', "");
    let rsa = Rsa::private_key_from_pem(rsask.as_bytes()).map_err(|_| "Failed to parse RSA key")?;
    let keypair = PKey::from_rsa(rsa).map_err(|_| "Failed to create key pair")?;

    // Sign the data
    let mut signer =
        Signer::new(MessageDigest::sha1(), &keypair).map_err(|_| "Failed to create signer")?;
    let latin1_vec = to_latin1(&dd_one_line);
    signer
        .update(&latin1_vec)
        .map_err(|_| "Failed to update signer")?;
    let signature = signer.sign_to_vec().map_err(|_| "Failed to sign data")?;
    Ok(encode(signature))
}
