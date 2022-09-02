
//key 32bytes
let mut key = [0u8; 32];
let mut nonce = [0u8; 24];
OsRng.fill_bytes(&mut key);
OsRng.fill_bytes(&mut nonce);
fn main() {

}