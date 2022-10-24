use std::env::args;

use std::error::Error;

const SHA1_HEX_STRING_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH{
        return  Err("sha1 ain't valid".into());
    }
    Ok(())
}
