use crate::objects::tree_object_entry::TreeObjectEntry;
use crate::objects::{BoxedError, GitObject, NULL_BYTE, SPACE_BYTE};
use sha1::{Digest, Sha1};
use std::ops::Add;
use std::sync::Arc;

pub struct TreeObject {
    entries: Arc<Vec<TreeObjectEntry>>,
}
impl TreeObject {
    const REGEX_VAL: &'static str = "([0-9]{5,6}) ([^\x00]+)\x00(?-u:[^\x00]{20})";
    pub fn new_from_file(unformatted_tree_entries: Vec<String>) -> Self {
        let mut entries: Vec<TreeObjectEntry> = vec![];
        for unformatted_tree_entry in unformatted_tree_entries {
            entries.push(TreeObjectEntry::new_from_file(
                unformatted_tree_entry.as_str(),
            ))
        }
        entries.sort();
        Self {
            entries: Arc::new(entries),
        }
    }
    fn get_only_entities_from_str(s: &str) -> Vec<String> {
        let mut result = s
            .split("\x00")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        result.remove(0);
        result
    }
    fn split_bytes_from_treefile_into_entities(file_data: Vec<u8>) -> Vec<String> {
        let re = regex::bytes::Regex::new(Self::REGEX_VAL).unwrap();
        //println!("{}", re.is_match(&file_data));
        let entities = re.captures_iter(file_data.as_slice());
        let mut result: Vec<Vec<u8>> = vec![];
        for captures in entities {
            //println!("{:?}", &captures[1]);
            //println!("{:?}", &captures[2]);
            //println!("{:?}", &captures[3].to_vec());
            let mut to_send = vec![];
            to_send.append(&mut captures[1].to_vec());
            to_send.push(b' ');
            to_send.append(&mut captures[2].to_vec());
            result.push(to_send);
        }
        result.sort();
        result
            .into_iter()
            .map(|e| String::from_utf8(e).unwrap())
            .collect()
    }
    pub fn name_only(&self) -> Vec<String> {
        let mut names = self
            .entries
            .iter()
            .map(|e| e.name_as_string())
            .collect::<Vec<String>>();
        names.sort();
        names
    }
}

impl TryFrom<Vec<u8>> for TreeObject {
    type Error = BoxedError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Self::decode_file(value) {
            Ok(text_value) => {
                if &text_value[..4] != b"tree" {
                    return Err("The file read is not a valid tree object".into());
                }
                let first_null_byte_pos = text_value.iter().position(|&b| b == NULL_BYTE).unwrap();
                let tree_size_bytes: &str =
                    std::str::from_utf8(text_value[5..].split_at(first_null_byte_pos).0).unwrap();
                let tree_size = tree_size_bytes.parse().unwrap();
                //let tree_size = usize::from(text_value[5..].iter().take_while(|&&charcter| value == NULL_BYTE).copied().collect());
                let mut results: Vec<&[u8]> = vec![];
                let separated_entities = &text_value[(text_value.len() - tree_size)..];
                let mut buf: Vec<u8> = vec![];
                let i: usize = 0;
                loop {
                    if i == tree_size {break;}
                }
                //let split = re.split(&text_value);
                //let mut separated_entities = Self::get_only_entities_from_str(&text_value);
                Ok(Self::new_from_file(
                    Self::split_bytes_from_treefile_into_entities(text_value), //split
                                                                               //    .map(|e| String::from_utf8(Vec::from(e)).unwrap())
                                                                               //    .collect::<Vec<String>>(),
                ))
            }
            Err(e) => Err(e),
        }
    }
}

impl GitObject for TreeObject {
    fn formatted_value(&self) -> String {
        let mut entries = String::new();
        for entry in self.entries.to_vec().as_slice() {
            entries = entries.add(&entry.formatted_value())
        }
        format!("tree {}\x00{}", self.size(), entries)
    }

    fn unformatted_value(&self) -> String {
        let mut result = String::new();
        for entry in self.entries.to_vec().as_slice() {
            result += &format!("{} \n", entry.unformatted_value()).to_string();
        }
        result
    }

    fn formatted_value_as_bytes(&self) -> Vec<u8> {
        let answer = self.formatted_value();
        answer.as_bytes().to_vec()
    }

    fn size(&self) -> usize {
        let mut size = 0;
        for formatted_entity in &self
            .entries
            .iter()
            .map(|e| e.formatted_value())
            .collect::<Vec<String>>()
        {
            size += formatted_entity.len();
        }
        size
    }

    fn is_valid_object(string_to_check: &str) -> bool {
        let re = regex::bytes::Regex::new("tree [0-9]+\x00.*$").unwrap();
        re.is_match(string_to_check.as_bytes())
    }
}
