use crate::v1::jwt;

pub struct CreateAuthorizationToken {
    /// App private key: specific for every Blockstack ID and application combination
    app_private_key: String,
    /// 
    transit_public_key: String,
}

// v1:eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NksifQ.eyJnYWlhQ2hhbGxlbmdlIjoiW1wiZ2FpYWh1YlwiLFwiMFwiLFwic3RvcmFnZTIuYmxvY2tzdGFjay5vcmdcIixcImJsb2Nrc3RhY2tfc3RvcmFnZV9wbGVhc2Vfc2lnblwiXSIsImh1YlVybCI6Imh0dHBzOi8vaHViLmJsb2Nrc3RhY2sub3JnIiwiaXNzIjoiMDJkY2RiMzk2OTBhMThjNzYxZDVjYmYwZTdmODM2YmVmNTFmNWU1ZjUyZWE3ZDFjZjhiNWYzMzFjNjFmMjc4ZTI2Iiwic2FsdCI6eyJ0eXBlIjoiQnVmZmVyIiwiZGF0YSI6WzQ2LDI1MCw0NCwyMDcsNTIsNDcsMTk0LDExLDE0NywyMjksMTAsOTIsMTYsMTU1LDY2LDEwNl19fQ.Tfv_xrGbkeGocucSAyVbtG8lexBHjceGnKD6FQcPwBXtdHeDfHf7mKCUMSTAoa0adxU1EPKDdPoHfGVCvS2LOw

// export function makeAssociationToken(appPrivateKey: string, identityKey: string) : string {
//   const appPublicKey = getPublicKeyFromPrivateKey(`${canonicalPrivateKey(appPrivateKey)}01`)
//   const FOUR_MONTH_SECONDS = 60 * 60 * 24 * 31 * 4
//   const salt = crypto.randomBytes(16).toString('hex')
//   const identityPublicKey = getPublicKeyFromPrivateKey(identityKey)
//   const associationTokenClaim = {
//     childToAssociate: appPublicKey,
//     iss: identityPublicKey,
//     exp: FOUR_MONTH_SECONDS + ((new Date().getTime())/1000),
//     salt 
//   }
//   const associationToken = new jsontokens.TokenSigner('ES256K', identityKey)
//     .sign(associationTokenClaim)
//   return associationToken
// }

impl CreateAuthorizationToken {

    pub fn run() -> String {
        // Create header
        let header = jwt::Header::new();
        // json encoding
        // base64 encoding

        // Create payload / claims
        // let payload = tokens::authorization::Payload::new();
        // jti = uuidv4
        // exp = 60 * 60 * 24 * 31 * 4 + date.now
        // iat = date.now
        // iss = did from address (from public key)

        // json encoding
        // base64 encoding

        // Encode base64
        // Concatenate "header.payload"
        // Sign
        // Append Sig "header.payload.sig"
    }
}



// function makeGaiaAssociationToken(secretKeyHex: string, childPublicKeyHex: string) {
//   const LIFETIME_SECONDS = 365 * 24 * 3600
//   const signerKeyHex = secretKeyHex.slice(0, 64)
//   const compressedPublicKeyHex = getPublicKeyFromPrivate(signerKeyHex)
//   const salt = randomBytes(16).toString('hex')
//   const payload = { childToAssociate: childPublicKeyHex,
//                     iss: compressedPublicKeyHex,
//                     exp: LIFETIME_SECONDS + (new Date()/1000),
//                     iat: Date.now()/1000,
//                     salt }

//   const token = new TokenSigner('ES256K', signerKeyHex).sign(payload)
//   return token
// }





