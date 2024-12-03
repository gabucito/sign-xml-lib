use handlers::get_signature;

use crate::{
    enums::SignatureType, models::key::KeyPairCert, utils::insert_element::insert_after_element,
};

mod handlers;

pub fn sign_documento(xml_str: &str, key_pair_cert: KeyPairCert) -> Result<String, &'static str> {
    let signature = get_signature(xml_str, SignatureType::DTE, key_pair_cert)?;
    Ok(insert_after_element(xml_str, &signature, "Documento"))
}

pub fn sign_envio(xml_str: &str, key_pair_cert: KeyPairCert) -> Result<String, &'static str> {
    let signature = get_signature(xml_str, SignatureType::EnvioDTE, key_pair_cert)?;
    Ok(insert_after_element(xml_str, &signature, "SetDTE"))
}

pub fn sign_token(xml_str: &str, key_pair_cert: KeyPairCert) -> Result<String, &'static str> {
    let signature = get_signature(xml_str, SignatureType::Token, key_pair_cert)?;
    Ok(insert_after_element(xml_str, &signature, "item"))
}
