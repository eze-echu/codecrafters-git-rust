use flate2::read::*;
use sha1::{Digest, Sha1};
#[allow(unused_imports)]
use std::env;
use std::error::Error;
use std::fmt::{format, Display, Formatter};
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory");
        }
        "cat-file" => {
            let hash = &args[3];
            let path = Path::new(".git/objects")
                .join(hash[..2].chars().as_str())
                .join(hash[2..].chars().as_str());
            let zlib_compressed = fs::read(&path).unwrap_or_else(|e| {
                panic!(
                    "Unable to read file: {}\nError: {e}",
                    path.to_string_lossy()
                )
            });
            match HashObject::try_from(zlib_compressed) {
                Ok(hash_object) => {
                    print!("{}", hash_object)
                }
                Err(e) => {
                    panic!(
                        "Error decoding file: {}\nError:{}",
                        path.to_string_lossy(),
                        e.to_string()
                    );
                }
            }
        }
        "hash-object" => {
            let command: (bool, String) = if &args[2] == "-w" {
                (true, args[3].clone())
            } else {
                (false, args[2].clone())
            };
            match HashObject::from_str(
                fs::read_to_string(PathBuf::from(command.1))
                    .unwrap()
                    .as_str(),
            ) {
                Ok(hash_object) => {
                    let hash = Sha1::digest(hash_object.value_as_byte());
                    let hash_hex = format!("{:x}", hash);
                    print!("{}", hash_hex);
                    if command.0 {
                        let folder_path = PathBuf::from(".git/objects").join(&hash_hex[..2]);
                        let file_path = folder_path.join(&hash_hex[2..]);
                        if !file_path.exists() {
                            fs::create_dir_all(folder_path).unwrap_or_else(|e| panic!("{}", e));
                        }
                        match fs::write(file_path, hash_object.encode()) {
                            Ok(_) => {}
                            Err(e) => {
                                print!("{}", hash_hex);
                                panic!("Error: {e}")
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!(
                        "Error encoding the following file: {} \nWith Text: {} \nError:{e}",
                        &args[2],
                        fs::read_to_string(PathBuf::from(&args[2])).unwrap()
                    )
                }
            }
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}
struct HashObject {
    value: Vec<u8>,
}
impl HashObject {
    fn new(text: &str) -> Self {
        Self { value: text.into() }
    }
    fn value(&self) -> String {
        format!(
            "blob {}\x00{}",
            self.size(),
            String::from_utf8(self.value.clone()).unwrap()
        )
    }
    fn value_as_byte(&self) -> Vec<u8> {
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
                    let value = decompressed.split("\x00").collect::<Vec<_>>()[1];
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
        Ok(HashObject::new(HashObject::decode(value)?.as_str()))
    }
}
