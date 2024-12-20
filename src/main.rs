mod objects;
use objects::{GitObject, HashObject, TreeObject};

use sha1::{Digest, Sha1};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

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
                    print!("{}", hash_object.formatted_value())
                }
                Err(e) => {
                    panic!(
                        "Error decoding file: {}\nError:{}",
                        path.to_string_lossy(),
                        e
                    );
                }
            }
        }
        "hash-object" => {
            let command: (bool, String) = {
                if &args[2] == "-w" {
                    (true, args[3].clone())
                } else {
                    (false, args[2].clone())
                }
            };
            match HashObject::from_str(
                fs::read_to_string(PathBuf::from(command.1))
                    .unwrap()
                    .as_str(),
            ) {
                Ok(hash_object) => {
                    let hash = Sha1::digest(hash_object.formatted_value_as_bytes());
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
        "ls-tree" => {
            let hash = &args[3];
            let path = Path::new(".git/objects")
                .join(hash[..2].chars().as_str())
                .join(hash[2..].chars().as_str());
            match TreeObject::try_from(fs::read(&path).unwrap()) {
                Ok(tree_object) => {
                    for name in tree_object.name_only() {
                        println!("{}", name);
                    }
                }
                Err(e) => {
                    panic!(
                        "Error decoding file: {}\nError:{}",
                        path.to_string_lossy(),
                        e
                    );
                }
            }
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}
