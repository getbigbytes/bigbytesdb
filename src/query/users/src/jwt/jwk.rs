// Copyright 2024 Digitrans Inc
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use base64::engine::general_purpose;
use base64::prelude::*;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use jwt_simple::prelude::ES256PublicKey;
use jwt_simple::prelude::RS256PublicKey;
use log::info;
use log::warn;
use p256::EncodedPoint;
use p256::FieldBytes;
use parking_lot::RwLock;
use serde::Deserialize;
use serde::Serialize;

use super::PubKey;

const JWKS_REFRESH_TIMEOUT: u64 = 10;
const JWKS_REFRESH_INTERVAL: u64 = 600;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwkKey {
    pub kid: String,
    pub kty: String,
    pub alg: Option<String>,

    /// (Modulus) Parameter for kty `RSA`.
    #[serde(default)]
    pub n: String,
    /// (Exponent) Parameter for kty `RSA`.
    #[serde(default)]
    pub e: String,

    /// (X Coordinate) Parameter for kty `EC`
    #[serde(default)]
    pub x: String,
    /// (Y Coordinate) Parameter for kty `EC`
    #[serde(default)]
    pub y: String,
}

fn decode(v: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(v.as_bytes())
        .map_err(|e| ErrorCode::InvalidConfig(e.to_string()))
}

impl JwkKey {
    fn get_public_key(&self) -> Result<PubKey> {
        match self.kty.as_str() {
            // Todo(youngsofun): the "alg" field is optional, maybe we need a config for it
            "RSA" => {
                let k = RS256PublicKey::from_components(&decode(&self.n)?, &decode(&self.e)?)?;
                Ok(PubKey::RSA256(Box::new(k)))
            }
            "EC" => {
                // borrowed from https://github.com/RustCrypto/traits/blob/master/elliptic-curve/src/jwk.rs#L68
                let xs = decode(&self.x)?;
                let x = FieldBytes::from_slice(&xs);
                let ys = decode(&self.y)?;
                let y = FieldBytes::from_slice(&ys);
                let ep = EncodedPoint::from_affine_coordinates(x, y, false);

                let k = ES256PublicKey::from_bytes(ep.as_bytes())?;
                Ok(PubKey::ES256(k))
            }
            _ => Err(ErrorCode::InvalidConfig(format!(
                " current not support jwk with typ={:?}",
                self.kty
            ))),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct JwkKeys {
    pub keys: Vec<JwkKey>,
}

pub struct JwkKeyStore {
    pub(crate) url: String,
    cached_keys: Arc<RwLock<HashMap<String, PubKey>>>,
    pub(crate) last_refreshed_at: RwLock<Option<Instant>>,
    pub(crate) refresh_interval: Duration,
    pub(crate) refresh_timeout: Duration,
    pub(crate) load_keys_func: Option<Arc<dyn Fn() -> HashMap<String, PubKey> + Send + Sync>>,
}

impl JwkKeyStore {
    pub fn new(url: String) -> Self {
        Self {
            url,
            cached_keys: Arc::new(RwLock::new(HashMap::new())),
            refresh_interval: Duration::from_secs(JWKS_REFRESH_INTERVAL),
            refresh_timeout: Duration::from_secs(JWKS_REFRESH_TIMEOUT),
            last_refreshed_at: RwLock::new(None),
            load_keys_func: None,
        }
    }

    // only for test to mock the keys
    pub fn with_load_keys_func(
        mut self,
        func: Arc<dyn Fn() -> HashMap<String, PubKey> + Send + Sync>,
    ) -> Self {
        self.load_keys_func = Some(func);
        self
    }

    pub fn with_refresh_interval(mut self, interval: u64) -> Self {
        self.refresh_interval = Duration::from_secs(interval);
        self
    }

    pub fn with_refresh_timeout(mut self, timeout: u64) -> Self {
        self.refresh_timeout = Duration::from_secs(timeout);
        self
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}

impl JwkKeyStore {
    #[async_backtrace::framed]
    async fn load_keys(&self) -> Result<HashMap<String, PubKey>> {
        if let Some(load_keys_func) = &self.load_keys_func {
            return Ok(load_keys_func());
        }

        let client = reqwest::Client::builder()
            .timeout(self.refresh_timeout)
            .build()
            .map_err(|e| {
                ErrorCode::InvalidConfig(format!("Failed to create jwks client: {}", e))
            })?;
        let response = client.get(&self.url).send().await.map_err(|e| {
            ErrorCode::AuthenticateFailure(format!("Could not download JWKS: {}", e))
        })?;
        let jwk_keys: JwkKeys = response
            .json()
            .await
            .map_err(|e| ErrorCode::InvalidConfig(format!("Failed to parse JWKS: {}", e)))?;
        let mut new_keys: HashMap<String, PubKey> = HashMap::new();
        for k in &jwk_keys.keys {
            new_keys.insert(k.kid.to_string(), k.get_public_key()?);
        }
        Ok(new_keys)
    }

    #[async_backtrace::framed]
    async fn load_keys_with_cache(&self, force: bool) -> Result<HashMap<String, PubKey>> {
        let need_reload = force
            || match *self.last_refreshed_at.read() {
                None => true,
                Some(last_refreshed_at) => last_refreshed_at.elapsed() > self.refresh_interval,
            };

        let old_keys = self.cached_keys.read().clone();
        if !need_reload {
            return Ok(old_keys);
        }

        // if got network issues on loading JWKS, fallback to the cached keys if available
        let new_keys = match self.load_keys().await {
            Ok(new_keys) => new_keys,
            Err(err) => {
                warn!("Failed to load JWKS: {}", err);
                if !old_keys.is_empty() {
                    return Ok(old_keys);
                }
                return Err(err.add_message("failed to load JWKS keys, and no available fallback"));
            }
        };

        // the JWKS keys are not always changes, but when it changed, we can have a log about this.
        if !new_keys.keys().eq(old_keys.keys()) {
            info!("JWKS keys changed.");
        }
        *self.cached_keys.write() = new_keys.clone();
        self.last_refreshed_at.write().replace(Instant::now());
        Ok(new_keys)
    }

    #[async_backtrace::framed]
    pub async fn get_key(&self, key_id: Option<String>) -> Result<PubKey> {
        let keys = self.load_keys_with_cache(false).await?;

        // if the key_id is not set, and there is only one key in the store, return it
        let key_id = match key_id {
            Some(key_id) => key_id,
            None => {
                if keys.len() != 1 {
                    return Err(ErrorCode::AuthenticateFailure(
                        "must specify key_id for jwt when multi keys exists ",
                    ));
                } else {
                    return Ok((keys.iter().next().unwrap().1).clone());
                }
            }
        };

        match keys.get(&key_id) {
            None => Err(ErrorCode::AuthenticateFailure(format!(
                "key id {} not found in jwk store",
                key_id
            ))),
            Some(key) => Ok(key.clone()),
        }
    }
}
