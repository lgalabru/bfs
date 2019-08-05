use crate::v1;

#[test]
fn it_should_fail_if_prefix_other_than_v1() {
    let token = "v2:x".to_string();
    let mut auth = v1::Authentication::new(token);
    assert!(auth.validate().unwrap_err() == v1::Error::VersionMismatch);
}

#[test]
fn it_should_fail_if_doe_not_include_3_parts() {
    let token = "v1:header.payload".to_string();
    let mut auth = v1::Authentication::new(token);
    assert!(auth.validate().unwrap_err() == v1::Error::MalFormattedToken);
}

#[test]
fn it_should_fail_if_can_not_b64_decode_header() {
    let token = "v1:@header.payload.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::HeaderEncodingCorrupted);
}

#[test]
fn it_should_fail_if_can_not_deserialize_header() {
    let token = "v1:eyJhbGciOiJIUzI1NiJx.@.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::HeaderDataCorrupted);
}

#[test]
fn it_should_fail_if_can_not_b64_decode_payload() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.@.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::PayloadEncodingCorrupted);
}

#[test]
fn it_should_fail_if_can_not_deserialize_payload() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOxIxIn0.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::PayloadDataCorrupted);
}

#[test]
fn it_should_fail_if_iss_missing() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiIxIn0.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::PrincipalMissing);
}

#[test]
fn it_should_fail_() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIwMjUwODYzYWQ2NGE4N2FlOGEyZmU4M2MxYWYxYTg0MDNjYjUzZjUzZTQ4NmQ4NTExZGFkOGEwNDg4N2U1YjIzNTIifQ.sig".to_string();
    let mut auth = v1::Authentication::new(token);
    println!("{:?}", auth.validate().unwrap_err());
    assert!(auth.validate().unwrap_err() == v1::Error::PrincipalMissing);
}


    // let token = "v1:@eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.ErO0fS1yKjr73zJmeYqazVauy8z4Xwuhebs9fXVr3u4".to_string();

// #[test]
// fn it_should_fail_if_can_not_b64_decode_payload() {
//     let token = "v1:x.y.z".to_string();
//     let mut auth = v1::Authentication::new(token);
//     auth.validate().unwrap_err();
// }