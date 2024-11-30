use crate::objects::tree_object_entry::TreeObjectEntry;
use crate::objects::{BoxedError, GitObject, NULL_BYTE, SPACE_BYTE};
use sha1::{Digest, Sha1};
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
    fn split_bytes_from_treefile_into_entities(file_data: Vec<u8>) -> Vec<String> {
        let split_data = file_data
            .split(|&byte| byte == NULL_BYTE || byte == SPACE_BYTE)
            .map(ToOwned::to_owned)
            .collect::<Vec<Vec<u8>>>();

        let mut buffer_for_mode_and_name: Vec<Vec<u8>> = Vec::new();
        buffer_for_mode_and_name.push(split_data[2].clone());

        for (index, data_chunk) in split_data.iter().enumerate().skip(3).step_by(2) {
            buffer_for_mode_and_name.push(data_chunk.clone());
            let sha = Sha1::digest(data_chunk).to_vec();
            if let Some(next_item) = split_data.get(index + 1) {
                let remainder = next_item.split_at(sha.len()).1.to_vec();
                buffer_for_mode_and_name.push(remainder);
            }
        }
        // for i in (3..split_data.len()).step_by(2) {
        //     let value = &split_data[i];
        //     buffer_for_mode_and_name.push(value.clone());
        //
        //     let sha = Sha1::digest(value).to_vec();
        //
        //     if let Some(next_item) = split_data.get(i + 1) {
        //         if next_item.starts_with(sha.as_slice()) {
        //             let remainder = next_item.split_at(sha.len()).1.to_vec();
        //             buffer_for_mode_and_name.push(remainder);
        //         }
        //     }
        // }
        buffer_for_mode_and_name.pop();
        let mut buffer_to_group_mode_and_name = vec![];
        for (i, entry) in buffer_for_mode_and_name.iter().enumerate() {
            if i % 2 == 0 {
                let mut temp = entry.to_vec();
                temp.push(b' ');
                temp.append(&mut buffer_for_mode_and_name[i + 1].to_vec());
                buffer_to_group_mode_and_name.push(temp);
            }
        }
        buffer_to_group_mode_and_name
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
                let re = regex::bytes::Regex::new("^tree [0-9]+\x00").unwrap();
                if re.is_match(text_value.as_slice()) {
                    //let split = re.split(&text_value);
                    //let mut separated_entities = Self::get_only_entities_from_str(&text_value);
                    Ok(Self::new_from_file(
                        Self::split_bytes_from_treefile_into_entities(text_value), //split
                                                                                   //    .map(|e| String::from_utf8(Vec::from(e)).unwrap())
                                                                                   //    .collect::<Vec<String>>(),
                    ))
                } else {
                    Err(format!("File is not a valid TreeObject:\n{:x?}", text_value).into())
                }
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