/**
 * Generates an authentication request that can be sent to the Blockstack
 * browser for the user to approve sign in. This authentication request can
 * then be used for sign in by passing it to the `redirectToSignInWithAuthRequest`
 * method.
 *
 * *Note: This method should only be used if you want to roll your own authentication
 * flow. Typically you'd use `redirectToSignIn` which takes care of this
 * under the hood.*
 *
 * @param  {String} transitPrivateKey - hex encoded transit private key
 * @param {String} redirectURI - location to redirect user to after sign in approval
 * @param {String} manifestURI - location of this app's manifest file
 * @param {Array<String>} scopes - the permissions this app is requesting
 * @param {String} appDomain - the origin of this app
 * @param {Number} expiresAt - the time at which this request is no longer valid
 * @param {Object} extraParams - Any extra parameters you'd like to pass to the authenticator.
 * Use this to pass options that aren't part of the Blockstack auth spec, but might be supported
 * by special authenticators.
 * @return {String} the authentication request
 */
// function makeAuthRequest(transitPrivateKey, redirectURI, manifestURI, scopes = authConstants_1.DEFAULT_SCOPE, appDomain, expiresAt = utils_1.nextMonth().getTime(), extraParams = {}) {
//     if (!transitPrivateKey) {
//         transitPrivateKey = new userSession_1.UserSession().generateAndStoreTransitKey();
//     }
//     const getWindowOrigin = (paramName) => {
//         const origin = typeof window !== 'undefined' && window.location && window.location.origin;
//         if (!origin) {
//             const errMsg = `\`makeAuthRequest\` called without the \`${paramName}\` param specified but`
//                 + ' the default value uses `window.location.origin` which is not available in this environment';
//             logger_1.Logger.error(errMsg);
//             throw new Error(errMsg);
//         }
//         return origin;
//     };
//     if (!redirectURI) {
//         redirectURI = `${getWindowOrigin('redirectURI')}/`;
//     }
//     if (!manifestURI) {
//         manifestURI = `${getWindowOrigin('manifestURI')}/manifest.json`;
//     }
//     if (!appDomain) {
//         appDomain = getWindowOrigin('appDomain');
//     }
//     /* Create the payload */
//     const payload = Object.assign({}, extraParams, {
//         jti: utils_1.makeUUID4(),
//         iat: Math.floor(new Date().getTime() / 1000),
//         exp: Math.floor(expiresAt / 1000),
//         iss: null,
//         public_keys: [],
//         domain_name: appDomain,
//         manifest_uri: manifestURI,
//         redirect_uri: redirectURI,
//         version: VERSION,
//         do_not_include_profile: true,
//         supports_hub_url: true,
//         scopes
//     });
//     logger_1.Logger.info(`blockstack.js: generating v${VERSION} auth request`);
//     /* Convert the private key to a public key to an issuer */
//     const publicKey = jsontokens_1.SECP256K1Client.derivePublicKey(transitPrivateKey);
//     payload.public_keys = [publicKey];
//     const address = keys_1.publicKeyToAddress(publicKey);
//     payload.iss = dids_1.makeDIDFromAddress(address);
//     /* Sign and return the token */
//     const tokenSigner = new jsontokens_1.TokenSigner('ES256k', transitPrivateKey);
//     const token = tokenSigner.sign(payload);
//     return token;
// }
// exports.makeAuthRequest = makeAuthRequest;
/**
 * Encrypts the private key for decryption by the given
 * public key.
 * @param  {String} publicKey  [description]
 * @param  {String} privateKey [description]
 * @return {String} hex encoded ciphertext
 * @private
 */
// function encryptPrivateKey(publicKey, privateKey) {
//     const encryptedObj = ec_1.encryptECIES(publicKey, privateKey);
//     const encryptedJSON = JSON.stringify(encryptedObj);
//     return (Buffer.from(encryptedJSON)).toString('hex');
// }
// exports.encryptPrivateKey = encryptPrivateKey;
/**
 * Decrypts the hex encrypted private key
 * @param  {String} privateKey  the private key corresponding to the public
 * key for which the ciphertext was encrypted
 * @param  {String} hexedEncrypted the ciphertext
 * @return {String}  the decrypted private key
 * @throws {Error} if unable to decrypt
 *
 * @private
 */
