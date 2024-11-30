pub(crate) use super::GitObject;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;

pub struct HashObject {
    value: Arc<[u8]>,
}
impl HashObject {
    fn new(text: &str) -> Self {
        Self {
            value: Arc::from(text.as_bytes()),
        }
    }
}
impl GitObject for HashObject {
    fn formatted_value(&self) -> String {
        format!(
            "blob {}{}",
            self.size(),
            String::from_utf8(self.value.to_vec()).unwrap()
        )
    }

    fn unformatted_value(&self) -> String {
        String::from_utf8(self.value.to_vec()).expect("Error displaying value")
    }

    fn formatted_value_as_bytes(&self) -> Vec<u8> {
        let mut answer = vec![];
        answer.append(&mut b"blob ".to_vec());
        answer.append(&mut self.size().to_be_bytes().to_vec());
        answer.append(&mut b"\x00".to_vec());
        answer.append(&mut self.value.to_vec());
        answer.to_vec()
    }
    fn size(&self) -> usize {
        self.value.len()
    }
    fn is_valid_object(string_to_check: &str) -> bool {
        let re = regex::bytes::Regex::new("blob [0-9]+\x00.*").unwrap();
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
        let decoded_text = Self::decoded_to_string(value)?;
        let value = decoded_text.split("\x00").collect::<Vec<&str>>();
        Ok(HashObject::new(value[1]))
    }
}
impl Display for HashObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.value.to_vec()).expect("Error displaying value")
        )
    }
}
