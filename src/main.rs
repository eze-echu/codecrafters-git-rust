use flate2::read::*;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::path::Path;

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
            let mut decoder = ZlibDecoder::new(&zlib_compressed[..]);
            let mut uncompressed_string: String = String::new();
            decoder
                .read_to_string(&mut uncompressed_string)
                .unwrap_or_else(|e| panic!("Failed to decode ZLib file:\n {e}"));
            print!("{}", uncompressed_string);
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}