// function decryptPrivateKey(privateKey, hexedEncrypted) {
//     const unhexedString = Buffer.from(hexedEncrypted, 'hex').toString();
//     const encryptedObj = JSON.parse(unhexedString);
//     const decrypted = ec_1.decryptECIES(privateKey, encryptedObj);
//     if (typeof decrypted !== 'string') {
//         throw new Error('Unable to correctly decrypt private key');
//     }
//     else {
//         return decrypted;
//     }
// }
// exports.decryptPrivateKey = decryptPrivateKey;
/**
 * Generates a signed authentication response token for an app. This
 * token is sent back to apps which use contents to access the
 * resources and data requested by the app.
 *
 * @param  {String} privateKey the identity key of the Blockstack ID generating
 * the authentication response
 * @param  {Object} profile the profile object for the Blockstack ID
 * @param  {String} username the username of the Blockstack ID if any, otherwise `null`
 * @param  {AuthMetadata} metadata an object containing metadata sent as part of the authentication
 * response including `email` if requested and available and a URL to the profile
 * @param  {String} coreToken core session token when responding to a legacy auth request
 * or `null` for current direct to gaia authentication requests
 * @param  {String} appPrivateKey the application private key. This private key is
 * unique and specific for every Blockstack ID and application combination.
 * @param  {Number} expiresAt an integer in the same format as
 * `new Date().getTime()`, milliseconds since the Unix epoch
 * @param {String} transitPublicKey the public key provide by the app
 * in its authentication request with which secrets will be encrypted
 * @param {String} hubUrl URL to the write path of the user's Gaia hub
 * @param {String} blockstackAPIUrl URL to the API endpoint to use
 * @param {String} associationToken JWT that binds the app key to the identity key
 * @return {String} signed and encoded authentication response token
 * @private
 */
// function makeAuthResponse(privateKey, profile = {}, username = null, metadata, coreToken = null, appPrivateKey = null, expiresAt = utils_1.nextMonth().getTime(), transitPublicKey = null, hubUrl = null, blockstackAPIUrl = null, associationToken = null) {
//     /* Convert the private key to a public key to an issuer */
//     const publicKey = jsontokens_1.SECP256K1Client.derivePublicKey(privateKey);
//     const address = keys_1.publicKeyToAddress(publicKey);
//     /* See if we should encrypt with the transit key */
//     let privateKeyPayload = appPrivateKey;
//     let coreTokenPayload = coreToken;
//     let additionalProperties = {};
//     if (appPrivateKey !== undefined && appPrivateKey !== null) {
//         logger_1.Logger.info(`blockstack.js: generating v${VERSION} auth response`);
//         if (transitPublicKey !== undefined && transitPublicKey !== null) {
//             privateKeyPayload = encryptPrivateKey(transitPublicKey, appPrivateKey);
//             if (coreToken !== undefined && coreToken !== null) {
//                 coreTokenPayload = encryptPrivateKey(transitPublicKey, coreToken);
//             }
//         }
//         additionalProperties = {
//             email: metadata.email ? metadata.email : null,
//             profile_url: metadata.profileUrl ? metadata.profileUrl : null,
//             hubUrl,
//             blockstackAPIUrl,
//             associationToken,
//             version: VERSION
//         };
//     }
//     else {
//         logger_1.Logger.info('blockstack.js: generating legacy auth response');
//     }
//     /* Create the payload */
//     const payload = Object.assign({}, {
//         jti: utils_1.makeUUID4(),
//         iat: Math.floor(new Date().getTime() / 1000),
//         exp: Math.floor(expiresAt / 1000),
//         iss: dids_1.makeDIDFromAddress(address),
//         private_key: privateKeyPayload,
//         public_keys: [publicKey],
//         profile,
//         username,
//         core_token: coreTokenPayload
//     }, additionalProperties);
//     /* Sign and return the token */
//     const tokenSigner = new jsontokens_1.TokenSigner('ES256k', privateKey);
//     return tokenSigner.sign(payload);
// }
// exports.makeAuthResponse = makeAuthResponse;
//# sourceMappingURL=authMessages.js.map