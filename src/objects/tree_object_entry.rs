use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(super) struct TreeObjectEntry {
    value: Vec<u8>,
}
impl TreeObjectEntry {
    pub(super) fn new(unformatted_entry: &str) -> Self {
        Self {
            value: unformatted_entry.into(),
        }
    }
}

impl FromStr for TreeObjectEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl TryFrom<Vec<u8>> for TreeObjectEntry {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl super::GitObject for TreeObjectEntry {
    fn formatted_value(&self) -> String {
        todo!()
    }

    fn unformatted_value(&self) -> String {
        todo!()
    }

    fn value_as_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn is_valid_object(string_to_check: &str) -> bool {
        todo!()
    }
}
