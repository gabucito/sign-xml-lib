use crate::enums::SignatureType;

pub struct SignedInfo {
    pub with_namespace: String,
    pub without_namespace: String,
}

impl SignedInfo {
    pub fn new(
        digest_value: &str,
        signature_type: SignatureType,
        uri_data: Option<&str>,
    ) -> SignedInfo {
        // Conditionally create the Reference element's opening tag
        let reference_start = match uri_data {
            Some(uri) => format!(r###"<Reference URI="#{}">"###, uri),
            None => "<Reference>".to_string(),
        };

        let namespace = match signature_type.xsi_namespace() {
            Some(xsi) => format!(r###"xmlns="http://www.w3.org/2000/09/xmldsig#" {xsi}"###),
            None => r###"xmlns="http://www.w3.org/2000/09/xmldsig#""###.to_string(),
        };

        // xmlns="http://www.w3.org/2000/09/xmldsig#" seems to be always the default namespace in signedinfo
        let with_namespace = format!(
            r###"<SignedInfo {namespace}>
        <CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"></CanonicalizationMethod>
        <SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"></SignatureMethod>
        {}
            <Transforms>
                <Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"></Transform>
            </Transforms>
            <DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"></DigestMethod>
            <DigestValue>{}</DigestValue>
        </Reference>
    </SignedInfo>"###,
            reference_start, digest_value
        );
        let without_namespace = format!(
            r###"<SignedInfo>
        <CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"></CanonicalizationMethod>
        <SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"></SignatureMethod>
        {}
            <Transforms>
                <Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"></Transform>
            </Transforms>
            <DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"></DigestMethod>
            <DigestValue>{}</DigestValue>
        </Reference>
    </SignedInfo>"###,
            reference_start, digest_value
        );

        SignedInfo {
            with_namespace,
            without_namespace,
        }
    }
}
