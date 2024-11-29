mod hash_object;
mod tree_object;
mod tree_object_entry;

use flate2::read::{ZlibDecoder, ZlibEncoder};
pub use hash_object::HashObject;
use std::error::Error;
use std::io::Read;

pub type BoxedError = Box<dyn std::error::Error>;

/// GitObject is an object used in the git file system, it holds a value and provides
/// different forms of that immutable value.
///
/// To use/create a GitObjects one needs to create it via one of two ways:
///
/// - Converting from a slice of u8 characters, used when reading an existing value from a file
///     and converting it to the usable GitObject struct
/// - Converting from a str/string, Used when creating a new GitObject to be saved.
pub(crate) trait GitObject {
    /// provides the formatted value for the object using the raw data inserted
    ///
    /// Used when encoding and/or when printing the formatted value before saving, since the value variable
    /// is intended to keep the UNFORMATTED data.
    ///
    /// #### NOTE:
    /// this is not the data that should be saved, it is purely for display, when saving the
    /// formatted value, use value_as_bytes()
    ///
    /// ---
    /// For example, for the hash_object, it takes the value stored in the struct (i.e. "foo")
    /// and returns the formated value for its type:
    /// ```text
    /// blob 3\0foo
    /// ```
    /// This uses the stored unformatted value and the size() function.
    fn formatted_value(&self) -> String;
    /// value_as_bytes() does the same as value(), however, it returns the formatted value in a
    /// byte vector.
    ///
    /// Used when encrypting to hash.
    fn unformatted_value(&self) -> String;
    fn value_as_bytes(&self) -> Vec<u8>;
    fn size(&self) -> usize;
    /// Checks if the string provided fulfills the conditions to be a formatted value.
    /// It returns true if the string equals the formatted value (see value())
    ///
    /// It is used when decoding values to ensure they can be parsed to make a new Object
    ///
    /// NOTE: The value might not be equal due to decoding and bytes so regex is also a valid way of checking
    fn is_valid_object(string_to_check: &str) -> bool;
    /// encode takes the formatted value as bytes and returns a Zlib encoded Vec<u8>
    /// It is to be used when saving the object to a file
    fn encode(&self) -> Vec<u8> {
        let formatted_value = self.value_as_bytes();
        let mut encoder =
            ZlibEncoder::new(formatted_value.as_slice(), flate2::Compression::default());
        let mut buffer = vec![];
        encoder.read_to_end(&mut buffer).unwrap();
        buffer.to_vec()
    }
    /// decode(encoded_value: Vec<u8>) receives a vector of bytes and decodes it if they are encoded
    /// in Zlib.
    ///
    /// Used when reading values from files in a TryForm<Vec<u8>>
    ///
    /// ### Errors
    /// This functions returns an error if it can decode the value, or if the decoded value is not the desired object
    fn decode(encoded_value: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let mut decoder = ZlibDecoder::new(&encoded_value[..]);
        let mut decompressed: String = String::new();
        match decoder.read_to_string(&mut decompressed) {
            Ok(_) => {
                if Self::is_valid_object(&decompressed) {
                    Ok(decompressed)
                } else {
                    Err(format!(
                        "Decoded Value is not a valid Hash Object:\n. {}",
                        decompressed
                    )
                    .to_string()
                    .into())
                }
            }
            Err(e) => {
                eprintln!("Error Decoding from Vec<u8> to HashObject: {e}");
                Err(e.into())
            }
        }
    }
}
