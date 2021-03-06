use crate::v1::{errors::Error, tokens::VerifyAuthorizationToken};

#[test]
fn should_fail_when_prefix_other_than_v1() {
    let token = "v2:x".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    assert!(auth.run().unwrap_err() == Error::VersionMismatch);
}

#[test]
fn should_fail_when_doe_not_include_3_parts() {
    let token = "v1:header.payload".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    assert!(auth.run().unwrap_err() == Error::MalFormattedToken);
}

#[test]
fn should_fail_when_can_not_b64_decode_header() {
    let token = "v1:@header.payload.sig".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    println!("{:?}", auth.run().unwrap_err());
    assert!(auth.run().unwrap_err() == Error::HeaderEncodingCorrupted);
}

#[test]
fn should_fail_when_can_not_deserialize_header() {
    let token = "v1:eyJhbGciOiJIUzI1NiJx.@.sig".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    println!("{:?}", auth.run().unwrap_err());
    assert!(auth.run().unwrap_err() == Error::HeaderDataCorrupted);
}

#[test]
fn should_fail_when_can_not_b64_decode_payload() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.@.sig".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    println!("{:?}", auth.run().unwrap_err());
    assert!(auth.run().unwrap_err() == Error::PayloadEncodingCorrupted);
}

#[test]
fn should_fail_when_can_not_deserialize_payload() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOxIxIn0.sig".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    println!("{:?}", auth.run().unwrap_err());
    assert!(auth.run().unwrap_err() == Error::PayloadDataCorrupted);
}

#[test]
fn should_fail_when_iss_missing() {
    let token = "v1:eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiIxIn0.sig".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    println!("{:?}", auth.run().unwrap_err());
    assert!(auth.run().unwrap_err() == Error::PrincipalMissing);
}

#[test]
fn should_succeed_with_a_valid_token() {
    let token = "v1:eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NksifQ.eyJnYWlhQ2hhbGxlbmdlIjoiW1wiZ2FpYWh1YlwiLFwiMFwiLFwic3RvcmFnZTIuYmxvY2tzdGFjay5vcmdcIixcImJsb2Nrc3RhY2tfc3RvcmFnZV9wbGVhc2Vfc2lnblwiXSIsImh1YlVybCI6Imh0dHBzOi8vaHViLmJsb2Nrc3RhY2sub3JnIiwiaXNzIjoiMDNkOTg5YzA5YzNhZjhlYjcxYzBiM2I3NTQ1ODc4MmYzYjQzZDZkOTk2MGY5N2Y0ZmIyNjY3ZDczNjhiZTJhZjk0Iiwic2FsdCI6eyJ0eXBlIjoiQnVmZmVyIiwiZGF0YSI6WzExOCwxMjgsMTAyLDMsMTIsMTMzLDc2LDQyLDE2LDIwMSwxMDQsNjEsMTgyLDE4OCwxNiw2N119fQ.LCeFnVh7vi9zUowMgULpLGXY1acpswH4EWv5qhtOV7kWJxnSNJVkQf8tDRPN_Yhxt1sRONDwJwtCImU6TvlZYQ".to_string();
    let mut auth = VerifyAuthorizationToken::new(token);
    assert!(auth.run().unwrap() == ());
}
