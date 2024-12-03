pub enum SignatureType {
    #[allow(clippy::upper_case_acronyms)]
    DTE,
    ReciboMercaderia,
    EnvioDTE,
    EnvioBOLETA,
    EnvioLibro,
    Resultado,
    Mercaderia,
    Token,
}

impl SignatureType {
    pub fn element_to_sign(&self) -> &'static str {
        match self {
            SignatureType::DTE => "Documento",
            SignatureType::ReciboMercaderia => "DocumentoRecibo",
            SignatureType::EnvioDTE => "SetDTE",
            SignatureType::EnvioBOLETA => "SetDTE",
            SignatureType::EnvioLibro => "EnvioLibro",
            SignatureType::Resultado => "Resultado",
            SignatureType::Mercaderia => "SetRecibos",
            SignatureType::Token => "getToken",
        }
    }

    pub fn default_namespace(&self) -> Option<&'static str> {
        match self {
            SignatureType::DTE => None,
            SignatureType::ReciboMercaderia => None,
            SignatureType::EnvioDTE => Some(r###"xmlns="http://www.sii.cl/SiiDte""###),
            SignatureType::EnvioBOLETA => Some(r###"xmlns="http://www.sii.cl/SiiDte""###),
            SignatureType::EnvioLibro => Some(r###"xmlns="http://www.sii.cl/SiiDte""###),
            SignatureType::Resultado => Some(r###"xmlns="http://www.sii.cl/SiiDte""###),
            SignatureType::Mercaderia => Some(r###"xmlns="http://www.sii.cl/SiiDte""###),
            SignatureType::Token => None,
        }     
    }

    pub fn xsi_namespace(&self) -> Option<&'static str> {
        match self {
            SignatureType::DTE => None,
            SignatureType::ReciboMercaderia => None,
            SignatureType::EnvioDTE => Some(r###"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""###),
            SignatureType::EnvioBOLETA => Some(r###"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""###),
            SignatureType::EnvioLibro => Some(r###"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""###),
            SignatureType::Resultado => Some(r###"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""###),
            SignatureType::Mercaderia => Some(r###"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""###),
            SignatureType::Token => None,
        }    
    }

    pub fn namespaces(&self) -> Option<String> {
        match (self.default_namespace(), self.xsi_namespace()) {
            (None, None) => None,
            (None, Some(_)) => None,
            (Some(default), None) => Some(default.to_string()),
            (Some(default), Some(xsi)) => Some(format!("{default} {xsi}")),
        }
    }
}