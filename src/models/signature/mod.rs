pub mod signed_info;

use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    sign::Signer,
};

use crate::{models::key::KeyPairCert, utils::encode::encode};

use signed_info::SignedInfo;

pub struct Signature {
    pub signed_info: SignedInfo,
}

struct SignatureElements {
    signature_value: String,
    modulus: String,
    exponent: String,
    x509: String,
}

impl Signature {
    pub fn new(signed_info: SignedInfo) -> Self {
        Self { signed_info }
    }

    pub fn create(&self, key_cert_pair: KeyPairCert) -> String {
        let signature_elements =
            get_signature_values(&self.signed_info.with_namespace, key_cert_pair);

        // let is_valid = verify(&self.signed_info.with_namespace, &signature_value);
        // println!("is valid: {}", is_valid);

        format!(
            r###"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">
    {}
    <SignatureValue>{}</SignatureValue>
    <KeyInfo>
        <KeyValue>
            <RSAKeyValue>
                <Modulus>{}</Modulus>
                <Exponent>{}</Exponent>
            </RSAKeyValue>
        </KeyValue>
        <X509Data>
            <X509Certificate>{}</X509Certificate>
        </X509Data>
    </KeyInfo>
</Signature>"###,
            self.signed_info.without_namespace,
            signature_elements.signature_value,
            signature_elements.modulus,
            signature_elements.exponent,
            signature_elements.x509
        )
    }
}

fn get_signature_values(signed_info: &str, key_cert_pair: KeyPairCert) -> SignatureElements {
    // Convert the certificate from Vec<u8> to a string
    let certificate = std::str::from_utf8(&key_cert_pair.cert_vec).unwrap();

    // Remove the PEM headers and footers from the certificate
    let x509 = remove_cert_headers(certificate);
    // format_long_string_in_place(&mut cert_without_headers);

    // Calculate the signature value of the provided XML string
    let signature_value = get_signature_value(signed_info, &key_cert_pair.private_key);
    // format_long_string_in_place(&mut signature_value);

    // let private_vec = private_key.private_key_to_pem_pkcs8().unwrap();
    // println!("{}", std::str::from_utf8(&private_vec).unwrap());

    // Extract RSA modulus and exponent from the private key for further use
    let rsa = key_cert_pair.private_key.rsa().unwrap();
    let modulus = encode(rsa.n().to_vec());
    // format_long_string_in_place(&mut modulus);
    let exponent = encode(rsa.e().to_vec());

    SignatureElements {
        signature_value,
        modulus,
        exponent,
        x509,
    }
}

fn get_signature_value(signed_info: &str, keypair: &PKey<Private>) -> String {
    // Create a new signer with SHA-1 algorithm
    let mut signer = Signer::new(MessageDigest::sha1(), keypair).unwrap();

    // Update the signer with the XML data
    signer.update(signed_info.as_bytes()).unwrap();

    // Sign the data and return the base64 encoded signature
    let signature = signer.sign_to_vec().unwrap();
    encode(signature)
}

fn remove_cert_headers(cert: &str) -> String {
    // Remove the BEGIN and END certificate headers and any newline characters
    cert.replace("-----BEGIN CERTIFICATE-----", "")
        .replace("-----END CERTIFICATE-----", "")
        .replace('\n', "")
}

fn _format_long_string_in_place(input: &mut String) {
    let mut insert_positions = Vec::new();
    let mut line_length = 0;

    for (index, _char) in input.chars().enumerate() {
        line_length += 1;
        if line_length >= 65 {
            insert_positions.push(index);
            line_length = 0;
        }
    }

    // Insert newlines at the recorded positions
    for (i, pos) in insert_positions.iter().enumerate() {
        input.insert(pos + i, '\n');
    }
}
