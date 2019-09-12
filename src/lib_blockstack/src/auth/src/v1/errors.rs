#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Authorization header should start with 'v1:'
    VersionMismatch,
    /// Tokens should have 3 components
    MalFormattedToken,
    HeaderEncodingCorrupted,
    HeaderDataCorrupted,
    PayloadEncodingCorrupted,
    PayloadDataCorrupted,
    PrincipalMissing,
    SignatureEncodingCorrupted,
    SecretKeyCorrupted,
    PublicKeyCorrupted,
    KeyDerivationFailed,
}
