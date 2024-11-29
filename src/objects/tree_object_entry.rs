use super::GitObject;
use sha1::{Digest, Sha1};
#[cfg(test)]
use std::fs;
use std::sync::Arc;
#[cfg(test)]
use tempfile::TempDir;

#[derive(Clone)]
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

    fn value_as_bytes(&self) -> Vec<u8> {
        self.formatted_value().as_bytes().to_vec()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn is_valid_object(string_to_check: &str) -> bool {
        true
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
    let read = TreeObjectEntry::decode(fs::read(temp.join("test")).unwrap()).unwrap();
    println!("{}", read);
    assert_eq!(new.formatted_value(), read);
    assert_eq!(
        new.formatted_value(),
        format!("100644 foo.txt\x00{:x}", Sha1::digest("foo.txt"))
    )
}
