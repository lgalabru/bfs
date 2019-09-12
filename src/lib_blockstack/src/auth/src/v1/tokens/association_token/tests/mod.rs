use crate::v1::{errors::Error, tests::get_hardened_m_0, tokens::CreateAssociationToken};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

#[test]
fn should_succeed_decoding_well_formed_secret_keys() {
    let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
    let (user_sk, user_pk) = get_hardened_m_0(&bip39_seed).unwrap();

    let app_pk = "02acc15622984fbd9205e078c241a93b1e93f4a0a1c3cf62142ac2671d8df73003".to_string();

    let mut command = CreateAssociationToken::new(user_sk, user_pk, app_pk);
    command.run().unwrap();
    // todo(ludo): write expectations:
    // - Verifying signature should work
    // - The payload must include user public key
    // - The payload must incluse app public key
}
