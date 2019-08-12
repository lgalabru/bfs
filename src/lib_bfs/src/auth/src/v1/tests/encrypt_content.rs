use crate::v1::{
    tests::get_hardened_m_0,
    errors::Error,
    encrypt_content::EncryptContent,
    decrypt_content::DecryptContent
};

#[test]
fn should_be_able_to_decrypt_an_encrypted_content() {
    // Using mnemonic:
    // sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media
    let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
    let (user_sk, user_pk) = get_hardened_m_0(&bip39_seed).unwrap();
    let encryption_key = hex::decode(&user_pk.to_string()).unwrap();

    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
    let command = EncryptContent::new(encryption_key, data.clone());
    let encrypted_data = command.run().unwrap();

    let command = DecryptContent::new(user_sk, encrypted_data);
    let decrypted_data = command.run().unwrap();

    assert!(data == decrypted_data);
}
