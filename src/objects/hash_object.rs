pub(crate) use super::GitObject;
use crate::objects::GitObjectEncoding;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

pub struct HashObject {
    value: Vec<u8>,
}
impl HashObject {
    fn new(text: &str) -> Self {
        Self { value: text.into() }
    }
}
impl GitObject for HashObject {
    fn formatted_value(&self) -> String {
        format!(
            "blob {}\x00{}",
            self.size(),
            String::from_utf8(self.value.clone()).unwrap()
        )
    }

    fn unformatted_value(&self) -> String {
        String::from_utf8(self.value.clone()).expect("Error displaying value")
    }

    fn value_as_bytes(&self) -> Vec<u8> {
        let answer = self.formatted_value();
        answer.as_bytes().to_vec()
    }
    fn size(&self) -> usize {
        self.value.len()
    }
    fn is_valid_object(string_to_check: &str) -> bool {
        let re = regex::bytes::Regex::new("blob [0-9]+\x00.*$").unwrap();
        re.is_match(string_to_check.as_bytes())
    }
}
impl FromStr for HashObject {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
impl TryFrom<Vec<u8>> for HashObject {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let encoder = GitObjectEncoding;
        let decoded_text = GitObjectEncoding::decode::<Self>(value)?;
        let value = decoded_text.split("\x00").collect::<Vec<&str>>();
        Ok(HashObject::new(value[1]))
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
