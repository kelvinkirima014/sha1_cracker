use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read}
};
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
    }

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH{
        return  Err("sha1 ain't valid".into());
    }

    let f = File::open(&args[1])?;
    let mut read_file = BufReader::new(&f);
    let mut file_contents = String::new();
    read_file.read_to_string(&mut file_contents)?;

    for line in read_file.lines(){
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())){
            println!("Password Found: {}", common_password);
            return Ok(());
        }
    } println!("Password not found in wordlist :(");

    Ok(())
}
