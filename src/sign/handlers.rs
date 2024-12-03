use roxmltree::Document;
use std::cmp::min;

use crate::{
    enums::SignatureType,
    key::KeyPairCert,
    models::signature::{signed_info::SignedInfo, Signature},
    utils::digest::get_digest_value,
};

// Parse the xml to create the signature
pub fn get_signature(
    xml_str: &str,
    signature_type: SignatureType,
    key_pair_cert: KeyPairCert,
) -> Result<String, &'static str> {
    match Document::parse(xml_str) {
        Ok(doc) => {
            let element = signature_type.element_to_sign();
            // Attempt to find the element
            if let Some(node) = doc.descendants().find(|n| n.has_tag_name(element)) {
                // Attempt to retrieve the ID attribute
                let uri_data = node.attribute("ID");
                // Retrieve the element to sign (Ex: Documento)
                let mut element_to_sign = xml_str[node.range().start..node.range().end].to_string();

                // We should inherit/add namespaces if there are any
                if let Some(namespaces) = &signature_type.namespaces() {
                    element_to_sign = add_namespaces(&element_to_sign, namespaces);
                }

                let digest_value = get_digest_value(&element_to_sign);

                // Create the SignedInfo Element
                let signed_info = SignedInfo::new(&digest_value, signature_type, uri_data);

                let signature = Signature::new(signed_info);

                // create the signature by signing the SingedInfo, not the element!
                Ok(signature.create(key_pair_cert))
            } else {
                Err("Could not find the element")
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err("Error parsing XML: ")
        }
    }
}

fn add_namespaces(xml: &str, namespaces: &str) -> String {
    let space_index = xml.chars().position(|c| c == ' ').unwrap();
    let close_index = xml.chars().position(|c| c == '>').unwrap();
    let index = min(space_index, close_index);
    let start = &xml[..index];
    let end = &xml[index..];
    format!("{} {}{}", start, &namespaces, end)
}
