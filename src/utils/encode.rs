use base64::{engine::general_purpose, Engine as _};

pub fn encode<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(input)
}