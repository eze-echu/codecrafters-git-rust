use crate::objects::tree_object_entry::TreeObjectEntry;
use crate::objects::{BoxedError, GitObject};
use std::ops::Add;
use std::sync::Arc;

pub struct TreeObject {
    entries: Arc<Vec<TreeObjectEntry>>,
}
impl TreeObject {
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
}

impl TryFrom<Vec<u8>> for TreeObject {
    type Error = BoxedError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Self::decode_file(value) {
            Ok(text_value) => {
                let re = regex::bytes::Regex::new("[0-9]+ .*\x00").unwrap();
                let split = re.split(&text_value);
                //let mut separated_entities = Self::get_only_entities_from_str(&text_value);
                println!("{:?}", split);
                //separated_entities.remove(0);
                Ok(Self::new_from_file(
                    split
                        .map(|e| String::from_utf8(Vec::from(e)).unwrap())
                        .collect::<Vec<String>>(),
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
