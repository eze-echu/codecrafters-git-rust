use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

pub struct HashObject {
    value: Vec<u8>,
}
impl HashObject {
    pub fn new(text: &str) -> Self {
        Self { value: text.into() }
    }
    pub fn value(&self) -> String {
        format!(
            "blob {}\x00{}",
            self.size(),
            String::from_utf8(self.value.clone()).unwrap()
        )
    }
    pub fn value_as_byte(&self) -> Vec<u8> {
        let answer = format!(
            "blob {}\x00{}",
            self.size(),
            String::from_utf8(self.value.clone()).unwrap()
        );
        answer.as_bytes().to_vec()
    }
    fn size(&self) -> usize {
        self.value.len()
    }
    pub fn decode(encoded_value: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let mut decoder = ZlibDecoder::new(&encoded_value[..]);
        let mut decompressed: String = String::new();
        match decoder.read_to_string(&mut decompressed) {
            Ok(_) => {
                if Self::is_valid_hash_object(&decompressed) {
                    Ok(decompressed)
                } else {
                    Err(Box::from(
                        format!(
                            "Decoded Value is not a valid Hash Object:\n. {}",
                            decompressed
                        )
                        .to_string(),
                    ))
                }
            }
            Err(e) => {
                eprintln!("Error Decoding from Vec<u8> to HashObject: {e}");
                Err(e.into())
            }
        }
    }
    fn is_valid_hash_object(string_to_check: &str) -> bool {
        let re = regex::bytes::Regex::new("blob [0-9]+\x00.*$").unwrap();
        re.is_match(string_to_check.as_bytes())
    }
    pub fn encode(&self) -> Vec<u8> {
        let value = self.value();
        let mut encoder = ZlibEncoder::new(value.as_bytes(), flate2::Compression::default());
        let mut buffer = vec![];
        encoder.read_to_end(&mut buffer).unwrap();
        buffer.to_vec()
    }
}
impl Display for HashObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.value.clone()).expect("Error displaying value")
        )
    }
}
impl FromStr for HashObject {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
impl TryFrom<Vec<u8>> for HashObject {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let decoded_text = HashObject::decode(value)?;
        let value = decoded_text.split("\x00").collect::<Vec<&str>>();
        Ok(HashObject::new(value[1]))
    }
}
