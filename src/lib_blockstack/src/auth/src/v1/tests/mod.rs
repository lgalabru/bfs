use hex;

use crate::v1::errors::Error;
use crate::v1::helpers::get_hardened_child_keypair;
use secp256k1::{Secp256k1, SecretKey};

pub fn get_hardened_m_0(bip39_seed: &str) -> Result<(Vec<u8>, String), Error> {
    let bytes = hex::decode(&bip39_seed).unwrap();
    get_hardened_child_keypair(&bytes, &[0])
}

// todo(ludo): Move + update this tests
// #[test]
// fn should_succeed_decoding_wif_to_public_key() {
//     // Using mnemonic:
//     // sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media
//     let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
//     let (user_sk, user_pk) = get_hardened_m_0(&bip39_seed);
//     let expected_user_pk = "0380341e5ab267eddc529cadd925abb083cd710317b5a19058bc700b7da0e0c573".to_string();
//     let app_pk = "02acc15622984fbd9205e078c241a93b1e93f4a0a1c3cf62142ac2671d8df73003".to_string();
//     let mut command = CreateAssociationToken::new(user_sk, user_pk, app_pk);
//     let (user_pk, _) = command.get_user_key_pair(&bip39_seed_bytes).unwrap();
//     assert!(expected_user_pk == user_pk);
// }

// #[test]
// fn should_succeed_decoding_wif_to_secret_key() {
//     let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
//     let (user_sk, user_pk) = get_hardened_m_0(&bip39_seed);
//     // L1efcRz84ZzLyU6HbC2P44vzeGj4VdeNQiqBb1mh6zkEJk26p57e
//     let expected_user_sk = "d5e9c5aba28dc46ca45757d57d46671f28a663c019f21e5c56b532c4e3982805".to_string();
//     let app_pk = "02acc15622984fbd9205e078c241a93b1e93f4a0a1c3cf62142ac2671d8df73003".to_string();
//     let mut command = CreateAssociationToken::new(user_sk, user_pk, app_pk);
//     let (_, user_sk) = command.get_user_key_pair(&bip39_seed_bytes).unwrap();
//     assert!(expected_user_sk == user_sk.to_string());
// }
