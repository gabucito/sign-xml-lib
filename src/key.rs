use openssl::{pkcs12::Pkcs12, pkey::{PKey, Private, Public}};

pub struct KeyPairCert {
    pub private_key: PKey<Private>,
    pub public_key: PKey<Public>,
    pub cert_vec: Vec<u8>,
}

impl KeyPairCert {
    pub fn get_key(der: &[u8], pass: &str) -> Self {
        // Load the PKCS#12 file (digital certificate)
        let pkcs12 = Pkcs12::from_der(der).unwrap();
    
        // Extract the private key and certificate from the PKCS#12 file
        let parsed_pkcs12 = pkcs12.parse2(pass).unwrap();
        let private_key = parsed_pkcs12.pkey.unwrap();
        let cert_vec = parsed_pkcs12.cert.clone().unwrap().to_pem().unwrap();
        let public_key = PKey::from_rsa(
            parsed_pkcs12
                .cert
                .unwrap()
                .public_key()
                .unwrap()
                .rsa()
                .unwrap()
                .clone(),
        )
        .unwrap();
    
        KeyPairCert{
            private_key,
            public_key,
            cert_vec
        }
    }
}