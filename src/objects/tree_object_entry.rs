use super::GitObject;
use sha1::{Digest, Sha1};
#[cfg(test)]
use std::fs;
use std::sync::Arc;
#[cfg(test)]
use tempfile::TempDir;

/// TreeObjectEntry
///
///
/// ### Formatted value
/// ``` <mode> <name>\0<sha1>
#[derive(Clone, Ord, Eq, PartialOrd, PartialEq)]
pub(super) struct TreeObjectEntry {
    entry_name: Arc<Vec<u8>>,
    entry_mode: u32,
}
impl TreeObjectEntry {
    pub(super) fn new_from_file(unformatted_entry: &str) -> Self {
        let entry = unformatted_entry.split(" ").collect::<Vec<&str>>();
        Self {
            entry_name: Arc::new(Vec::from(entry[1])),
            entry_mode: entry[0]
                .parse()
                .expect("Unable to parse mode from string when creating TreeObjectEntry"),
        }
    }
    fn hash_value(&self) -> Vec<u8> {
        Sha1::digest(self.entry_name.to_vec()).to_vec()
    }
}

impl GitObject for TreeObjectEntry {
    fn formatted_value(&self) -> String {
        let mut formatted = String::new();
        formatted.push_str(&format!("{} ", self.entry_mode));
        formatted.push_str(&format!(
            "{}\x00",
            String::from_utf8(self.entry_name.to_vec()).unwrap()
        ));
        let hash = Sha1::digest(self.entry_name.to_vec());
        let hash_hex = format!("{:x}", hash);
        formatted.push_str(&hash_hex.to_string());
        formatted
    }

    fn unformatted_value(&self) -> String {
        String::from_utf8(self.entry_name.to_vec()).expect("Error displaying value")
    }

    fn formatted_value_as_bytes(&self) -> Vec<u8> {
        let mut formatted: Vec<u8> = vec![];
        formatted.push(self.entry_mode.to_be_bytes()[0]);
        formatted.append(&mut b"\x00".to_vec());
        formatted.append(&mut self.entry_name.to_vec());
        let mut hash = Sha1::digest(self.entry_name.to_vec()).to_vec();
        formatted.append(&mut hash);
        formatted
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn is_valid_object(string_to_check: &str) -> bool {
        let re = regex::bytes::Regex::new("[0-9]+ .*\x00").unwrap();
        re.is_match(string_to_check.as_bytes())
    }
}
#[test]
fn test_unformatted_display() {
    let new = TreeObjectEntry::new_from_file("100644 foo.txt");
    assert_eq!(new.unformatted_value(), "foo.txt");
}
#[test]
fn test_formatted_display() {
    let new = TreeObjectEntry::new_from_file("100644 foo.txt");
    println!("{}", new.formatted_value());
    assert_eq!(
        new.formatted_value(),
        format!("100644 foo.txt\x00{:x}", Sha1::digest("foo.txt"))
    );
}
#[test]
fn test_creation() {
    let temp = TempDir::new().unwrap().into_path();
    // fs::create_dir(temp.join(".git")).unwrap();
    // fs::create_dir(temp.join(".git/objects")).unwrap();
    // fs::create_dir(temp.join(".git/refs")).unwrap();
    // fs::write(temp.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
    // println!("Initialized git directory");

    let new = TreeObjectEntry::new_from_file("100644 foo.txt");
    fs::write(temp.join("test"), new.encode()).unwrap();
    let checked = new.formatted_value_as_bytes();
    let read = fs::read(temp.join("test")).unwrap();
    println!("{:?}", temp.join("test").canonicalize());
    println!("{:x?}", read);
    assert_eq!(
        TreeObjectEntry::decode_file(read.clone()).unwrap()[read.len() - 29..],
        new.hash_value()
    );
    assert_eq!(TreeObjectEntry::decode_file(read).unwrap(), checked);
    //assert!(read.to_vec().(Sha1::digest(new.entry_name.to_vec()).as_slice()));
}
