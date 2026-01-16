// #![allow(unused)]
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use kaspa_addresses::{Address, Prefix, Version};
use hex;
//
pub fn extf() {
    // println!("##### wRpc call #####");
    //
}
pub fn work(scmap: HashMap<String, [u64; 2]>) {
    //
    let dbfile = crfile("setsm1.txt").expect("Err create");
    for (mut key, value) in scmap {
        if value[1] < 500000000 && value[0] >= 1000000000000 {
            let mut vers = Version::PubKey;
            if key.len() == 66 { vers = Version::PubKeyECDSA; }
            if key.len() == 68 { 
                vers = Version::ScriptHash;
                key = key[4..].to_string(); }
            let pbyt = hex::decode(key).unwrap();
            let pubk: Box<[u8]> = pbyt.into_boxed_slice();
            let addr = Address::new(Prefix::Mainnet, vers, &pubk).to_string();
            let am = value[0] / 100000000;
            let stw = format!("{}, {}, daa:{}\n", &addr, &am, &value[1]);
            wrfile(&dbfile, &stw).expect("Err write");
        }
    }
    //
}

fn crfile(name: &str) -> std::io::Result<File> {
    let file = File::create(name)?;
    Ok(file)
}
fn wrfile(mut file: &File, st: &str) -> std::io::Result<()> {
    file.write_all(st.as_bytes())?;
    Ok(())
}
