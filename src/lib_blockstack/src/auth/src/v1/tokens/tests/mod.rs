use crate::v1::tokens::CreateAppKeypair;

#[test]
fn should_succeed_generating_app_key_pair_when_inputs_are_valid() {
    // Using mnemonic:
    // sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media
    let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
    // Expected address for m/888'/0': "1JeTQ5cQjsD57YGcsVFhwT7iuQUXJR6BSk";
    // Expected salt: "c15619adafe7e75a195a1a2b5788ca42e585a3fd181ae2ff009c6089de54ed9e";

    let app_domain = "https://amazing.app:443";
    let expected_app_address = "1A9NEhnXq5jDp9BRT4DrwadRP5jbBK896X";
    let expected_app_pk = "0329b5e65a1d392b795c310a830a527a0e1a928dd1ba5895d6b4d311c0f6dcf8ae";

    let bip39_seed = hex::decode(&bip39_seed).unwrap();
    let mut command = CreateAppKeypair::new(bip39_seed, app_domain.to_string());
    let (_, pk, address) = command.run().unwrap();

    assert!(expected_app_address == address);
    assert!(expected_app_pk == pk);
}

#[test]
fn should_succeed() {
    // Using mnemonic:
    // sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media
    let bip39_seed = "04afee363e2382656264c317d7440056e7ca1af65021152f9d927ee1cd057c56846c2afe54bb1c2c2de533a233ebb4025d5067caf0b48b04f3068d6e795db154";
    let app_domain = "https://amazing.app:443";

    let expected_app_address = "1A9NEhnXq5jDp9BRT4DrwadRP5jbBK896X";
    let expected_app_pk = "0329b5e65a1d392b795c310a830a527a0e1a928dd1ba5895d6b4d311c0f6dcf8ae";

    let mut command = CreateAppKeypair::new(bip39_seed.to_string(), app_domain.to_string());
    let (_, pk, address) = command.run().unwrap();

    assert!(expected_app_address == address);
    assert!(expected_app_pk == pk);
}
