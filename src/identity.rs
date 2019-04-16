use exonum::crypto::{Hash, PublicKey, hash};
use crate::common_structs;


// 160, 176, 192 and 208
const A_NUMBER: u8 = 160;
const B_NUMBER: u8 = 176;
const C_NUMBER: u8 = 192;
const D_NUMBER: u8 = 208;

pub enum IdType {
    Account(PublicKey),
    Token { owner_id: Vec<u8>, symbol: String }
}

fn gen_id(source: &IdType) -> Vec<u8> {
    let prepared = match source {
        IdType::Account(public_key) => (public_key.as_ref().to_vec(), A_NUMBER),
        IdType::Token { owner_id, symbol} =>
            ([owner_id, symbol.as_bytes()].concat(), B_NUMBER)
    };

    let mut first_bytes = hash(&prepared.0).as_ref().to_vec();
    first_bytes.truncate(15);
    let mut address: Vec<u8> = vec![prepared.1];
    address.extend(first_bytes);
    address
}

#[cfg(test)]
mod tests {
    use exonum::crypto::{SEED_LENGTH, Seed, gen_keypair_from_seed};
    use super::*;
    use hex::encode;

    #[test]
    fn gen_id_account() {
        let (public_key, secret_key) =
            gen_keypair_from_seed(&Seed::new([1; SEED_LENGTH]));
        let id_type = IdType::Account(public_key);
        let id = gen_id(&id_type);
        assert_eq!(hex::encode(id), "a034750f98bd59fcfc946da45aaabe93")
    }

    #[test]
    fn gen_token_account() {
        let account_id = vec![1; 16];
        let id_type = IdType::Token {
            owner_id: account_id,
            symbol: String::from("USD")
        };
        let id = gen_id(&id_type);
        assert_eq!(hex::encode(id), "b02abae7735206dd7e0901f2f967eea8")
    }
}
