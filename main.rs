/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

use aes_gcm::{
    aead::{Aead, AeadCore, Generate, Key, KeyInit},
    Aes256Gcm, Nonce, // Or `Aes128Gcm`
};
use std::fs;    // reading files
use std::env;   // cmdline args
use std::path::Path;    // for checking file extensions
use hex;



fn main() {

    // get user input from the cmd line
    let args: Vec<String> = env::args().collect();

    // checking for 2 cmdline args
    if args.len() < 3{
        // eprintln!() prints to sterr
        eprintln!("Error: Missing required arguments!");
        eprintln!("Usage: cryptkatool <word_to_encrypt> <output_file>");
        std::process::exit(1); // quits program
    }



    // sorting cmdline args
    let config = CmdLineArgs::get_cmdline_args(&args);
    let password: String = config.word_to_encrypt;
    let output_file: String = config.output_file;
    let title_output_file = &output_file;

    // checking if Ok or Err and printing Err
    let encryption_data = match Encrypt(&password){
        // ^^(pointing up) Encrypt(&password) runs and will either return Ok or Err data will be stored in this

        // vv(pointing down) if data matches Ok the Ok function stores the successful data in successful_output
        // the => then means do this or then so it will then do successful_output meaning return the value
        Ok(successful_output) => successful_output,

        // vv(pointing down) if data comes back as Err its saved in error_details and will run the eprint which is just stderr
        // printing and return (by return i mean print it, it cant return value to _check_data)
        // because std::process::exit(1) quits the program so all data destoryed
        Err(error_details) =>{
            eprintln!("Encryption failed: {:?}", error_details);
            std::process::exit(1); // quits program
        }
    };

    /*// DEBUG
    println!("password: {password}");
    println!("output file: {output_file}");
    */// DEBUG



    // seperating data
    let ciphertext = encryption_data.ciphertext;
    let key = encryption_data.key;
    let nonce = encryption_data.nonce;

    // getting hex value of seperated data
    let hex_ciphertext = hex::encode(&ciphertext);
    let hex_ciphertext = format!("{}\n",hex_ciphertext);
    let hex_key = hex::encode(&key);
    let hex_nonce = hex::encode(&nonce);



    // convert string into a path structure so rust can inspect extension
    let output_path = Path::new(title_output_file);

    // find users chosen file extension or default to .txt
    let extension = match output_path.extension() {
        Some(ext_os_str) => {
            // if an extension exists e.g txt csv etc it will convert it to a string slice
            ext_os_str.to_str().unwrap_or("txt")
        }
        None => {
            // default to txt file if no extension typed
            "txt"
        }
    };

    // extract file name without extension so it doesnt accidentally get "file.txt_secrets.txt"
    let file_stem = output_path
    .file_stem()
    .and_then(|s| s.to_str())
    .unwrap_or(title_output_file);

    // create the final file
    let secrets = format!("{}_secrets.{}", file_stem, extension);


    // creating hex_key + hex_nonce String
    let secret_data: String = format!("Key: {}\nNonce: {}\nOriginal Password: {}\nEncrypted Password: {}", hex_key, hex_nonce, password, hex_ciphertext);

    /*// DEBUG
    println!("Plain-text password: {password}\n");
    println!("Encrypted password: {:?}\n", ciphertext);
    println!("Key: {:?}\n", key);
    println!("Nonce: {:?}\n", nonce);

    println!("Hex ciphertext: {hex_ciphertext}");
    println!("Hex key: {hex_key}");
    println!("Hex nonce: {hex_nonce}");
    */// DEBUG

    // writing to output file
    println!("Printing encrypted password to: {output_file}");
    fs::write(&output_file, &hex_ciphertext).expect("Error!");

    // writing secrets to secrets file
    println!("Printing key and nonce to: {secrets}");
    fs::write(&secrets, &secret_data).expect("Error!");
    println!("");

    // printing encrytped password on screen
    print!("Encrypted password: {hex_ciphertext}");



}




// creating function to generate keys, cipher, nonce and encrypt password
// if you choose to return tuple you would do below:
// fn Encrypt(pass: String) -> Result<(Vec<u8>, Key<Aes256Gcm>, Nonce), aes_gcm::Error>{ // reurns a Result (meaning if no error this returns as a Vec<u8>) and if Error it returns as aes_gcm::Error value

fn Encrypt(pass: &String) -> Result<EncryptionData, aes_gcm::Error>{ // returns custom struct as Result (aka normal result) and aes_gcm::Error as error type
    // generating key, cipher and nonce
    let key = Key::<Aes256Gcm>::generate();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::generate(); // MUST be unique per message

    // creating encrypted password
    let ciphertext = cipher.encrypt(&nonce, pass.as_bytes())?; // these ? are returing the Error if needed
    // this makes ciphertext = our pass (password) converted into bytes NOTE as_bytes() makes it a &[u8] so pass
    // converted to bytes which are a u8 array and ciphertext is referencing it

    // decrypting password
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?; // these ? are returing the Error if needed
    // btw .as_ref() is same as & it means as reference so basically &ciphertext
    assert_eq!(plaintext, pass.as_bytes());
    // btw plaintext is bytes aswell not String etc its a Vec<u8> same as pass its also as Vec<u8> thats why it can compare

    // returning cipheertext, key and nonce variables as Ok Result
    Ok(EncryptionData {ciphertext, key, nonce})
    // can return as tuple if you want just amek sure to use right fn version above near fn
    // Ok((ciphertext, key, nonce))
}

// creating struct for encryption
struct EncryptionData{
    ciphertext: Vec<u8>,
    key: Key<Aes256Gcm>,
    nonce: aes_gcm::aead::Nonce<Aes256Gcm>,
}





// creating struct for holding cmdline args
struct CmdLineArgs{
    word_to_encrypt: String,
    output_file: String,
}

impl CmdLineArgs{
    fn get_cmdline_args(input: &Vec<String>) -> CmdLineArgs {
        let word_to_encrypt = input[1].clone(); // this is [1] because [0] is programs pwd
        let output_file = input[2].clone(); // same reason as above

        CmdLineArgs { word_to_encrypt, output_file }
    }

}
