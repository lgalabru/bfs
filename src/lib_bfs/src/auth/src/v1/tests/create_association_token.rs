use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};

use crate::v1::{
    errors::Error,
    create_association_token::CreateAssociationToken
};

#[test]
fn should_succeed_computing_keypair_given_secret_key() {
    let user_sk = "Kxk3GdPwATaCoNt4NJcueMupPu8njKsa3TWYiAKKZ3w4TXysLpT1".to_string();
    let expected_user_pk = "0332953eb4e423f933486b44925b836c0e663667b76864f0e0bfed46ff3d8cf84d".to_string();

    let app_pk = "02acc15622984fbd9205e078c241a93b1e93f4a0a1c3cf62142ac2671d8df73003".to_string();

    let mut command = CreateAssociationToken::new(user_sk, app_pk);
    let (user_pk, _) = command.get_user_key_pair().unwrap();
    
    assert!(expected_user_pk == user_pk);
}

#[test]
fn should_succeed_decoding_well_formed_secret_keys() {
    let user_secret_key = "Kxk3GdPwATaCoNt4NJcueMupPu8njKsa3TWYiAKKZ3w4TXysLpT1".to_string();
    let app_public_key = "02acc15622984fbd9205e078c241a93b1e93f4a0a1c3cf62142ac2671d8df73003".to_string();

    let mut command = CreateAssociationToken::new(user_secret_key, app_public_key);
    command.run().unwrap();
}
