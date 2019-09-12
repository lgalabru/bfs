use serde::{Deserialize, Serialize};
use reqwest;
use std::collections::HashMap;

pub enum Error {
    Unknown,
}

pub fn get_identities(address: &str) -> Result<Vec<String>, Error> {
    let url = format!("https://core.blockstack.org/v1/addresses/bitcoin/{}", address);
    let payload: HashMap<String, Vec<String>> = reqwest::get(&url).unwrap().json().unwrap();
    // todo(ludo): handle these unwrap()
    let names = payload.get("names").unwrap();
    Ok(names.to_vec())
}

pub fn get_user(name: &str) -> Result<HashMap<String, User>, Error> {
    let url = format!("https://core.blockstack.org/v1/users/{}", name);
    let resp: HashMap<String, User> = reqwest::get(&url).unwrap().json().unwrap();
    // todo(ludo): handle these unwrap()
    let user = &*resp.get(name).unwrap();
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub owner_address: String,
    pub profile: Profile
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub apps: HashMap<String, String>,
    pub api: Api
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    #[serde(rename = "gaiaHubConfig")]
    pub gaia_hub_config: GaiaHubConfig,
    #[serde(rename = "gaiaHubUrl")]
    pub gaia_hub_url: String
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct GaiaHubConfig {
    pub url_prefix: String
} 

// {
//   "lgalabru.id.blockstack": {
//     "owner_address": "122nafhTmAPXqDhW8uVxqMnJfivf8fz3ee", 
//     "profile": {
//       "@context": "http://schema.org", 
//       "@type": "Person", 
//       "api": {
//         "gaiaHubConfig": {
//           "url_prefix": "https://gaia.blockstack.org/hub/"
//         }, 
//         "gaiaHubUrl": "https://hub.blockstack.org"
//       }, 
//       "apps": {
//         "http://localhost:3000": "https://gaia.blockstack.org/hub/1ADkdfTPW1hve21gfbgh83NYeGcu6wxy4B/", 
//         "https://animalkingdoms.netlify.com": "https://gaia.blockstack.org/hub/18SyJgAppQCELUBthnff44sYrjYmwv26P/", 
//         "https://aodh.xyz": "https://gaia.blockstack.org/hub/1LBu6riHyKnywgVL28MHNwU77VeKGN8chj/", 
//         "https://app.dappywallet.com": "https://gaia.blockstack.org/hub/1LzhG2zijbq3Ns66cAAsKmrdHkbLH2AHHA/", 
//         "https://app.dmail.online": "https://gaia.blockstack.org/hub/14zuyV8zw125HEAC1J55d5oTy9H8KQvRdA/", 
//         "https://bitcoin4photos.net": "https://gaia.blockstack.org/hub/145ykjd4sMrMCmsp3xQBXNgKANFa5q9Tmq/", 
//         "https://bitpatron.co": "https://gaia.blockstack.org/hub/1A58Fiz4iQFaBWc6U4ZDSa2Gm7B7mtii3L/", 
//         "https://blackhole.run": "https://gaia.blockstack.org/hub/12EgqorQrt4UVUqGn34ahmND3SQk2vRsv6/", 
//         "https://blockslack.io": "https://gaia.blockstack.org/hub/12VxjPrL5aN1TL5s1ZT1NYbHeE5y2BpZ82/", 
//         "https://cafe-society.news": "https://gaia.blockstack.org/hub/14JaARd3tJU7h1rABvvTisaw9aEXWLv3dK/", 
//         "https://cineflick.herokuapp.com": "https://gaia.blockstack.org/hub/1G8zj6pcHo2Lzz5QSFPGF8mkPyB4UGANG/", 
//         "https://properpass.top": "https://gaia.blockstack.org/hub/12nxouRjtxm5AstXbwHzhiFz9YpyZwzDn8/", 
//         "https://rtasks.app": "https://gaia.blockstack.org/hub/1Nv8tTNyvA1LWyH2ZUcmG4cCPmEEdvBsEq/", 
//         "https://wage.pm": "https://gaia.blockstack.org/hub/16MRjckdfThYwkgNbFrUgpEdiL718cS67T/"
//       }, 
//       "image": [
//         {
//           "@type": "ImageObject", 
//           "contentUrl": "https://gaia.blockstack.org/hub/122nafhTmAPXqDhW8uVxqMnJfivf8fz3ee//avatar-0", 
//           "name": "avatar"
//         }
//       ], 
//       "name": "Ludovic Galabru"
//     }, 
//     "public_key": "0245c5eff05fe6af78b0f797a3047bc5335d2b20f8acb9a1d57a1f1cc6d116bfa3", 
//     "verifications": [
//       "No verifications for non-id namespaces."
//     ], 
//     "zone_file": {
//       "$origin": "lgalabru.id.blockstack", 
//       "$ttl": 3600, 
//       "uri": [
//         {
//           "name": "_http._tcp", 
//           "priority": 10, 
//           "target": "https://gaia.blockstack.org/hub/122nafhTmAPXqDhW8uVxqMnJfivf8fz3ee/profile.json", 
//           "weight": 1
//         }
//       ]
//     }
//   }
// }